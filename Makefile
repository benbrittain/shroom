TOOLCHAIN = arm-none-eabi-

TARGET = config/thumbv7em-none-eabi
TRIPLE = arm-unknown-linux-gnueabi
OPT = 0

ARM_OPTS = -C opt-level=0 -Z no-landing-pads --target $(TARGET) -g

all: bin/shroom

bin/shroom: obj/runtime.o

obj/runtime.o: dirs lib/libcore.rlib
	rustc $(ARM_OPTS) --emit obj -L ./libcore --out-dir ./objs/ src/main.rs

lib/libcore.rlib: rust
	rustc $(ARM_OPTS) ./rust/src/libcore/lib.rs --out-dir libcore

rust:
	if [ ! -d "rust" ]; then git clone https://github.com/rust-lang/rust; fi

sel4:
	if [ ! -d "sel4" ]; then git clone https://github.com/seL4/seL4; fi

clean:
	rm -rf libcore objs bin target

# Make some directories
dirs:
	if [ ! -d "libcore" ]; then mkdir -p libcore; fi
	if [ ! -d "objs" ]; then mkdir -p objs; fi
	if [ ! -d "bin" ]; then mkdir -p bin; fi
