package instructions

import (
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop/share"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
)

type SyncRobotIdRequest struct {
	RobotId string `json:"robot_id"`
}

const InstructionSyncRobotId = "sync_robot_id"

var SyncRobotIdHandler = InstructionHandler{
	Instruction: InstructionSyncRobotId,
	Action:      share.WrapOneShotAction(SyncRobotIdAction),
}

func SyncRobotIdAction(request SyncRobotIdRequest) {
	// Currently a no-op
	logger.Logger().Info("SyncRobotIdAction called with ID: " + request.RobotId)
}
