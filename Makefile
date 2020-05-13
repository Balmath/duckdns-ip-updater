BIN_DIR = $(HOME)/bin
CONFIG_DIR = $(HOME)/.config

ifdef XDG_CONFIG_HOME
	CONFIG_DIR = $XDG_CONFIG_HOME
endif

all:
	cargo build --release

update:
	mkdir -p $(BIN_DIR)
	cp ./target/release/duckdns-ip-updater $(BIN_DIR)/duckdns-ip-updater

install: update
	mkdir -p $(CONFIG_DIR)/duckdns-ip-updater
	cp ./config/duckdns-ip-updater.conf $(CONFIG_DIR)/duckdns-ip-updater/default.conf

clean:
	rm -R target
