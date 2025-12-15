package instructions

import (
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop/share"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"github.com/shirou/gopsutil/v4/net"
)

const InstructionFetchNetwork = "fetch_network"

type FetchNetworkRequest struct{}

var FetchNetworkHandler = InstructionHandler{
	Instruction: InstructionFetchNetwork,
	Action:      share.WrapResponseAction(FetchNetworkAction),
}

func FetchNetworkAction(request FetchNetworkRequest) []net.InterfaceStat {
	logger.Logger().Info("FetchNetworkAction called")
	netInterfaces, _ := net.Interfaces()

	return netInterfaces
}
