all:
	cargo build --release

install:
	cp ./target/release/duckdns-ip-updater $HOME/bin
	cp ./config/duckdns-ip-updater.conf $XDG_CONFIG_HOME/duckdns-ip-updater/default.conf

clean:
	rm -R target