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
	config, ok := config.GetConfigFromCtx(ctx)
	if !ok || config.Service.Api == "" {
		return response, fmt.Errorf("base URL not found in context")
	}

	// Construct full URL
	fullURL := config.Service.Api + b.EndpointPath

	// Marshal request body
	var bodyReader io.Reader
	if b.HTTPMethod != "GET" && b.HTTPMethod != "HEAD" {
		bodyBytes, err := sonic.Marshal(b.Body)
		if err != nil {
			return response, fmt.Errorf("failed to marshal request body: %w", err)
		}
		bodyReader = bytes.NewReader(bodyBytes)
	}

	// Create HTTP request
	req, err := http.NewRequestWithContext(ctx, b.HTTPMethod, fullURL, bodyReader)
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
