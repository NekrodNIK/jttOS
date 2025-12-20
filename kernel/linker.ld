ENTRY(boot_entry)

SECTIONS {
    . = 0x7c00;

    .boot : {
        *(.boot_sector);
        _copy_start = .;
        *(.boot);
    }

    .kernel : ALIGN(512) {
        *(.vbe);
        *(.text);
        *(.text.*);
        *(.rodata);
        *(.rodata.*);
        *(.data);
        *(.data.*);
        *(.bss);
        *(.bss.*);
    }
    
    . = ALIGN(512);

    _copy_size = . - _copy_start;
    ASSERT(_copy_size <= 200 * 1024, "OS is too large, max 200KiB")
    _copy_sectors = _copy_size / 512;

    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.property)
        *(.comment)
    }
}

