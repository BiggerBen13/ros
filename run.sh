
#!/bin/bash

QEMU=qemu-system-riscv32

$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
    -kernel "$1" 
