ENTRY(_start)

MEMORY
{
    RAM : ORIGIN = 0x80000000, LENGTH = 0x7ffffff 
}

SECTIONS
{
    .init :
    {
         KEEP(*(.init))
    }
    > RAM
    . = ALIGN(4);

    _stack_start = . + 0x800;
}
