.section .text.boot
.align 4
.global boot
.extern __stack_top

boot:
    la sp, __stack_top
    j kernel_main
