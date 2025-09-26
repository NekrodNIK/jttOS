global boot_entry
extern kmain
extern _kernel_sectors

LIM_CYLINDER equ 80
LIM_HEAD equ 2
LIM_SECTOR equ 19

section .boot
; ==========
;    CODE
; ==========
bits 16
boot_entry:
    cli

    ; Set segment registers and setup stack
    xor ax, ax
    mov ds, ax
    mov ss, ax
    mov sp, 0x7c00

    ; Setting before read
    mov ax, 0x7e0
    mov es, ax
    xor bx, bx

    mov di, _kernel_sectors
    
    xor dh, dh
    xor ch, ch
    mov cl, 2
    
    ; ==========================================================
    ;    dl - disk number
    ; es:bx - starting address
    ;    di - number of sectons to read
    ;    dh - starting head number
    ;    ch - low 8 bits of starting cylinder number
    ;    cl - starting sector number (bits 0-5),
    ;         high 2 bits of starting cylinder number (bits 6-7)
    ; ==========================================================
read_loop:
    mov ah, 0x2
    mov al, 1

    int 0x13    
    jc disk_err

    inc cl
    cmp cl, LIM_SECTOR
    jne .next
    
    mov cl, 1
    inc dh
    cmp dh, LIM_HEAD
    jne .next

    xor dh, dh
    inc ch
    cmp ch, LIM_CYLINDER
    je disk_err
.next:
    mov ax, es
    add ax, (512 >> 4)
    mov es, ax

    dec di
    jnz read_loop
    jmp switch_protected
        
; Display an error message,
; wait press any key
; and try reading the sector again
disk_err:
    mov ah, 0xe
    mov si, msg.disk

.loop:
    mov al, [si]
    test al, al
    jz .end
    
    int 0x10
    
    inc si
    jmp .loop
.end:
    mov ah, 0
    int 0x16
    jmp read_loop

    
switch_protected:
    lgdt [gdt_desc]
    cld
    mov eax, cr0
    or al, 1
    mov cr0, eax
    jmp (cs_desc-gdt):trampoline

bits 32
trampoline:
    mov ax, (ds_desc-gdt)
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    call kmain

; ==========
;    DATA
; ==========
msg:
.disk:
    db `\n\r`
    db 'Sector read error!', `\n\r`
    db 'Press any key...', `\n\r`, 0

align 8
gdt:
    dq 0
cs_desc:
    .limitLow:                dw 0xff
    .baseLow:                 dw 0
    .baseMid:                 db 0
    .p_dpl_s_type:            db 0b1001_1010
    .g_dl_l_avl_limitHigh:    db 0b1100_1111
    .baseHigh:                db 0

ds_desc:
    .limitLow:                dw 0xff
    .baseLow:                 dw 0
    .baseMid:                 db 0
    .p_dpl_s_type:            db 0b1001_0001
    .g_dl_l_avl_limitHigh:    db 0b1100_1111
    .baseHigh:                db 0

gdt_desc:
    dw 0x17
    dd gdt

times 510-($-$$) db 0
dw 0xaa55
