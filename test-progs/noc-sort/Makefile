OUT_DIR = ~/cheri/output/rootfs128/rustprogs
CRATE_NAME = noc-sort

test: src/main.rs
	cargo build --release
	target/release/$(CRATE_NAME)

deploy:
	cargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/$(CRATE_NAME) $(OUT_DIR)
