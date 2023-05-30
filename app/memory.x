MEMORY
{
  ROM : ORIGIN = 0x80000000, LENGTH = 1M
  RAM : ORIGIN = 0x80100000, LENGTH = 1M
}

REGION_ALIAS("REGION_TEXT", ROM);
REGION_ALIAS("REGION_RODATA", ROM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);
