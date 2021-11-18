ENTRY(start)

SECTIONS
{
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
    stack_base = .;
    . += 0xf000;

    . = 0x40000;
    heap_base = .;
    . = 0x4f000;

	. = ALIGN(4K); /* should be unnecessary */
	kernel_limit = .;
}
