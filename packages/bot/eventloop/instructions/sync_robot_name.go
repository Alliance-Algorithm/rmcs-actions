package instructions

import (
	"context"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop/share"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"go.uber.org/zap"
)

type SyncRobotNameRequest struct {
	RobotName string `json:"robot_name"`
}

const InstructionSyncRobotName = "sync_robot_name"

var SyncRobotNameHandler = InstructionHandler{
	Instruction: InstructionSyncRobotName,
	Action:      share.WrapOneShotAction(SyncRobotNameAction),
}

func SyncRobotNameAction(ctx context.Context, request SyncRobotNameRequest) {
	// Currently a no-op
	logger.Logger().Info("SyncRobotNameAction called with Name: " + request.RobotName)
	err := lib.SetRobotName(ctx, request.RobotName)
	if err != nil {
		logger.Logger().Error("Failed to set robot name", zap.Error(err))
	}
}
