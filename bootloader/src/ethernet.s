.section .text

// initialize the root usb hub
s ethernet_entry
        appmbtc power
        appmb   #8
        appmb   #8
        appmb   #3
        appmb   #1
        pushmb  #8
        popmb   xzr, #8
        ret
