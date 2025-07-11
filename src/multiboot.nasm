MAGIC equ 0xe85250d6
ARCH  equ 0x0 ; x86

section .multiboot
header_start:
    dd MAGIC
    dd ARCH
    dd header_end - header_start
    dd 0x100000000 - (MAGIC + ARCH + header_end-header_start)

    ; end tag
    dw 0 ; type
    dw 0 ; flags
    dd 8 ; size
header_end:
