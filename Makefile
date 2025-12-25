TMP_DIR=.tmp

all: clean build test

cargo:
	cargo build --release

$(TMP_DIR)/kernel.elf: cargo
	cp target/i386/release/kernel $@
$(TMP_DIR)/userspace1.elf: cargo
	cp target/i386/release/userspace1 $@
$(TMP_DIR)/userspace2.elf: cargo
	cp target/i386/release/userspace2 $@
$(TMP_DIR)/userspace3.elf: cargo
	cp target/i386/release/userspace3 $@
$(TMP_DIR)/userspace4.elf: cargo
	cp target/i386/release/userspace4 $@

$(TMP_DIR)/kernel.bin: $(TMP_DIR)/kernel.elf
	objcopy -O binary $< $@
$(TMP_DIR)/userspace1.bin: $(TMP_DIR)/userspace1.elf
	objcopy -O binary $< $@
$(TMP_DIR)/userspace2.bin: $(TMP_DIR)/userspace2.elf
	objcopy -O binary $< $@
$(TMP_DIR)/userspace3.bin: $(TMP_DIR)/userspace3.elf
	objcopy -O binary $< $@
$(TMP_DIR)/userspace4.bin: $(TMP_DIR)/userspace4.elf
	objcopy -O binary $< $@

os.img: $(TMP_DIR)/kernel.bin $(TMP_DIR)/userspace1.bin $(TMP_DIR)/userspace2.bin $(TMP_DIR)/userspace3.bin $(TMP_DIR)/userspace4.bin
	dd if=/dev/zero of=os.img bs=1024 count=1440
	dd if=$(word 1, $^) of=os.img conv=notrunc
	dd if=$(word 2, $^) of=os.img conv=notrunc oflag=seek_bytes seek=99328
	dd if=$(word 3, $^) of=os.img conv=notrunc oflag=seek_bytes seek=164864
	dd if=$(word 4, $^) of=os.img conv=notrunc oflag=seek_bytes seek=230400
	dd if=$(word 5, $^) of=os.img conv=notrunc oflag=seek_bytes seek=295936
	
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
