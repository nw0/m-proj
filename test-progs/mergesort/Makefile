target test: src/main.rs
	cargo build --release
	target/release/mergesort

target deploy:
	xargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/mergesort ~/cheri/output/rootfs128
