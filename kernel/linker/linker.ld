SECTIONS
{
    . = 0x1000;
    interface_base = .;
    interface_log_character = .;

    . = 0x1010;
    interface_read_event = .;

    . = 0x1020;
    interface_reboot = .;

    . = 0x1030;
    interface_peripherals_base = .;

	. = 0x6000;
    kernel_base = .;

    .text : {
        KEEP(*(.start))
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
	   *(.rodata .rodata.*)
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
    }

    .bss : ALIGN(4K) {
        bss_base = .;
        *(.bss .bss.*)
        . = ALIGN(16);
        bss_limit = .;
    }

    .got : ALIGN(4K) {
        *(.got)
    }

    .got.plt : ALIGN(4K) {
        *(.got.plt)
    }

    .data.rel.ro : ALIGN(4K) {
        *(.data.rel.ro.local*)
        *(.data.rel.ro .data.rel.ro.*)
    }

    .gcc_except_table : ALIGN(4K) {
        *(.gcc_except_table)
    }

	. = ALIGN(4K);
	kernel_limit = .;
}
