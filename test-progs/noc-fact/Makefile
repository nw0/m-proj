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

stage: main.out
	cp main.out $(OUT_DIR)

main.out: main.o
	clang --target=cheri-unknown-freebsd --sysroot=$(CHERIBUILD)/output/sdk/sysroot128 -fuse-ld=$(LLD) -mcpu=cheri128 -mabi=purecap -fPIC -v main.o -lc -o main.out

main.o: main.ll
	llc -mcpu=cheri128 -cheri-cap-table-abi=pcrel -mattr="+cheri128,+soft-float" -filetype obj -target-abi=purecap -relocation-model=pic -o main.o main.ll

main.ll: libcore_cheri.rlib src/main.rs
	$(RUSTC) src/main.rs --emit=llvm-ir --target cheri-unknown-freebsd --extern libcore_cheri=libcore_cheri.rlib -C panic=abort -C debug-assertions=off

libcore_cheri.rlib: $(LIBCORE_CHERI)
	$(RUSTC) $(LIBCORE_CHERI) --crate-type lib --emit=link=libcore_cheri.rlib -C panic=abort -C debug-assertions=off --target cheri-unknown-freebsd -C linker=$(CHERIBUILD)/output/sdk/bin/cheri-unknown-freebsd-clang -C link-arg=--sysroot=$(CHERIBUILD)/output/sdk/sysroot128 -C link-arg=-fPIC -C target-feature=+soft-float -C panic=abort

clmain:
	rm main.ll main.o main

clean: clmain
	rm libcore_cheri.rlib
