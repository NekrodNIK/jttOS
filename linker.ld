ENTRY(boot_entry)

SECTIONS {
    . = 0x7c00;

    .boot : {
        *(.boot);
    }

    _kernel_start = .;
    
    .kernel : ALIGN(512) {
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

    _kernel_size = . - _kernel_start;
    ASSERT(_kernel_size <= 200 * 1024, "kernel is too large, max 200KiB")
    _kernel_sectors = _kernel_size / 512;

    _cs_selector = cs_desc - gdt;
    _ds_selector = ds_desc - gdt;

    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.property)
        *(.comment)
    }
}

