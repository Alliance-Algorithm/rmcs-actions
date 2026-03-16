package instructions

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"net/url"
	"os"
	"path/filepath"
	"syscall"
	"time"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop/share"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"go.uber.org/zap"
)

const InstructionUpdateBinary = "update_binary"

// UpdateBinaryRequest is the request payload sent from the service.
type UpdateBinaryRequest struct {
	ArtifactUrl string `json:"artifact_url"`
}

// UpdateBinaryResponse is the response payload sent back to the service.
type UpdateBinaryResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

// UpdateBinaryHandler registers the update_binary instruction using the
// ResponseAction pattern.
var UpdateBinaryHandler = InstructionHandler{
	Instruction: InstructionUpdateBinary,
	Action:      share.WrapResponseAction(UpdateBinaryAction),
}

// elfMagic is the first 4 bytes of any valid ELF binary.
var elfMagic = []byte{0x7f, 'E', 'L', 'F'}

// UpdateBinaryAction downloads a new binary from the given artifact URL,
// validates it as an ELF executable, atomically replaces the current
// executable, and schedules a restart via syscall.Exec.
// sanitizeURL returns a host/path summary with query parameters stripped to
// avoid leaking presigned URL credentials into logs.
func sanitizeURL(raw string) string {
	u, err := url.Parse(raw)
	if err != nil {
		return "<invalid-url>"
	}
	return u.Host + u.Path
}

func UpdateBinaryAction(ctx context.Context, req UpdateBinaryRequest) UpdateBinaryResponse {
	logger.Logger().Info("UpdateBinaryAction called", zap.String("artifact_url", sanitizeURL(req.ArtifactUrl)))

	execPath, err := os.Executable()
	if err != nil {
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to get executable path: %v", err)}
	}
	execPath, err = filepath.EvalSymlinks(execPath)
	if err != nil {
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to resolve symlinks: %v", err)}
	}

	execDir := filepath.Dir(execPath)

	// Create temp file in the same directory to ensure same-filesystem for
	// atomic rename.
	tmpFile, err := os.CreateTemp(execDir, ".update_binary_*")
	if err != nil {
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to create temp file: %v", err)}
	}
	tmpPath := tmpFile.Name()

	// Cleanup helper — removes the temp file on any error path.
	cleanup := func() {
		tmpFile.Close()
		os.Remove(tmpPath)
	}

	// Download the binary.
	httpClient := &http.Client{Timeout: 30 * time.Second}
	httpReq, err := http.NewRequestWithContext(ctx, http.MethodGet, req.ArtifactUrl, nil)
	if err != nil {
		cleanup()
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to create request: %v", err)}
	}
	resp, err := httpClient.Do(httpReq)
	if err != nil {
		cleanup()
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to download binary: %v", err)}
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		cleanup()
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("download returned status %d", resp.StatusCode)}
	}

	_, err = io.Copy(tmpFile, resp.Body)
	if err != nil {
		cleanup()
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to write binary: %v", err)}
	}
	tmpFile.Close()

	// Validate ELF magic bytes.
	header := make([]byte, 4)
	f, err := os.Open(tmpPath)
	if err != nil {
		os.Remove(tmpPath)
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to open temp file for validation: %v", err)}
	}
	_, err = io.ReadFull(f, header)
	f.Close()
	if err != nil {
		os.Remove(tmpPath)
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to read header: %v", err)}
	}
	for i := 0; i < 4; i++ {
		if header[i] != elfMagic[i] {
			os.Remove(tmpPath)
			return UpdateBinaryResponse{Status: "error", Message: "downloaded file is not a valid ELF binary"}
		}
	}

	// Set executable permissions.
	if err := os.Chmod(tmpPath, 0755); err != nil {
		os.Remove(tmpPath)
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to chmod: %v", err)}
	}

	// Atomic replace via same-filesystem rename.
	if err := os.Rename(tmpPath, execPath); err != nil {
		os.Remove(tmpPath)
		return UpdateBinaryResponse{Status: "error", Message: fmt.Sprintf("failed to replace binary: %v", err)}
	}

	logger.Logger().Info("Binary replaced successfully, scheduling restart", zap.String("path", execPath))

	// Schedule restart after a short delay so the WebSocket response can flush.
	go func() {
		time.Sleep(1 * time.Second)
		logger.Logger().Info("Restarting via syscall.Exec", zap.String("path", execPath))
		if err := syscall.Exec(execPath, os.Args, os.Environ()); err != nil {
			logger.Logger().Error("Failed to exec new binary", zap.Error(err))
		}
	}()

	return UpdateBinaryResponse{Status: "post_update", Message: "success, restarting..."}
}
