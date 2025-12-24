ENTRY(boot_entry)

SECTIONS {
    . = 0x7c00;

    .boot : {
        *(.boot_sector);
        k_start = .;
        *(.boot);
    }

    .kernel : ALIGN(512) {
        *(.text);
        *(.text.*);
        *(.rodata);
        *(.rodata.*);
        *(.data);
        *(.data.*);
        *(.bss);
        *(.bss.*);
        RAM_SIZE = .;
        LONG(10 * 1024 * 1024)
        . = . + 4;
    }
    
    . = ALIGN(512);

    k_size = . - k_start;
    ASSERT(k_size <= 97 * 1024, "Kernel is too large, max 97 KiB")
    _copy_bytes = 0x58400;
    _copy_sectors = _copy_bytes / 512;

    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.property)
        *(.comment)
    }
}

