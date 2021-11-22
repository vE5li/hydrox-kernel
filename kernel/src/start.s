.section .start

.global start

start:
        mrs     x1, mpidr_el1
        and     x1, x1, #3
        cbz     x1, rust_setup
        wfe
        b       .

rust_setup:
        adr     x0, bss_base
        adr     x1, bss_limit
0:      str     xzr, [x0], #8
        cmp     x0, x1
        b.lt    0b
        adr		x30, stack_base
        mov 	sp, x30
        bl 		kernel_main
        b       .
