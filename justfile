[working-directory: 'run']
run: build-kernel
	cargo run -p funcos

[working-directory: 'run']
debug: build-kernel
	cargo run -p funcos -- --debug

[working-directory: 'run']
test: build-kernel-tests
	cargo run -p funcos -- --test

[working-directory: 'os']
build-kernel:
	cargo build -p kernel --target targets/x86_64-funcos.json

[working-directory: 'os']
build-kernel-tests:
	cargo test -p kernel --target targets/x86_64-funcos.json --no-run
	# Rename the test binary to `kernel-tests.elf`.
	mv $(cargo test -p kernel --target targets/x86_64-funcos.json --no-run --message-format=json \
		| grep -o 'target/x86_64-funcos/debug/deps/kernel-[0-9a-f]*\.elf' \
		| head -n 1) target/x86_64-funcos/debug/kernel-tests.elf
