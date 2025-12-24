ENTRY(_start)

SECTIONS {
    . = 0x800000;
    
    u_start = .;
    .text : {
        *(.text)
        *(.text.*);
    }

    .rodata : {
        *(.rodata);
        *(.rodata.*);
    }

    .data : {
        *(.data);
        *(.data.*);
    }

    .bss : {
        *(.bss);
        *(.bss.*);
    }
    u_end = .;
    
    ASSERT(u_end - u_start <= 64 * 1024, "Userspace is too large, max 64KiB")
    
    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.property)
        *(.comment)
    }
}

