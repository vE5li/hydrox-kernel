.section .start

// assembly entry point
g start
        mrs     x1, mpidr_el1
        and     x1, x1, #3
        cbz     x1, parse_parameters
        wfe
        b       .

// parse the command line arguments
s parse_parameters
        adr     x1, kernel_base
        mov     sp, x1
        // parse parameters
        actled  #1
        mov     x29, #0

// entry point for soft resets
g common_entry
        // TODO: serial or ethernet depending on the parameters
        b       serial_entry
        b       ethernet_entry

// run the kernel if it is loaded
g boot_kernel
        adr     x30, kernel_base
        br      x30

// reboot the device
g reboot
        // TODO: do a proper reboot
        b       parse_parameters
