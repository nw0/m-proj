CRATE_NAME = nostd-memset
HOME = /home/nwss2
LIBCORE_CHERI = $(HOME)/rust-xargo/src/libcore
LIBCORE_CHERI_RLIB = $(LIBCORE_CHERI)/libcore_cheri.rlib
LIBALLOC = $(HOME)/rust-xargo/src/liballoc
LIBALLOC_RLIB = $(LIBALLOC)/liballoc.rlib
COMPILER_BUILTINS = $(HOME)/deps/compiler-builtins
COMPILER_BUILTINS_RLIB = $(COMPILER_BUILTINS)/libcompiler_builtins.rlib
CHERIBUILD = $(HOME)/cheri
OUT_DIR = $(CHERIBUILD)/output/rootfs128/rustprogs
LLVM_BUILD = $(HOME)/work/llvm-project/build
LLD = $(LLVM_BUILD)/bin/ld.lld
RUSTC = /home/nwss2/rust/build/x86_64-unknown-freebsd/stage1/bin/rustc

test: src/main.rs
	xargo build --release
	target/release/$(CRATE_NAME)

deploy:
	xargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/$(CRATE_NAME) $(OUT_DIR)

stage: $(CRATE_NAME).out
	cp $> $(OUT_DIR)

$(CRATE_NAME).out: $*.o $(LIBCORE_CHERI)/libcore_cheri.so $(COMPILER_BUILTINS)/libcompiler_builtins.so $(LIBALLOC)/liballoc.so
	clang --target=cheri-unknown-freebsd --sysroot=$(CHERIBUILD)/output/sdk/sysroot128 -fuse-ld=$(LLD) -mcpu=cheri128 -mabi=purecap -fPIC -fno-lto -fno-strict-aliasing -v $> -lc -o $@

$(CRATE_NAME).o: $*.ll
	llc -mtriple=cheri-unknown-freebsd -mcpu=cheri128 -mattr="+cheri128,+soft-float" -verify-machineinstrs -target-abi=purecap -relocation-model=pic -filetype=obj -o $@ $*.ll

$(CRATE_NAME).ll: $(LIBCORE_CHERI_RLIB) src/main.rs
	$(RUSTC) src/main.rs --emit=llvm-ir --target cheri-unknown-freebsd -C panic=abort -C debug-assertions=off --extern core=$(LIBCORE_CHERI_RLIB) --extern compiler_builtins=$(COMPILER_BUILTINS_RLIB) --extern alloc=$(LIBALLOC_RLIB) -o $@

cloc:
	rm -f *.ll *.z *.o *.out
	cargo clean

clean: cloc
	rm -rf ~/.xargo/lib/rustlib/cheri-unknown-freebsd
	rm -f *.rlib
