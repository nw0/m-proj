test: src/main.rs
	cargo build --release
	target/release/noc

deploy:
	cargo build --target cheri-unknown-freebsd --release
	cp target/cheri-unknown-freebsd/release/noc ~/cheri/output/rootfs128

main.o: main.ll
	llc -mcpu=cheri128 -cheri-cap-table-abi=pcrel -mattr="+cheri128,+soft-float" -filetype obj -target-abi=purecap -relocation-model=pic -o main.o main.ll

main: main.o
	clang --target=cheri-unknown-freebsd --sysroot=/home/nwss2/cheri/output/sdk/sysroot128 -fuse-ld=/home/nwss2/work/build/bin/ld.lld -mcpu=cheri128 -mabi=purecap -fPIC -v main.o -lc -o main
	cp main ~/cheri/output/rootfs128

cpurecap.o: cpurecap.ll
	llc -mcpu=cheri128 -cheri-cap-table-abi=pcrel -mattr="+cheri128,+soft-float" -filetype obj -target-abi=purecap -o cpurecap.o cpurecap.ll

cpurecap: cpurecap.o
	clang --target=cheri-unknown-freebsd --sysroot=/home/nwss2/cheri/output/sdk/sysroot128 -fuse-ld=/home/nwss2/work/build/bin/ld.lld -mcpu=cheri128 -mabi=purecap -v cpurecap.o -lc -o cpurecap
	cp cpurecap ~/cheri/output/rootfs128
