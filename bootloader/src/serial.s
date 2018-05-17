// log a character (macro)
.macro slgch character
        push    x1, x30
        mov     w1, \character
        bl      serial_log_character
        pop     x1, x30
.endm

// log a string from address (macro)
.macro slgstr string_base
        push    x1, x30
        adr     x1, \string_base
        bl      serial_log_string
        pop     x1, x30
.endm

// log a string from a register (macro)
.macro slgstrr base_register
        push    x1, x30
        mov     x1, \base_register
        bl      serial_log_string
        pop     x1, x30
.endm

// log success
.macro slgsuc
        slgch   '!'
.endm

// log an error
.macro slgerr message
        slgch   '-'
        slgch   '>'
        slgch   ' '
        slgstr  error_\message
.endm

// log s register (macro)
.macro slgreg number
        push    x1, x2
        push    x3, x30
        slgch   '\n'
        mov     x1, #\number
        mov     x2, x\number
        bl      serial_log_register
        pop     x3, x30
        pop     x1, x2
.endm

// get the next character (macro)
.macro srdch destination
        push    x1, x2
        periph  x1, serial
0:      ldr     w0, [x1, #20]
        and     w0, w0, #1
        cbz     w0, 0b
        ldrb    w0, [x1]
        pop     x1, x2
        mov     \destination, w0
.endm

// shift the bits left by 8 and set the zeroed bits from serial (macro)
.macro srdshft destination source
        push    x1, x2
        lsl     w1, \source, #8
        srdch   w2
        orr     w0, w1, w2
        pop     x1, x2
        mov     \destination, w0
.endm

// compare the command buffer to a string and jump to destination on match (macro)
.macro cmpcmdb identifier destination
        cmpstr  x0, command_buffer, command_\identifier
        cbz     x0, \destination
.endm

.section .rodata

// user interface messages
s register_spacing
        .asciz  " ................ "
s help_message
        .ascii  "\nload .............. load the kernel"
        .ascii  "\ndump .............. display a hexdump of the loaded kernel"
        .ascii  "\nboot .............. boot the loaded kernel"
        .ascii  "\nreboot ............ reboots the device"
        .ascii  "\ncommand ........... print the command line passed to the bootloader"
        .ascii  "\nregisters ......... display the contents of all registers"
        .asciz  "\nhelp .............. list all avalible commands"

// commands
s command_load_kernel
        .asciz  "load"
s command_dump_kernel
        .asciz  "dump"
s command_boot_kernel
        .asciz  "boot"
s command_reboot
        .asciz  "reboot"
s command_command
        .asciz  "command"
s command_registers
        .asciz  "registers"
s command_help
        .asciz  "help"

// error messages
s error_missing_kernel
        .asciz  "no kernel image loaded"

.section .data

// space reserved for user input
s command_buffer
        .space 64

.section .text

// add a character to the debug log
//  x1              > character to be logged
s serial_log_character
        push    x2, x3
        periph  x2, serial
        cmp     w1, '\n'
        b.ne    1f
        slgch   '\r'
1:      ldr     w3, [x2, #20]
        and     w3, w3, #32
        cbz     w3, 1b
        strb    w1, [x2]
        pop     x2, x3
        ret

// add a string to the debug log
//  x1              > base of the string to be logged
s serial_log_string
        push    x2, x3
        periph  x2, serial
10:     ldrb    w3, [x1], #1
        cbz     w3, 11f
        slgch   w3
        b       10b
11:     pop     x2, x3
        ret

// get keyboard input from the user
//  x0              < modifier mask and keycode
s serial_read_event
        push    x1, x2
        srdshft w0, wzr
        srdshft w0, w0
        pop     x1, x2
        ret

// check if a kernel loader is avalible
s serial_entry
        strint  serial_log_character, log_character
        strint  serial_read_event, read_event
        slgch   '?'
        bl      serial_load_kernel
        b       boot_kernel

// user interface for the bootloader
s serial_interface
        srdch   w1
        cmp     w1, #13
        b.eq    serial_evaluate_command
        mov     w1, w0
        appstr  command_buffer, #64, x0
        cbz     x0, serial_interface
        slgch   w1
        b       serial_interface

// handle the entered command
s serial_evaluate_command
        slgch   ' '
        cmpcmdb load_kernel, serial_load_kernel_clear
        cmpcmdb dump_kernel, serial_dump_kernel
        cmpcmdb boot_kernel, serial_boot_kernel
        cmpcmdb reboot, serial_reboot
        cmpcmdb command, serial_log_command
        cmpcmdb registers, serial_log_registers
        cmpcmdb help, serial_help

// clear the screen and insert a new command prompt
s serial_clear
        slgch   '\n'
        slgch   '>'
        slgch   ' '
        clrstr  command_buffer
        b       serial_interface

// laod the kernel
s serial_load_kernel
        srdch   w1
        cmp     w1, '!'
        b.ne    serial_clear
        adr     x1, kernel_base
        srdshft w2, wzr
        srdshft w2, w2
        srdshft w2, w2
        srdshft w2, w2
        mov     x29, #0
10:     srdch   w3
        strb    w3, [x1, x29]
        add     x29, x29, #1
        cmp     w2, w29
        b.ne    10b
        ret

// load the kernel via serial
s serial_load_kernel_clear
        slgsuc
        bl      serial_load_kernel
        b       serial_clear

// dump the loaded binary kernel as hexadecimal
s serial_dump_kernel
        cbz     x29, 12f
        slgsuc
        adr     x1, kernel_base
        mov     x2, #0
10:     mov     x3, #0
        slgch   '\n'
11:     ldrb    w4, [x1, x2]
        hexstr  x4, x4, #2
        slgstrr x4
        add     x2, x2, #1
        cmp     x29, x2
        b.eq    serial_clear
        add     x3, x3, #1
        cmp     x3, #32
        b.eq    10b
        slgch   ' '
        b       11b
        b       serial_clear
12:     slgerr  missing_kernel
        b       serial_clear

// format and run the loaded kernel
s serial_boot_kernel
        cbnz    x29, 10f
        slgerr  missing_kernel
        b       serial_clear
10:     slgsuc
        slgch   '\n'
        clrstr  command_buffer
        b       boot_kernel
        slgch   '\n'

// reboot the device
s serial_reboot
        slgsuc
        slgch   '\n'
        b       reboot

// log the command line arguments passed to the bootloader
s serial_log_command
        slgsuc
        b       serial_clear

// log a single register
//  x1              > register number
//  x2              > source register
s serial_log_register
        hexstr  x1, x1, #2
        slgstrr x1
        slgstr  register_spacing
        hexstr  x2, x2, #16
        slgstrr x2
        ret

// display the content of all registers
s serial_log_registers
        slgsuc
        slgreg  0
        slgreg  1
        slgreg  2
        slgreg  3
        slgreg  4
        slgreg  5
        slgreg  6
        slgreg  7
        slgreg  8
        slgreg  9
        slgreg  10
        slgreg  11
        slgreg  12
        slgreg  13
        slgreg  14
        slgreg  15
        slgreg  16
        slgreg  17
        slgreg  18
        slgreg  19
        slgreg  20
        slgreg  21
        slgreg  22
        slgreg  23
        slgreg  24
        slgreg  25
        slgreg  26
        slgreg  27
        slgreg  28
        slgreg  29
        slgreg  30
        b       serial_clear

// display a help menu
s serial_help
        slgsuc
        slgstr  help_message
        b       serial_clear
