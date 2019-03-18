OUT_DIR = ~/cheri/output/rootfs128/rustprogs
CRATE_NAME = noc-hello

test: src/main.rs
	cargo build --release
	target/release/$(CRATE_NAME)

deploy:
	cargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/$(CRATE_NAME) $(OUT_DIR)

rs:
	/home/nwss2/work/rust/build/x86_64-unknown-freebsd/stage1/bin/rustc --crate-name rs src/main.rs --crate-type bin --emit=dep-info,link -C panic=abort -C debug-assertions=off --target cheri-unknown-freebsd -C linker=/home/nwss2/cheri/output/sdk/bin/cheri-unknown-freebsd-clang -C link-arg=--sysroot=/home/nwss2/cheri/output/sdk/sysroot128 -C target-feature=+soft-float -C link-arg=-fPIC
	cp rs $(OUT_DIR)

main.ll: src/main.rs
	$(RUSTC) src/main.rs --target cheri-unknown-freebsd --emit=llvm-ir

main.o: main.ll
	llc -mcpu=cheri128 -cheri-cap-table-abi=pcrel -mattr="+cheri128,+soft-float" -filetype obj -target-abi=purecap -relocation-model=pic -o main.o main.ll

main: main.o
	clang --target=cheri-unknown-freebsd --sysroot=/home/nwss2/cheri/output/sdk/sysroot128 -fuse-ld=/home/nwss2/work/build/bin/ld.lld -mcpu=cheri128 -mabi=purecap -fPIC -v main.o -lc -o main
	cp main $(OUT_DIR)

c.ll: c.c
	clang --target=cheri-unknown-freebsd --sysroot=/home/nwss2/cheri/output/sdk/sysroot128 -fuse-ld=/home/nwss2/work/build/bin/ld.lld -mcpu=cheri128 -mabi=purecap -cheri-cap-table-abi=pcrel -msoft-float -v -emit-llvm -S c.c

c.o: c.ll
	llc -mcpu=cheri128 -cheri-cap-table-abi=pcrel -mattr="+cheri128,+soft-float" -filetype obj -target-abi=purecap -relocation-model=pic -o c.o c.ll

c: c.o
	clang --target=cheri-unknown-freebsd --sysroot=/home/nwss2/cheri/output/sdk/sysroot128 -fuse-ld=/home/nwss2/work/build/bin/ld.lld -mcpu=cheri128 -mabi=purecap -fPIC -v c.o -lc -o c
	cp c $(OUT_DIR)

clean:
	rm -f *.o *.ll *.d c main rs
