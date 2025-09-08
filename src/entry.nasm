global _start
extern kmain

MAGIC equ 0xe85250d6
ARCH  equ 0x0 ; x86

section .multiboot
header_start:
    dd MAGIC
    dd ARCH
    dd header_end - header_start
    dd 0x100000000 - (MAGIC + ARCH + header_end-header_start)

    ; framebuffer_tag_start:
    ;     dw  0x05    ;Type: framebuffer
    ;     dw  0x01    ;Optional tag
    ;     dd  framebuffer_tag_end - framebuffer_tag_start ;size
    ;     dd  0   ;Width - if 0 we let the bootloader decide
    ;     dd  0   ;Height - same as above
    ;     dd  0   ;Depth  - same as above
    ; framebuffer_tag_end:
    ;     dw 0 ; flags
    ;     dd 8 ; size      

    ; end tag
    dw 0 ; type
    dw 0 ; flags
    dd 8 ; size
header_end:

STACK_SIZE equ 16*1024

section .bss
stack: align 16, resb STACK_SIZE

section .text
_start:
    mov esp, stack+STACK_SIZE
    mov ebp, esp
    
    call kmain
    hlt
