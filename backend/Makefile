run:
	RUST_LOG=debug RUST_BACKTRACE=1 CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER='sudo -E -S' cargo watch -x 'run'

arm64:
	RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" cargo build --release --target aarch64-unknown-linux-gnu
