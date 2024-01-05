build:
	RUST_BACKTRACE=full cargo build -Zfeatures=build_dep --target thumbv7em-none-eabihf

asm:
	RUSTFLAGS="--emit asm" cargo build --target thumbv7em-none-eabihf

asm-release:
	RUSTFLAGS="--emit asm" cargo build --target thumbv7em-none-eabihf --release

docker-build:
	docker-compose build

docker:
	docker-compose up 