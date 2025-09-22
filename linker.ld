ENTRY(boot_entry)

SECTIONS {
    . = 0x7c00;

    .boot : {
        *(.boot);
    }

    .kernel : {
        *(.text);
        *(.rodata);
        *(.rodata.*);
        *(.data);
        *(.bss);
    }

    /DISCARD/ : {
        *(.eh_frame)
        *(.note.gnu.property)
        *(.comment)
    }
}

kernel_size = SIZEOF(.kernel);
ASSERT(kernel_size <= 200 * 1024, "kernel is too large, max 200KiB");
