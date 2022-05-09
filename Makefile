export NECROMANZER_VERSION_POSTFIX=
export RUST_BACKTRACE=1

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

clippy-pedantic:
	cargo clippy -- -D clippy::pedantic -A clippy::cast_precision_loss --verbose --no-deps

clean:
	cargo clean
