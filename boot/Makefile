NASM=nasm -fbin

PAYLOAD_SIZE_KB=450
PAYLOAD_SECTORS=$(shell echo $$(($(PAYLOAD_SIZE_KB) * 2)))

all: clean build test

build/payload.bin: /dev/urandom
	dd if=$< of=$@ bs=1024 count=$(PAYLOAD_SIZE_KB)

build/boot0.bin: src/boot0.nasm
	$(NASM) -o $@ $< -dN=$(PAYLOAD_SECTORS)

os.img: build/boot0.bin build/payload.bin
	dd if=/dev/zero of=os.img bs=1024 count=1440
	dd if=$< of=os.img conv=notrunc
	dd if=$(word 2, $^) of=os.img conv=notrunc bs=512 seek=1

build: os.img

clean:
	rm -f os.img
	rm -rf build
	mkdir build

test: build
	qemu-system-i386 -cpu pentium2 -m 1g -fda os.img -monitor stdio -device VGA

debug: build
	qemu-system-i386 -cpu pentium2 -m 1g -fda os.img -monitor stdio -device VGA -s -S &
	gdb

.PHONY: all build clean test debug
