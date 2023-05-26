/****************************************************************************
 *
 *   Copyright (C) 2022 bsvtgc@gmail.com. All rights reserved.
 *   Author: Vincent <bsvtgc@gmail.com>
 *
 ****************************************************************************/


/* Comments here */

OUTPUT_ARCH("riscv")

/* Entry point */

ENTRY(_start)


MEMORY
{
    ram : ORIGIN = 0x80000000, LENGTH = 0x7ffffff 
}

SECTIONS
{
    .init :
    {
         KEEP(*(.init))
    }
    > ram
    . = ALIGN(4);

    .code :
    {
         *(.code* .code.*)
         *(.text* .text.*)
    }
    > ram
    . = ALIGN(4);

    .rodata :
    {
        *(.rodata .rodata.*);
    } > ram
    . = ALIGN(4);

    .bss :
    {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > ram
    . = ALIGN(4);

    .data :
    {
        _sdata = .;
        *(.data .data.*)
        _edata = .;
    }
    > ram
    . = ALIGN(4);

    _sidata = LOADADDR(.data);

    /* Set Stack after code & data */
    _stack_start = 0x80010000;
}

