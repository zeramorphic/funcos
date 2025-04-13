run: build-kernel
	cargo run -p funcos

build-funcos: build-kernel
	cargo build -p funcos

build-kernel:
	cargo build -p kernel --target targets/x86_64-funcos.json
