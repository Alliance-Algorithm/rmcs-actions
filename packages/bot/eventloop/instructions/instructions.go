package instructions

import (
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/bytedance/sonic"
)

type InstructionMessage struct {
	Instruction string                 `json:"instruction"`
	Message     sonic.NoCopyRawMessage `json:"message"`
}

type InstructionHandler struct {
	Instruction string
	Action      lib.SessionAction
}

var instructionHandlers = []InstructionHandler{
	SyncRobotNameHandler,
	FetchNetworkHandler,
}

var InstructionHandlers = func() map[string]InstructionHandler {
	handlerMap := make(map[string]InstructionHandler)
	for _, handler := range instructionHandlers {
		handlerMap[handler.Instruction] = handler
	}
	return handlerMap
}()
