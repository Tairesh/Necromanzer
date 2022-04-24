export NECROMANZER_VERSION_POSTFIX=

build:
	cargo build --release

run:
	cargo run --release

before-commit: fmt test clippy
check: fmt-check test clippy

fmt:
	cargo fmt --

fmt-check:
	cargo fmt -- --check

test:
	cargo test

clippy:
	cargo clippy -- -Dwarnings --verbose
