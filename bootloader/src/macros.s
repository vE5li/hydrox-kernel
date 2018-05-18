// symbol (macro)
.macro s identifier
        \identifier:
.endm

// aligned symbol (macro)
.macro sa alignment identifier
        .balign \alignment
        \identifier:
.endm

// global symbol (macro)
.macro g identifier
        .global \identifier
        \identifier:
.endm

// global aligned symbol (macro)
.macro ga alignment identifier
        .balign \alignment
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

// set an interface item (macro)
.macro strint source destination
        push    x1, x2
        adr     x1, \source
        adr     x2, interface_\destination
        str     x1, [x2]
        pop     x1, x2
.endm

// set an interface item from register (macro)
.macro strintr source destination
        push    x1, x2
        mov     x1, \source
        adr     x2, interface_\destination
        str     x1, [x2]
        pop     x1, x2
.endm

// get a peripheral address from offet (macro)
.macro periph destination suffix
        push    x1, x2
        mov     x1, #0x3f00
        lsl     x1, x1, #16
        ldr     x2, peripheral_offset_\suffix
        orr     x0, x1, x2
        pop     x1, x2
        mov     \destination, x0
.endm
