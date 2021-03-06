CRATE_NAME = noc-fact
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

stage: $(CRATE_NAME).out
	cp $> $(OUT_DIR)

$(CRATE_NAME).out: $*.o
	clang --target=cheri-unknown-freebsd --sysroot=$(CHERIBUILD)/output/sdk/sysroot128 -fuse-ld=$(LLD) -mcpu=cheri128 -mabi=purecap -fPIC -v $*.o -lc -o $@

$(CRATE_NAME).o: $*.ll
	llc -mtriple=cheri-unknown-freebsd -mcpu=cheri128 -mattr="+cheri128,+soft-float" -verify-machineinstrs -target-abi=purecap -relocation-model=pic -filetype=obj -o $@ $*.ll

$(CRATE_NAME).ll: libcore_cheri.rlib src/main.rs
	$(RUSTC) --extern libcore_cheri=$> --emit=llvm-ir --target cheri-unknown-freebsd -C panic=abort -C debug-assertions=off -o $@

libcore_cheri.rlib: $(LIBCORE_CHERI)
	$(RUSTC) $(LIBCORE_CHERI) --crate-type lib --emit=link=$@ -C panic=abort -C debug-assertions=off --target cheri-unknown-freebsd -C linker=$(CHERIBUILD)/output/sdk/bin/cheri-unknown-freebsd-clang -C link-arg=--sysroot=$(CHERIBUILD)/output/sdk/sysroot128 -C link-arg=-fPIC -C target-feature=+soft-float

cloc:
	rm -f *.ll *.z *.o *.out

clean: cloc
	rm -f *.rlib
