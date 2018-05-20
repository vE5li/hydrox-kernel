// global symbol (macro)
.macro g identifier
        .global \identifier
        \identifier:
.endm

// push (macro)
.macro push source1, source2
        stp     \source1, \source2, [sp, #-16]!
.endm

// pop (macro)
.macro pop destination1, destination2
        ldp     \destination1, \destination2, [sp], #16
.endm

// branch to interface funtion
.macro bint workspace identifier
        ldr     \workspace, interface_\identifier
        blr     \workspace
.endm

.section .start

// kernel entry point
//  sets up a stack and zeros the bss section before calling rusts kernel_main.
g start
        adr     x0, bss_base
        adr     x1, bss_limit
0:      str     xzr, [x0], #8
        cmp     x0, x1
        b.lt    0b
        adr		x30, kernel_base
        mov 	sp, x30
        b 		kernel_main

// log a character throught the bootloader
g log_character
        push    x1, x2
        push    x3, x30
        mov     w1, w0
        bint    x2, log_character
        pop     x3, x30
        pop     x1, x2
        ret

// get user input from the bootloader
//  returns 16 relevant bits; 15-8 = modifier; keys 7-0 = keycode
g read_event
        push    x1, x30
        bint    x1, read_event
        pop     x1, x30
        ret
