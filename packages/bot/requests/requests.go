/*
 * Package requests provides utilities for making HTTP requests.
 *
 * This package is used to contact with the service provider's HTTP APIs,
 * including authentication, data retrieval, and error handling.
 */

package requests

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"net/url"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/config"
	"github.com/bytedance/sonic"
)

type Request interface {
	Endpoint() string
	Method() string
}

type TypedRequest[Req any, Resp any] interface {
	Request
	GetRequestBody() Req
	NewResponse() Resp
}

type BaseRequest[Req any, Resp any] struct {
	EndpointPath string
	HTTPMethod   string
	Body         Req
}

// Send executes the HTTP request and returns the typed response
// The base URL should be provided via context using RequestEndpointCtxKey
func (b *BaseRequest[Req, Resp]) Send(ctx context.Context) (Resp, error) {
	var response Resp

	// Get base URL from context
	cfg, ok := config.GetConfigFromCtx(ctx)
	if !ok || cfg.Service.Api == "" {
		return response, fmt.Errorf("base URL not found in context")
	}

	// Construct full URL
	fullURL := cfg.Service.Api + b.EndpointPath
	parsedURL, err := url.Parse(fullURL)
	if err != nil {
		return response, fmt.Errorf("failed to parse request url: %w", err)
	}

	// Marshal request body
	var bodyReader io.Reader
	if b.HTTPMethod != "GET" && b.HTTPMethod != "HEAD" {
		bodyBytes, err := sonic.Marshal(b.Body)
		if err != nil {
			return response, fmt.Errorf("failed to marshal request body: %w", err)
		}
		bodyReader = bytes.NewReader(bodyBytes)
	} else {
		query, err := encodeQueryParams(b.Body)
		if err != nil {
			return response, fmt.Errorf("failed to encode query params: %w", err)
		}
		if query != "" {
			parsedURL.RawQuery = query
		}
	}

	// Create HTTP request
	req, err := http.NewRequestWithContext(ctx, b.HTTPMethod, parsedURL.String(), bodyReader)
	if err != nil {
		return response, fmt.Errorf("failed to create request: %w", err)
	}

	// Set headers
	if b.HTTPMethod != "GET" && b.HTTPMethod != "HEAD" {
		req.Header.Set("Content-Type", "application/json")
	}
	req.Header.Set("Accept", "application/json")

	// Execute request
	client := &http.Client{}
	resp, err := client.Do(req)
	if err != nil {
		return response, fmt.Errorf("failed to execute request: %w", err)
	}
	defer resp.Body.Close()

	// Read response body
	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return response, fmt.Errorf("failed to read response body: %w", err)
	}

	// Check status code
	if resp.StatusCode < 200 || resp.StatusCode >= 300 {
		return response, fmt.Errorf("request failed with status %d: %s", resp.StatusCode, string(respBody))
	}

	// Unmarshal response
	if err := json.Unmarshal(respBody, &response); err != nil {
		return response, fmt.Errorf("failed to unmarshal response: %w", err)
	}

	return response, nil
}

// encodeQueryParams converts a request body object into URL query parameters for GET/HEAD requests.
func encodeQueryParams(body any) (string, error) {
	if body == nil {
		return "", nil
	}

	if values, ok := body.(url.Values); ok {
		return values.Encode(), nil
	}

	bodyBytes, err := sonic.Marshal(body)
	if err != nil {
		return "", err
	}

	var kv map[string]any
	if err := json.Unmarshal(bodyBytes, &kv); err != nil {
		return "", fmt.Errorf("expected object for query parameters: %w", err)
	}

	if len(kv) == 0 {
		return "", nil
	}

	values := url.Values{}
	for key, val := range kv {
		appendQueryValue(values, key, val)
	}

	return values.Encode(), nil
}

func appendQueryValue(values url.Values, key string, val any) {
	if val == nil {
		return
	}

	switch v := val.(type) {
	case []any:
		for _, item := range v {
			if item == nil {
				continue
			}
			values.Add(key, fmt.Sprint(item))
		}
	default:
		values.Add(key, fmt.Sprint(v))
	}
}
