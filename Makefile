export NECROMANZER_VERSION_POSTFIX=
export RUST_BACKTRACE=1

build:
	cargo build --release

run:
	cargo run --release

before-commit: fmt check
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
	cargo clippy -- -D clippy::pedantic -A clippy::cast_precision_loss -A clippy::cast_possible_truncation -A clippy::cast_possible_wrap -A clippy::cast_sign_loss -A clippy::cast_lossless -A clippy::module_name_repetitions -A clippy::too_many_lines --verbose --no-deps

clean:
	cargo clean
