package lib

import (
	"context"
	"errors"
	"os"
	"os/user"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/config"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/requests/ident"
	"github.com/bytedance/sonic"
	"github.com/google/uuid"
	"go.uber.org/zap"
)

type robotInfo struct {
	RobotId string    `json:"robot_id"`
	Mac     string    `json:"mac"`
	Name    string    `json:"name"`
	Uuid    uuid.UUID `json:"uuid"`
}

func getRobotIdFromLocalStorage(ctx context.Context) (*robotInfo, error) {
	cfg, ok := config.GetConfigFromCtx(ctx)
	if !ok {
		return nil, errors.New("no config is provided within context")
	}
	robotIdFile := cfg.Storage.Dir + "/robot_id"

	data, err := os.ReadFile(robotIdFile)
	if err != nil {
		return nil, err
	}

	var info *robotInfo
	err = sonic.Unmarshal(data, &info)
	if err != nil {
		return nil, err
	}

	return info, nil
}

func saveRobotIdToLocalStorage(ctx context.Context, info *robotInfo) error {
	cfg, ok := config.GetConfigFromCtx(ctx)
	if !ok {
		return errors.New("no config is provided within context")
	}
	robotIdFile := cfg.Storage.Dir + "/robot_id"

	data, err := sonic.Marshal(info)
	if err != nil {
		return err
	}
	err = os.WriteFile(robotIdFile, data, 0755)
	if err != nil {
		return err
	}
	return nil
}

func retrieveRemoteRobotId(ctx context.Context, username string, mac string) (ident.RetrieveResponse, error) {
	retrieveReq := ident.NewRetrieveRequest(ident.RetrieveRequestBody{
		Username:   username,
		MacAddress: mac,
	})
	resp, err := retrieveReq.Send(ctx)
	if err != nil || resp.RobotId == "" {
		whoamiReq := ident.NewWhoAmIRequest(ident.WhoAmIRequestBody{
			UserName: username,
			Mac:      mac,
		})
		whoami, err := whoamiReq.Send(ctx)
		if err != nil {
			return ident.RetrieveResponse{}, err
		}
		return ident.RetrieveResponse{
			RobotId: whoami.RobotId,
			Mac:     mac,
			Name:    whoami.RobotName,
			Uuid:    whoami.RobotUuid,
		}, nil
	}
	return resp, nil
}

func getRobotAuthInfo(ctx context.Context) (*robotInfo, error) {
	info, err := getRobotIdFromLocalStorage(ctx)
	// Robot ID not found, need to authenticate
	// Note that `error` does not necessarily mean file read error
	if err == nil {
		return info, nil
	}

	user, err := user.Current()
	if err != nil {
		return nil, err
	}

	mac, err := GetLocalMacAddress()
	if err != nil {
		return nil, err
	}

	logger.Logger().Info("Authenticating robot", zap.String("username", user.Username), zap.String("mac", mac))

	remoteInfo, err := retrieveRemoteRobotId(ctx, user.Username, mac)
	if err != nil {
		return nil, err
	}

	logger.Logger().Info("Robot authenticated", zap.String("robot_id", remoteInfo.RobotId))

	return &robotInfo{
		RobotId: remoteInfo.RobotId,
		Mac:     remoteInfo.Mac,
		Name:    remoteInfo.Name,
		Uuid:    remoteInfo.Uuid,
	}, nil
}

func AuthenticateRobot(ctx context.Context) (string, error) {
	info, err := getRobotAuthInfo(ctx)
	if err != nil {
		return "", err
	}

	err = saveRobotIdToLocalStorage(ctx, &robotInfo{
		RobotId: info.RobotId,
		Mac:     info.Mac,
		Name:    info.Name,
		Uuid:    info.Uuid,
	})
	if err != nil {
		return "", err
	}

	syncReq := ident.NewSyncRequest(ident.SyncRequestBody{
		RobotId: info.RobotId,
		Mac:     info.Mac,
		Name:    info.Name,
		Uuid:    info.Uuid,
	})
	_, err = syncReq.Send(ctx)
	if err != nil {
		return "", err
	}

	return info.RobotId, nil
}
