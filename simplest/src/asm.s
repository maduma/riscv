.section .init

_start:
    la a0, _trap_handler
    csrw mtvec, a0
    la sp, _stack_start
    mv s0, sp
    la a0, _start_rust
    jr a0

_trap_handler:
    j _trap_handler
