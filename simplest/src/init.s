.section .init

    la a0, trap_handler
    csrw mtvec, a0
    la sp, stack_start
    mv s0, sp
    la a0, start_rust
    jr a0

trap_handler:
    j trap_handler
