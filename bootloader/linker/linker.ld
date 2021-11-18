ENTRY(start)

SECTIONS
{
    . = 0x60000; /* COMPLETE OVERKILL */
    stack_base = .; /* stack grows down, so allocate with some space */
    heap_base = .;

    . = 0x80000;
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
