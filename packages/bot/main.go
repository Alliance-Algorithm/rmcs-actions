package main

import (
	"context"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"time"

	"github.com/coder/websocket"
	"go.uber.org/zap"

	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/config"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/eventloop"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/lib"
	"github.com/Alliance-Algorithm/rmcs-actions/packages/bot/logger"
)

func main() {
	// Parse command line arguments
	configPath := flag.String("config", "config.yaml", "Path to config file")
	flag.Parse()

	cfg, err := config.LoadConfig(*configPath)
	if err != nil {
		if err := fmt.Errorf("failed to load config: %v", err); err != nil {
			fmt.Println(err)
		}
		os.Exit(1)
	}

	logger.InitLogger(cfg.Log.Dir)
	defer logger.Logger().Sync()

	ctx := context.Background()
	ctx = context.WithValue(ctx, config.ConfigCtxKey{}, *cfg)

	robotId, err := lib.AuthenticateRobot(ctx)
	if err != nil {
		logger.Logger().Error("Failed to authenticate robot", zap.Error(err))
		os.Exit(1)
	}

	logger.Logger().Info("Robot authenticated successfully", zap.String("robot_id", robotId))

	wsUrl := cfg.Service.Websocket + "/" + robotId

	dialCtx, cancel := context.WithTimeout(ctx, time.Second)
	c, _, err := websocket.Dial(dialCtx, wsUrl, nil)
	cancel()
	if err != nil {
		logger.Logger().Error("Failed to connect to websocket", zap.Error(err))
		os.Exit(1)
	}
	defer c.CloseNow()

	logger.Logger().Info("Connected to websocket", zap.String("url", wsUrl))

	// Create a cancellable context for graceful shutdown
	eventLoopCtx, cancelEventLoop := context.WithCancel(ctx)
	defer cancelEventLoop()

	backend := lib.NewWsEventloop(c)
	go eventloop.ServeOn(eventLoopCtx, *backend)
	logger.Logger().Info("Event loop started")

	sigCh := make(chan os.Signal, 1)
	signal.Notify(sigCh, os.Interrupt)

	<-sigCh
	logger.Logger().Info("Shutdown signal received")

	logger.Logger().Info("Stopping event loop")
	cancelEventLoop()

	time.Sleep(100 * time.Millisecond)

	c.Close(websocket.StatusNormalClosure, "Bot shutting down")
	logger.Logger().Info("Websocket closed")

	time.Sleep(100 * time.Millisecond)
	logger.Logger().Info("Shutdown complete")
}
