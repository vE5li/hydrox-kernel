ENTRY(start)

SECTIONS
{
    .text : {
        KEEP(*(.start))
    }

    . = 0x1000;
    interface_log_character = .;

    . = 0x1010;
    interface_read_event = .;

    . = 0x1020;
    interface_reboot = .;

    . = 0x1030;
    interface_peripherals_base = .;

    . += 8;

    .text : ALIGN(4K) {
        *(.text)
    }

    .bss : ALIGN(4K) {
        *(.bss)
    }

    .data : ALIGN(4K) {
        *(.data)
    }

    .rodata : ALIGN(4K) {
        *(.rodata)
    }

    . = 0x6000;

    kernel_base = .;
}
