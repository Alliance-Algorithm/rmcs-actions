package config

import (
	"context"
	"os"

	"go.yaml.in/yaml/v4"
)

type Config struct {
	Log     LogConfig     `yaml:"log"`
	Storage StorageConfig `yaml:"storage"`
	Service ServiceConfig `yaml:"service"`
}

type LogConfig struct {
	Dir string `yaml:"dir"`
}

type StorageConfig struct {
	Dir string `yaml:"dir"`
}

type ServiceConfig struct {
	Api       string `yaml:"api"`
	Websocket string `yaml:"websocket"`
}

type ConfigCtxKey struct{}

func LoadConfig(configPath string) (*Config, error) {
	yamlFile, err := os.ReadFile(configPath)
	if err != nil {
		return nil, err
	}
	var config Config
	err = yaml.Unmarshal(yamlFile, &config)
	if err != nil {
		return nil, err
	}
	if err := config.validate(); err != nil {
		return nil, err
	}
	return &config, nil
}

func (c *Config) validate() error {
	if err := os.MkdirAll(c.Log.Dir, 0o755); err != nil {
		return err
	}
	if err := os.MkdirAll(c.Storage.Dir, 0o755); err != nil {
		return err
	}
	return nil
}

func GetConfigFromCtx(ctx context.Context) (Config, bool) {
	config, ok := ctx.Value(ConfigCtxKey{}).(Config)
	return config, ok
}
