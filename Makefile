all: build convert write

build:
	cargo build --release

convert:
	cargo objcopy --bin naive_stuff --target thumbv7m-none-eabi --release -- -O binary target/naive_stuff.bin

write:
	st-flash write target/naive_stuff.bin 0x8000000

openocd:
	openocd -f interface/stlink.cfg -f target/stm32f1x.cfg
