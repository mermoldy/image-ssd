ARM_TARGET=armv7-unknown-linux-musleabihf

default: build

build:
	PKG_CONFIG_ALLOW_CROSS=1 RUST_BACKTRACE=1 cargo build --release --target=$(ARM_TARGET) 
