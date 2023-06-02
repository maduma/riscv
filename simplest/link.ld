ENTRY(_start)
SECTIONS
{
    . = 0x80000000;
    .init : { *(.init); }
    .rodata : { *(.rodata.*); }
    . = 0x81000000;
    _stack_start = .;
}
