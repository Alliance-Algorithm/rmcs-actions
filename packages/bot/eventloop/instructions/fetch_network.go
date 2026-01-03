package instructions

import (
	"context"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop/share"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
	"github.com/shirou/gopsutil/v4/net"
)

const InstructionFetchNetwork = "fetch_network"

type NetworkInfo struct {
	Index        int      `json:"index"`
	Mtu          int      `json:"mtu"`
	Name         string   `json:"name"`
	HardwareAddr string   `json:"hardware_addr"`
	Flags        []string `json:"flags"`
	Addrs        []Addr   `json:"addrs"`
}

type Addr struct {
	Addr string `json:"addr"`
}

type FetchNetworkRequest struct{}

var FetchNetworkHandler = InstructionHandler{
	Instruction: InstructionFetchNetwork,
	Action:      share.WrapResponseAction(FetchNetworkAction),
}

func FetchNetworkAction(ctx context.Context, request FetchNetworkRequest) []NetworkInfo {
	logger.Logger().Info("FetchNetworkAction called")
	netInterfaces, _ := net.Interfaces()

	var networkInfos []NetworkInfo
	for i, iface := range netInterfaces {
		var addrs []Addr
		for _, addr := range iface.Addrs {
			addrs = append(addrs, Addr{Addr: addr.Addr})
		}

		networkInfos = append(networkInfos, NetworkInfo{
			Index:        i,
			Mtu:          iface.MTU,
			Name:         iface.Name,
			HardwareAddr: iface.HardwareAddr,
			Flags:        iface.Flags,
			Addrs:        addrs,
		})
	}

	return networkInfos
}
