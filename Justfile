database := "ariaonline"

all: server client

server: bindings publish

client:
	cargo run -p client

client-release:
	cargo run -p client --release

bindings:
	spacetime generate --lang rust --out-dir ./bindings/src/bindings --project-path server

publish:
	spacetime publish -c -y -p server {{database}}

bots *ARGS:
	cargo run -p bots -- {{ARGS}}
