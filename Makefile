database=ariaonline

.PHONY: publish client bindings server all

all: server client

server: bindings publish

client:
	cargo run -p client

client-release:
	cargo run -p client --release

bindings:
	spacetime generate --lang rust --out-dir ./client/src/bindings --project-path server

publish:
	spacetime publish -c -y -p server $(database)
