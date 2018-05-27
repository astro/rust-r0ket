MEMORY
{
    sram(rwx): ORIGIN = 0x10002000 - 2560, LENGTH = 2560
}

sram_top = ORIGIN(sram) + LENGTH(sram);
TheTable = 0x00000124;
ENTRY(start);

SECTIONS
{
  .text ORIGIN(sram) : ALIGN(4)
  {
    _stext = ABSOLUTE(.);
    *(.startup);
    *(.text .text.*);
    _etext = ABSOLUTE(.);
  } > sram
    
  .rodata :
  {
    . = ALIGN(4);
    *(.rodata*);
  } > sram


  .data :
  {
    . = ALIGN(4);
    __sdata = ABSOLUTE(.);
    *(.data .data.*)
    . = ALIGN(4);
    __edata = (.);
  } > sram
  .bss :
  {
    . = ALIGN(4);
    /* force zero initialized data to be present*/
    __sbss = ABSOLUTE(.);
    *(.bss .bss.*)
    . = ALIGN(4);
    __ebss = ABSOLUTE(.);
  } > sram

  end = .;

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Unused exception related info that only wastes space */
    *(.ARM.exidx.*);
  }
}
