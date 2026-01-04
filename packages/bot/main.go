package main

import (
	"context"
	"flag"
	"fmt"
	"os"
	"os/signal"
	"time"

	"github.com/bytedance/sonic"
	"github.com/coder/websocket"
	"github.com/google/uuid"
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

	rootCtx, stop := signal.NotifyContext(context.Background(), os.Interrupt)
	defer stop()

	baseCtx := context.WithValue(rootCtx, config.ConfigCtxKey{}, *cfg)

	authRetryDelay := 2 * time.Second
	reconnectDelay := 2 * time.Second
	dialTimeout := 5 * time.Second

	for {
		var robotId uuid.UUID
		for {
			select {
			case <-rootCtx.Done():
				logger.Logger().Info("Shutdown requested during authentication")
				return
			default:
			}

			rid, err := lib.AuthenticateRobot(baseCtx)
			if err != nil {
				logger.Logger().Warn("Failed to authenticate robot, retrying", zap.Error(err), zap.Duration("retry_in", authRetryDelay))
				if !waitForRetry(rootCtx, authRetryDelay) {
					logger.Logger().Info("Shutdown requested during authentication pending state")
					return
				}
				continue
			}

			robotId = rid
			logger.Logger().Info("Robot authenticated successfully", zap.String("robot_id", robotId.String()))
			break
		}

		runCtx := context.WithValue(baseCtx, lib.RobotIdCtxKey{}, robotId)
		wsUrl := cfg.Service.Websocket + "/" + robotId.String()

		select {
		case <-rootCtx.Done():
			logger.Logger().Info("Shutdown requested before connection")
			return
		default:
		}

		dialCtx, cancelDial := context.WithTimeout(runCtx, dialTimeout)
		c, _, err := websocket.Dial(dialCtx, wsUrl, nil)
		cancelDial()
		if err != nil {
			logger.Logger().Warn("Failed to connect to websocket, restarting from authentication", zap.Error(err), zap.Duration("retry_in", reconnectDelay))
			if !waitForRetry(rootCtx, reconnectDelay) {
				logger.Logger().Info("Shutdown requested during reconnect pending state")
				return
			}
			continue
		}

		logger.Logger().Info("Connected to websocket", zap.String("url", wsUrl))

		connCtx, cancelConn := context.WithCancel(runCtx)
		connErr := make(chan error, 1)

		backend := eventloop.NewWsEventloop(c)

		rawSend := backend.SendJson
		backend.SendJson = func(ctx context.Context, v any) error {
			err := rawSend(ctx, v)
			if err != nil && ctx.Err() == nil {
				select {
				case connErr <- err:
				default:
				}
			}
			return err
		}

		rawRecv := backend.RecvJson
		backend.RecvJson = func(ctx context.Context, v *sonic.NoCopyRawMessage) error {
			err := rawRecv(ctx, v)
			if err != nil && ctx.Err() == nil {
				select {
				case connErr <- err:
				default:
				}
			}
			return err
		}

		eventloop.ServeOn(connCtx, backend)
		logger.Logger().Info("Event loop started")

		select {
		case <-rootCtx.Done():
			logger.Logger().Info("Shutdown signal received")
			cancelConn()
			c.Close(websocket.StatusNormalClosure, "Bot shutting down")
			return
		case err := <-connErr:
			logger.Logger().Warn("Connection closed, restarting from authentication", zap.Error(err))
		}

		cancelConn()
		c.Close(websocket.StatusNormalClosure, "Restarting")

		if !waitForRetry(rootCtx, reconnectDelay) {
			logger.Logger().Info("Shutdown requested during reconnect pending state")
			return
		}
	}
}

func waitForRetry(ctx context.Context, delay time.Duration) bool {
	timer := time.NewTimer(delay)
	defer timer.Stop()

	select {
	case <-ctx.Done():
		return false
	case <-timer.C:
		return true
	}
}
