MEMORY
{
  RAM : ORIGIN = 0x80000000, LENGTH = 8K
}

SECTIONS
{
  .init :
  {
    *(.init .init.*);
  } > RAM

  stack_start = ORIGIN(RAM) + LENGTH(RAM);
}
