global framebuffer_addr
global framebuffer_height
global framebuffer_width

MODE_WIDTH  equ 640
MODE_HEIGHT equ 400
MODE_BPP    equ 32

struc VBEInfoBlock
	.Signature             resb 4
	.Version               resw 1
	.OEMNamePtr            resd 1
	.Capabilities          resd 1

	.VideoModesOffset      resw 1
	.VideoModesSegment     resw 1

	.CountOf64KBlocks      resw 1
	.OEMSoftwareRevision   resw 1
	.OEMVendorNamePtr      resd 1
	.OEMProductNamePtr     resd 1
	.OEMProductRevisionPtr resd 1
	.Reserved              resb 222
	.OEMData               resb 256
endstruc

struc VBEModeInfoBlock
    .ModeAttributes	        resw 1
    .FirstWindowAttributes	resb 1
    .SecondWindowAttributes	resb 1
    .WindowGranularity	    resw 1
    .WindowSize		        resw 1
    .FirstWindowSegment	    resw 1
    .SecondWindowSegment	resw 1
    .WindowFunctionPtr	    resd 1
    .BytesPerScanLine	    resw 1
	.Width			        resw 1
	.Height			        resw 1
	.CharWidth		        resb 1		
	.CharHeight		        resb 1		
	.PlanesCount		    resb 1
	.BitsPerPixel		    resb 1
	.BanksCount		        resb 1
	.MemoryModel		    resb 1		
	.BankSize		        resb 1	
	.ImagePagesCount	    resb 1	
	.Reserved1		        resb 1	
	.RedMaskSize		    resb 1
	.RedFieldPosition	    resb 1
	.GreenMaskSize		    resb 1
	.GreenFieldPosition	    resb 1
	.BlueMaskSize		    resb 1
	.BlueFieldPosition	    resb 1
	.ReservedMaskSize	    resb 1
	.ReservedMaskPosition	resb 1
	.DirectColorModeInfo	resb 1
	.LFBAddress		        resd 1
	.OffscreenMemoryOffset	resd 1
	.OffscreenMemorySize	resw 1
	.Reserved2		        resb 206
endstruc

bits 16
setup_framebuffer:
    push es
    push fs

    xor ax, ax
    mov es, ax
; =====================
;     Get VBE Info
; =====================
    mov ax, 0x4f00
    mov di, info_block
    int 0x10  

    cmp ax, 0x4f
    mov di, errors.vbe_not_supported
    jne print_error
; =====================
;    Select VBE mode
; =====================
    mov fs, word [info_block + VBEInfoBlock.VideoModesSegment]
	mov si, word [info_block + VBEInfoBlock.VideoModesOffset]

.mode_loop:
    mov cx, word [fs:si]
    add si, 2

    cmp cx, 0xffff
    je .error
    
    mov ax, 0x4f01
    mov di, mode_info_block
    int 0x10
    cmp ax, 0x4f
    jne .mode_loop

    ; checking if the mode is graphics
    mov ax, word [mode_info_block + VBEModeInfoBlock.ModeAttributes]
    and ax, 1 << 4
    jz .mode_loop
    
    ; checking if the mode is linear
    mov ax, word [mode_info_block + VBEModeInfoBlock.ModeAttributes]
    and ax, 1 << 7
    jz .mode_loop

    ; checking width
    mov ax, word [mode_info_block + VBEModeInfoBlock.Width]
    cmp ax, MODE_WIDTH
    jne .mode_loop
    
    ; checking height
    mov ax, [mode_info_block + VBEModeInfoBlock.Height]
    cmp ax, MODE_HEIGHT
    jne .mode_loop
    
    ; checking bpp
    mov al, [mode_info_block + VBEModeInfoBlock.BitsPerPixel]
    cmp al, MODE_BPP
    jne .mode_loop

    jmp .mode_loop_end
.error:
    mov di, errors.vbe_mode_not_found
    jmp print_error
.mode_loop_end:
    mov ax, 0x4f02
	mov bx, cx
	or bx, 0x4000  ; enable lfb
	mov di, 0
    int 0x10

    cmp ax, 0x4f
    mov di, errors.vbe_set_error
    jne print_error
    
    mov eax, dword [mode_info_block + VBEModeInfoBlock.LFBAddress]
    mov dword [framebuffer_addr], eax
    mov bx, word [mode_info_block + VBEModeInfoBlock.Width]
    mov word [framebuffer_width], bx
    mov cx, word [mode_info_block + VBEModeInfoBlock.Height]
    mov word [framebuffer_height], cx
    
    pop fs
    pop es
    cli
    ret

info_block:
    istruc VBEInfoBlock
        at .Signature, db "VBE2"
    iend

mode_info_block:
    istruc VBEModeInfoBlock
    iend

framebuffer_addr dd 0
framebuffer_width dw 0
framebuffer_height dw 0

errors:
    .vbe_not_supported dw "VBE: not supported", 0
    .vbe_mode_not_found dw "VBE: mode not found", 0
    .vbe_set_error dw "VBE: set mode error", 0
