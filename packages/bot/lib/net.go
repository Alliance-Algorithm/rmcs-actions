package lib

import (
	"errors"
	"slices"

	"github.com/shirou/gopsutil/v4/net"
)

func GetLocalMacAddress() (string, error) {
	stat, error := net.Interfaces()
	if error != nil {
		return "", error
	}
	for _, v := range stat {
		if !slices.Contains(v.Flags, "FlagLoopback") && v.HardwareAddr != "" {
			return v.HardwareAddr, nil
		}
	}
	return "", errors.New("failed to retrieve local MAC address")
}
