CRATE_NAME = nostd-ctr
HOME = /home/nwss2
LIBCORE_CHERI = $(HOME)/libcore-cheri
CHERIBUILD = $(HOME)/cheri
OUT_DIR = $(CHERIBUILD)/output/rootfs128/rustprogs
LLVM_BUILD = $(HOME)/work/llvm-project/build
LLD = $(LLVM_BUILD)/bin/ld.lld
RUSTC = /home/nwss2/rust/build/x86_64-unknown-freebsd/stage1/bin/rustc

test: src/main.rs
	cargo build --release
	target/release/$(CRATE_NAME)

deploy:
	xargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/$(CRATE_NAME) $(OUT_DIR)

stage: $(CRATE_NAME).out
	cp $> $(OUT_DIR)

$(CRATE_NAME).out: $*.o $(LIBCORE_CHERI)/libcore_cheri.so
	clang --target=cheri-unknown-freebsd --sysroot=$(CHERIBUILD)/output/sdk/sysroot128 -fuse-ld=$(LLD) -mcpu=cheri128 -mabi=purecap -fPIC -fno-lto -fno-strict-aliasing -v $> -lc -o $@

$(CRATE_NAME).o: $*.ll
	llc -mtriple=cheri-unknown-freebsd -mcpu=cheri128 -mattr="+cheri128,+soft-float" -verify-machineinstrs -target-abi=purecap -relocation-model=pic -filetype=obj -o $@ $*.ll

$(CRATE_NAME).ll: $(LIBCORE_CHERI)/libcore_cheri.rlib src/main.rs
	$(RUSTC) --extern libcore_cheri=$> --emit=llvm-ir --target cheri-unknown-freebsd -C panic=abort -C debug-assertions=off -o $@

cloc:
	rm -f *.ll *.z *.o *.out

clean: cloc
	rm -f *.rlib
