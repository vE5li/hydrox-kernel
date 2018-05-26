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

    . = 0x8000;

    kernel_base = .;
}
