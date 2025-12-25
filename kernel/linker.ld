ENTRY(boot_entry)

SECTIONS {
    . = 0x7c00;

    .boot : {
        *(.boot_sector);
        _copy_start = .;
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

    _copy_size = . - _copy_start;
    ASSERT(_copy_size <= 480 * 1024, "OS is too large, max 480KiB")
    _copy_sectors = _copy_size / 512;

    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.property)
        *(.comment)
    }
}

