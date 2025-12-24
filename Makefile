TMP_DIR=.tmp

all: clean build test

$(TMP_DIR)/kernel.elf:
	cargo build --release --package=kernel
	cp target/i386/release/kernel $@
	
$(TMP_DIR)/userspace.elf:
	cargo build --release --package=userspace
	cp target/i386/release/userspace $@

$(TMP_DIR)/kernel.bin: $(TMP_DIR)/kernel.elf
	objcopy -O binary $< $@

$(TMP_DIR)/userspace.bin: $(TMP_DIR)/userspace.elf
	objcopy -O binary $< $@

os.img: $(TMP_DIR)/kernel.bin $(TMP_DIR)/userspace.bin
	dd if=/dev/zero of=os.img bs=1024 count=1440
	dd if=$(word 1, $^) of=os.img conv=notrunc
	dd if=$(word 2, $^) of=os.img conv=notrunc oflag=seek_bytes seek=99328
	
build: os.img

clean:
	rm -f os.img
	rm -rf $(TMP_DIR)
	mkdir $(TMP_DIR)

test: build
	qemu-system-i386 -cpu pentium2 -m 4G -hda os.img -monitor stdio -device VGA

debug: build
	qemu-system-i386 -cpu pentium2 -m 4G -hda os.img -monitor stdio -device VGA -s -S &
	rust-gdb .tmp/kernel.elf

.PHONY: all build clean test debug
