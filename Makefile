# Load environment variables from .env if present
ifneq (,$(wildcard ./.env))
	include .env
	export
endif

# Default target: build + flash
all: build flash

build:
	cargo build --release

flash:
	@if [ -z "$(RAVEDUDE_PORT)" ]; then \
		echo "Error: RAVEDUDE_PORT is not set. Define it in .env or export it."; \
		exit 1; \
	fi
	RAVEDUDE_PORT=$(RAVEDUDE_PORT) cargo run --release

clean:
	cargo clean

.PHONY: all build flash clean
