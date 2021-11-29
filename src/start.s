.section .start

.global start

start:

        // execute cpu setup on core 0
        mrs     x1, mpidr_el1
        and     x1, x1, #3
        cbz     x1, cpu_setup
        wfe
        b       .

cpu_setup:

        // enable FPU and SIMD for el1
        mov     x0, #3 << 20
        msr     cpacr_el1, x0

        #
        mov     x0, #1 << 31
        orr     x0, x0, #2
        msr     hcr_el2, x0

        adr     x0, rust_setup
        msr     elr_el2, x0

        mov     x0, #0x5
        msr     spsr_el2, x0

        // switch to elevation level 1

        // disable mmu
        //mov   x0, #0
        //orr       x0, x0, #1 << 11 // reserved (?)
        //orr       x0, x0, #1 << 20 // reserved (?)
        //orr       x0, x0, #3 << 22 // reserved (?)
        //orr       x0, x0, #3 << 28 // reserved (?)
        //orr       x0, x0, #1 << 12 // i cache
        //orr       x0, x0, #1 << 2 // d cache
        //msr       sctlr_el1, x0

        eret

rust_setup:

        // clear the bss section
        adr     x0, bss_base
        adr     x1, bss_limit
0:      str     xzr, [x0], #8
        cmp     x0, x1
        b.lt    0b

        // set the stack pointer
        adr     x30, stack_base
        mov     sp, x30

        // jump into rust
        bl      kernel_main
        b       .

.globl get_el
get_el:
        mrs     x0, CurrentEL
        lsr     x0, x0, #2
        ret
