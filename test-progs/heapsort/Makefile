target test: src/main.rs
	cargo build --release
	target/release/heapsort

target deploy:
	xargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/heapsort ~/cheri/output/rootfs128
