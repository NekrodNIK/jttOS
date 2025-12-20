global boot_entry
extern kentry
extern _copy_sectors
extern GDT_DESC

section .boot_sector
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

    ; Disk number in dl
    call check_edd
    call read_disk
    call setup_framebuffer
    push word eax
    
    lgdt [GDT_DESC]
    cld
    mov eax, cr0
    or al, 1
    mov cr0, eax
    jmp 8:trampoline
    
bits 32
trampoline:
    mov ax, 16
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    call kentry

bits 16
check_edd:
    mov ah, 0x41
    mov bx, 0x55aa
    int 0x13
    jc .error
    cmp bx, 0xaa55
    jne .error
    test cx, 1
    jz .error
    ret  
.error:
    mov di, msg.no_edd
    jmp print_error
    
read_disk:
    mov dword [lba_dap.block_l], 1
    mov dword [lba_dap.block_h], 0
    mov word [lba_dap.offset], 0
    
    mov cx, _copy_sectors
    mov di, 0x7e0
read_loop:
    mov byte [lba_dap.count], 1
    mov word [lba_dap.segment], di
    
    mov si, lba_dap
    mov ah, 0x42
    int 0x13
    jc .error

    inc dword [lba_dap.block_l]
    jnc .next
    inc dword [lba_dap.block_h]
.next:
    add di, (512 >> 4)
    loop read_loop
    ret
.error:
    mov di, msg.disk
    jmp print_error

print_error:
    ; di - null-terminated string
    mov al, [di]
    test al, al
    jz .end
    
    mov ah, 0xe
    int 0x10
    
    inc di
    jmp print_error
.end:
    hlt

; ==========
;    DATA
; ==========
msg:
.disk:
    db `\n\r`
    db 'Disk read error!', 0
.no_edd:
    db `\n\r`
    db 'BIOS EDD extensions is not supported', 0
    
lba_dap:
    db 0x10 
    db 0
.count: 
    db 1
    db 0
.offset:
    dw 0
.segment:
    dw 0     
.block_l:
    dd 0     
.block_h:
    dd 0 

times 510-($-$$) db 0
dw 0xaa55

section .boot
%include "src/vbe.nasm"

