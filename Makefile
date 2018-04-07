
all: build

run:
	cargo run

build: .install-deps
	cargo build

.install-deps:
	sudo apt-get install libgtk-3-dev libdbus-1-dev
	touch .install-deps

clean:
	sudo apt-get purge libgtk-3-dev
	rm .install-deps