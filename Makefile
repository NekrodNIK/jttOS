TMP_DIR=.tmp

all: clean build test

$(TMP_DIR)/kernel.elf:
	cargo build
	cp target/i386/debug/kernel $@

$(TMP_DIR)/kernel.bin: $(TMP_DIR)/kernel.elf
	objcopy -O binary $< $@

os.img: $(TMP_DIR)/kernel.bin
	dd if=/dev/zero of=os.img bs=1024 count=1440
	dd if=$< of=os.img conv=notrunc

build: os.img

clean:
	rm -f os.img
	rm -rf $(TMP_DIR)
	mkdir $(TMP_DIR)

test: build
	qemu-system-i386 -cpu pentium2 -m 1g -fda os.img -monitor stdio -device VGA

debug: build
	qemu-system-i386 -cpu pentium2 -m 1g -fda os.img -monitor stdio -device VGA -s -S &
	gdb .tmp/kernel.elf

.PHONY: all build clean test debug
