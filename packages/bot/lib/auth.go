package lib

import (
	"context"
	"errors"
	"os"
	"os/user"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/config"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/requests/ident"
	"go.uber.org/zap"
)

func GetRobotIdFromLocalStorage(ctx context.Context) (string, error) {
	cfg, ok := config.GetConfigFromCtx(ctx)
	if !ok {
		return "", errors.New("no config is provided within context")
	}
	robotIdFile := cfg.Storage.Dir + "/robot_id"

	data, err := os.ReadFile(robotIdFile)
	if err != nil {
		return "", err
	}
	return string(data), nil
}

func SaveRobotIdToLocalStorage(ctx context.Context, robotId string) error {
	cfg, ok := config.GetConfigFromCtx(ctx)
	if !ok {
		return errors.New("no config is provided within context")
	}
	robotIdFile := cfg.Storage.Dir + "/robot_id"

	err := os.WriteFile(robotIdFile, []byte(robotId), 0755)
	if err != nil {
		return err
	}
	return nil
}

func AuthenticateRobot(ctx context.Context) (string, error) {
	robotId, err := GetRobotIdFromLocalStorage(ctx)
	// Robot ID not found, need to authenticate
	// Note that `error` does not necessarily mean file read error
	if err == nil {
		return robotId, nil
	}

	user, err := user.Current()
	if err != nil {
		return "", err
	}

	mac, err := GetLocalMacAddress()
	if err != nil {
		return "", err
	}

	logger.Logger().Info("Authenticating robot", zap.String("username", user.Username), zap.String("mac", mac))

	whoamiReq := ident.NewWhoAmIRequest(ident.WhoAmIRequestBody{
		UserName: user.Username,
		Mac:      mac,
	})
	resp, err := whoamiReq.Send(ctx)
	if err != nil {
		return "", err
	}

	err = SaveRobotIdToLocalStorage(ctx, resp.RobotId)
	if err != nil {
		return "", err
	}

	return resp.RobotId, nil
}
