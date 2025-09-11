all: build iso clean run

build:
	cargo build
	
iso: build
	mkdir -p iso/boot/grub/
	cp grub.cfg iso/boot/grub/grub.cfg
	cp target/i386/debug/kernel iso/boot/kernel.elf
	grub2-mkrescue iso -o os.iso

clean: iso
	rm iso -r

run:
	qemu-system-i386 -cdrom os.iso -nographic
	
.PHONY: build iso clean run
