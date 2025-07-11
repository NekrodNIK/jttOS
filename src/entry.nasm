global _start
extern kmain

STACK_SIZE equ 16*1024

section .bss
stack: align 16, resb STACK_SIZE

section .text
_start:
    mov esp, [stack+STACK_SIZE-1]
    mov ebp, esp
    call kmain
    hlt
