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

        // switch to elevation level 1

        // 1 << 31: el1 is aarch64
        // 1 << 1: trap to el2 when a lookup on device memory happens
        // 1: invalidate cache and perform a cache clear
        mov     x0, #1 << 31
        orr     x0, x0, #2
        msr     hcr_el2, x0

        adr     x0, rust_setup
        msr     elr_el2, x0

        mov     x0, #0x5
        msr     spsr_el2, x0

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

.macro kernel_entry
        sub     sp, sp, #256
        stp     x0, x1, [sp, #16 * 0]
        stp     x2, x3, [sp, #16 * 1]
        stp     x4, x5, [sp, #16 * 2]
        stp     x6, x7, [sp, #16 * 3]
        stp     x8, x9, [sp, #16 * 4]
        stp     x10, x11, [sp, #16 * 5]
        stp     x12, x13, [sp, #16 * 6]
        stp     x14, x15, [sp, #16 * 7]
        stp     x16, x17, [sp, #16 * 8]
        stp     x18, x19, [sp, #16 * 9]
        stp     x20, x21, [sp, #16 * 10]
        stp     x22, x23, [sp, #16 * 11]
        stp     x24, x25, [sp, #16 * 12]
        stp     x26, x27, [sp, #16 * 13]
        stp     x28, x29, [sp, #16 * 14]
        str     x30, [sp, #16 * 15]
.endm

.macro kernel_exit
        ldp     x0, x1, [sp, #16 * 0]
        ldp     x2, x3, [sp, #16 * 1]
        ldp     x4, x5, [sp, #16 * 2]
        ldp     x6, x7, [sp, #16 * 3]
        ldp     x8, x9, [sp, #16 * 4]
        ldp     x10, x11, [sp, #16 * 5]
        ldp     x12, x13, [sp, #16 * 6]
        ldp     x14, x15, [sp, #16 * 7]
        ldp     x16, x17, [sp, #16 * 8]
        ldp     x18, x19, [sp, #16 * 9]
        ldp     x20, x21, [sp, #16 * 10]
        ldp     x22, x23, [sp, #16 * 11]
        ldp     x24, x25, [sp, #16 * 12]
        ldp     x26, x27, [sp, #16 * 13]
        ldp     x28, x29, [sp, #16 * 14]
        ldr     x30, [sp, #16 * 15]
        add     sp, sp, #256
        eret
.endm

.macro handle_invalid_entry type
        kernel_entry
        mov     x0, #\type
        mrs     x1, esr_el1
        mrs     x2, elr_el1

        // TODO: call error formatter
        b       .
.endm

.macro vector_entry label
.align  7
        b   \label
.endm

.align  11
.globl vectors
vectors:
        vector_entry sync_invalid_el1t          // Synchronous EL1t
        vector_entry irq_invalid_el1t           // IRQ EL1t
        vector_entry fiq_invalid_el1t           // FIQ EL1t
        vector_entry error_invalid_el1t         // Error EL1t

        vector_entry sync_invalid_el1h          // Synchronous EL1h
        vector_entry handle_el1_irq             // IRQ EL1h
        vector_entry fiq_invalid_el1h           // FIQ EL1h
        vector_entry error_invalid_el1h         // Error EL1h

        vector_entry sync_invalid_el0_64        // Synchronous 64-bit EL0
        vector_entry irq_invalid_el0_64         // IRQ 64-bit EL0
        vector_entry fiq_invalid_el0_64         // FIQ 64-bit EL0
        vector_entry error_invalid_el0_64       // Error 64-bit EL0

        vector_entry sync_invalid_el0_32        // Synchronous 32-bit EL0
        vector_entry irq_invalid_el0_32         // IRQ 32-bit EL0
        vector_entry fiq_invalid_el0_32         // FIQ 32-bit EL0
        vector_entry error_invalid_el0_32       // Error 32-bit EL0


sync_invalid_el1t:
        handle_invalid_entry  0

irq_invalid_el1t:
        handle_invalid_entry  1

fiq_invalid_el1t:
        handle_invalid_entry  2

error_invalid_el1t:
        handle_invalid_entry  3

sync_invalid_el1h:
        handle_invalid_entry  4

fiq_invalid_el1h:
        handle_invalid_entry  6

error_invalid_el1h:
        handle_invalid_entry  7

sync_invalid_el0_64:
        handle_invalid_entry  8

irq_invalid_el0_64:
        handle_invalid_entry  9

fiq_invalid_el0_64:
        handle_invalid_entry  10

error_invalid_el0_64:
        handle_invalid_entry  11

sync_invalid_el0_32:
        handle_invalid_entry  12

irq_invalid_el0_32:
        handle_invalid_entry  13

fiq_invalid_el0_32:
        handle_invalid_entry  14

error_invalid_el0_32:
        handle_invalid_entry  15

handle_el1_irq:
        kernel_entry
        bl      handle_irq
        kernel_exit

.globl irq_init_vectors
irq_init_vectors:
        adr     x0, vectors
        msr     vbar_el1, x0
        ret

.globl irq_enable
irq_enable:
        msr daifclr, #2
        ret

.globl irq_disable
irq_disable:
        msr daifset, #2
        ret
