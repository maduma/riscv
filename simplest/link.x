MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 4K
}

ENTRY(_start)
SECTIONS
{
  .init :
  {
    *(.init .init.*);
  } > RAM

   _stack_start = ORIGIN(RAM) + LENGTH(RAM);
}
