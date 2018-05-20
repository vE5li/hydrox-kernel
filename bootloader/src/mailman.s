// reset the mailbox request structure (macro)
.macro resmb
        push    x1, x30
        bl      reset_mailbox
        pop     x1, x30
.endm

// append a register (macro)
.macro appmb source
        push    x1, x30
        mov     w1, \source
        bl      append_mailbox
        pop     x1, x30
.endm

// append a tag code by name (macro)
.macro appmbtc name
        push    x1, x30
        ldr     w1, tag_code_\name
        bl      append_mailbox
        pop     x1, x30
.endm

// push the request structure to the gpu (macro)
.macro pushmb channel
        push    x1, x30
        mov     w1, \channel
        bl      push_mailbox
        pop     x1, x30
.endm

// pop a response from the mailbox (macro)
.macro popmb destination channel
        push    x1, x30
        mov     w1, \channel
        bl      pop_mailbox
        pop     x1, x30
        mov     \destination, x0
.endm

// set the act leds state (macro)
.macro actled state
        resmb
        appmbtc led
        appmb   #8
        appmb   #0
        appmb   #130
        appmb   \state
        pushmb  #8
        popmb   xzr, #8
        resmb
.endm

.section .rodata

// tag code to select the power state of an led
sa 16, tag_code_led
        .word   0x38041

// taag to select the power state of any resource
sa 16, tag_code_power
        .word   0x28001

.section .data

// space for constructing the requests
sa 16, request_space
        .space  256

.section .text

// reset the mailbox request structure
s reset_mailbox
        push    x1, x2
        adr     x1, request_space
        mov     w2, #8
        str     w2, [x1]
        str     wzr, [x1, #4]
        pop     x1, x2
        ret

// append 32 bits to the mailbox request
// x1               > data to be appended
s append_mailbox
        push    x2, x3
        adr     x2, request_space
        ldr     w3, [x2]
        str     w1, [x2, x3]
        add     w3, w3, #4
        str     w3, [x2]
        pop     x2, x3
        ret

// push the request structure to the gpu
// x1               > channel to send to
s push_mailbox
        push    x2, x3
        appmb   #0
        periph  x2, mailbox
        adr     x3, request_space
        orr     w3, w3, w1
0:      ldr     w1, [x2, #56]
        and     w1, w1, #1 << 31
        cbnz    w1, 0b
        str     w3, [x2, #32]
        pop     x2, x3
        ret

// pop a response from the mailbox
// x0               < base address + 8 of the request strcture
// x1               > channel to read from
s pop_mailbox
        push    x2, x3
        adr     x0, request_space
        periph  x2, mailbox
0:      ldr     w3, [x2, #24]
        and     w3, w3, #1 << 30
        cbnz    w3, 0b
        ldr     w3, [x2]
        and     w3, w3, #0b1111
        cmp     w3, w1
        b.ne    0b
        pop     x2, x3
        ret
