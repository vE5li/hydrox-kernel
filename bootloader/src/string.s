// compare two strings (macro)
.macro cmpstr destination source1 source2
        push    x1, x2
        push    x3, x30
        mov     x0, #0
        adr     x1, \source1
        adr     x2, \source2
        bl      compare_string
        pop     x3, x30
        pop     x1, x2
        mov     \destination, x0
.endm

// create a hexadecimal string from a register (macro)
.macro hexstr destination source length
        push    x1, x30
        push    x2, x3
        mov     x1, \source
        mov     x2, \length
        bl      hexadecimal_string
        pop     x2, x3
        pop     x1, x30
        mov     \destination, x0
.endm

// append character to string (macro)
.macro appstr source length character
        push    x1, x2
        push    x3, x30
        adr     x1, \source
        mov     x2, \length
        mov     x3, \character
        bl      append_string
        pop     x3, x30
        pop     x1, x2
.endm

// append character to string stored in register (macro)
.macro appstrr source length character
        push    x1, x2
        push    x3, x30
        mov     x1, \source
        mov     x2, \length
        mov     x3, \character
        bl      append_string
        pop     x3, x30
        pop     x1, x2
.endm

// clear a string (macro)
.macro clrstr source
        push    x1, x2
        adr     x1, \source
        strb    wzr, [x1]
        pop     x1, x2
.endm

.section .rodata

// hexadecimal lookup table
s hextable
        .byte '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'

.section .data

// space reserved for building strings
s string_space
        .space 64

.section .text

// compare two strings
//  x0              < 0 if strings do match, otherwise the index of the check
//  x1              > base of first string
//  x2              > base of seconds string
s compare_string
        push    x3, x4
        mov     x3, x0
0:      ldrb    w4, [x1, x3]
        ldrb    w0, [x2, x3]
        add     x3, x3, #1
        cmp     w4, w0
        b.ne    1f
        orr     w0, w0, w4
        cbz     w0, 2f
        b       0b
1:      mov     x0, x3
2:      pop     x3, x4
        ret

// create a hexadecimal string from a register
//  x0              < base of the return string
//  x1              > source register for the hexadecimal string
//  x2              > number of gigits the string is supposed to have
s hexadecimal_string
        adr     x0, string_space
        push    x3, x4
        push    x5, x6
        adr     x3, hextable
        mov     x4, #0
        sub     x5, x2, #1
        lsl     x5, x5, #2
0:      sub     x6, x5, x4, lsl #2
        lsr     x6, x1, x6
        and     x6, x6, #15
        ldrb    w6, [x3, x6]
        strb    w6, [x0, x4]
        add     x4, x4, #1
        cmp     x4, x2
        b.ne    0b
        strb    wzr, [x0, x4]
        pop     x5, x6
        pop     x3, x4
        ret

// append a character to a string
//  x1              > string base
//  x2              > length of the string
//  x3              > character to be appended
s append_string
        push    x4, x5
        mov     x0, #0
        mov     x5, x0
0:      ldrb	w4, [x1, x5]
        add     x5, x5, #1
        cmp     x5, x2
        b.eq    1f
        cbnz	w4, 0b
        strb	wzr, [x1, x5]
        mov     x0, x5
        sub     x5, x5, #1
        strb	w3, [x1, x5]
1:      pop     x4, x5
        ret
