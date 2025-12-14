package logger

import (
	"os"
	"sync"

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
	"gopkg.in/natefinch/lumberjack.v2"
)

var (
	l    *zap.Logger
	once sync.Once
)

func InitLogger(logPath string) error {
	var err error
	once.Do(func() {
		debugging := zap.LevelEnablerFunc(func(lvl zapcore.Level) bool {
			return lvl >= zapcore.ErrorLevel
		})
		errors := zap.LevelEnablerFunc(func(lvl zapcore.Level) bool {
			return lvl < zapcore.ErrorLevel
		})

		consoleDebugging := zapcore.Lock(os.Stdout)
		consoleErrors := zapcore.Lock(os.Stderr)

		consoleEncoder := zapcore.NewConsoleEncoder(zap.NewDevelopmentEncoderConfig())

		logRotator := &lumberjack.Logger{
			Filename:   logPath + "/bot.log",
			MaxSize:    10, // megabytes
			MaxBackups: 5,
			MaxAge:     30,
			Compress:   true,
		}
		logEncoder := zapcore.NewJSONEncoder(zap.NewProductionEncoderConfig())
		logRotatorWriter := zapcore.AddSync(logRotator)

		core := zapcore.NewTee(
			zapcore.NewCore(consoleEncoder, consoleDebugging, debugging),
			zapcore.NewCore(consoleEncoder, consoleErrors, errors),
			zapcore.NewCore(logEncoder, logRotatorWriter, errors),
		)
		l = zap.New(core)
	})
	return err
}

func Logger() *zap.Logger {
	return l
}
