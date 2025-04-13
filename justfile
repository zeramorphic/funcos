[working-directory: 'run']
run: build-kernel
	cargo run -p funcos

[working-directory: 'run']
debug: build-kernel
	cargo run -p funcos -- --debug

[working-directory: 'os']
build-kernel:
	cargo build -p kernel --target targets/x86_64-funcos.json
