target test: src/main.rs
	cargo build --release
	target/release/nostd 495

target deploy:
	xargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/nostd ~/cheri/output/rootfs128
