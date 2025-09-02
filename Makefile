all: build iso run clean

build:
	cargo build
	
iso: build
	mkdir -p iso/boot/grub/
	cp src/grub.cfg iso/boot/grub/grub.cfg
	cp target/i386/debug/kernel iso/boot/kernel.elf
	grub2-mkrescue iso -o os.iso

clean: iso
	rm iso -r

run:
	
