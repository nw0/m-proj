CRATE_NAME = noc
HOME = /home/nwss2
LIBCORE_CHERI = $(HOME)/libcore-cheri/lib.rs
CHERIBUILD = $(HOME)/cheri
OUT_DIR = $(CHERIBUILD)/output/rootfs128/rustprogs
LLVM_BUILD = $(HOME)/work/llvm-project/build
LLD = $(LLVM_BUILD)/bin/ld.lld
RUSTC = /home/nwss2/rust/build/x86_64-unknown-freebsd/stage1/bin/rustc

test: src/main.rs
	cargo build --release
	target/release/$(CRATE_NAME)

deploy:
	cargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/$(CRATE_NAME) $(OUT_DIR)

stage: main.out
	cp $> $(OUT_DIR)

main.out: $*.o
	clang --target=cheri-unknown-freebsd --sysroot=$(CHERIBUILD)/output/sdk/sysroot128 -fuse-ld=$(LLD) -mcpu=cheri128 -mabi=purecap -fPIC -v $*.o -lc -o $@

main.o: $*.ll
	llc -mtriple=cheri-unknown-freebsd -mcpu=cheri128 -mattr="+cheri128,+soft-float" -verify-machineinstrs -target-abi=purecap -relocation-model=pic -filetype=obj -o $@ $*.ll

main.ll: src/main.rs
	$(RUSTC) $> --emit=llvm-ir --target cheri-unknown-freebsd -C panic=abort -C debug-assertions=off

scpc: cpurecap.out
	cp $> $(OUT_DIR)

cpurecap.out: $*.o
	clang --target=cheri-unknown-freebsd --sysroot=$(CHERIBUILD)/output/sdk/sysroot128 -fuse-ld=$(LLD) -mcpu=cheri128 -mabi=purecap -fPIC -v $*.o -lc -o $@

cpurecap.o: $*.ll
	llc -mtriple=cheri-unknown-freebsd -mcpu=cheri128 -mattr="+cheri128,+soft-float" -verify-machineinstrs -target-abi=purecap -relocation-model=pic -filetype=obj -o $@ $*.ll

clean:
	rm -f main.ll *.out *.o *.z
