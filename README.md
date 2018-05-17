# alb1
is an aarch64, raspberry pi only operating system that was build completely from scratch. it essentially consists of three parts: the bootloader, the kernel and the loader. the bootloader and the loader are two parts of a more generic booting mechanism which the kernel is based on. they are both easy to setup and provide an interface for the kernel to output text and receive user input. the kernel itself is specifically built for my setup and requirements, so it probably won't be of much use for anyone else.

# bootloader
the bootloader is written in assembly only and its only purpose is to setup a stack and load the kernel in form of a binary from the local network by using one of three modes. first there is the mini uart user mode, which the bootloader will fall into when there is no serial loader found. it provides a user interface in form of a basic command line tool. the second mode is the mini uart loader mode, in which the bootloader automatically loads the kernel over serial connection, without requiring user input. and finally there is an ethernet loader mode, which will load the binary from any machine on the network that has an instance of the loader running.

# kernel

# loader
the loader is a basic tool for specifying one or multiple binary files that are being sent to the bootloader on request, though it also supports instant loading as part of the bootloader user mode. the loader can be configured via command line arguments as well as a configuration file. there is also a feature for sending user input to the kernel.

# usage

# setup
