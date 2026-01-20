/* LINKER SCRIPT FILE */

ENTRY(reset_handler)
/* DEFINE MCU MEMORY */
MEMORY
{
    FLASH (rx)      : ORIGIN = 0x08000000, LENGTH = 64K
    RAM (rwx)       : ORIGIN = 0x20000000, LENGTH = 12K
    CCRAM (rwx)     : ORIGIN = 0x10000000, LENGTH = 4K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400;
_min_heap_size = 0x400;

/* DEFINE MEMORY SECTIONS */
SECTIONS
{
    /* ALL EXECUTABLE CODE */
    .text :
    {
        . = ORIGIN(FLASH);
        LONG(_start_of_stack);
        KEEP(*(.isr_vector))
        *(.text)
        *(.text*)
        . = ALIGN(4);
    } > FLASH

    /* READ ONLY (CONSTANTS, LITERALS) */
    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata*)
        . = ALIGN(4);
    } > FLASH

    /* INITIALIZED DATA (INITIALLY OCCUPIES FLASH THEN COPIED TO RAM MEMORY AT RUNTIME) */
    .data :
    {
        _sidata = LOADADDR(.data);
        . = ALIGN(4);
        _sdata = .;
        *(.data)
        *(.data*)
        . = ALIGN(4);
        _edata = .;
    } > RAM AT> FLASH

    /* UNITIALIZED DATA () */
    .bss :
    {
        . = ALIGN(4);
        _sbss = .;
        *(.bss)
        *(.bss*)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .ram_usage_check :
    {
        . = ALIGN(8);
        . = . + _min_stack_size;
        . = . + _min_heap_size;
        . = ALIGN(8);
    } > RAM
}

PROVIDE(mem_manage_handler = default_handler);
PROVIDE(busfault_handler = default_handler);
PROVIDE(usagefault_handler = default_handler);
PROVIDE(svcall_handler = default_handler);
PROVIDE(pendsv_handler = default_handler);
PROVIDE(systick_handler = default_handler);
PROVIDE(wwdg_irqhandler = default_handler);
PROVIDE(pvd_irqhandler = default_handler);
PROVIDE(tamp_stamp_irqhandler = default_handler);
PROVIDE(rtc_wkup_irqhandler = default_handler);
PROVIDE(flash_irqhandler = default_handler);
PROVIDE(rcc_irqhandler = default_handler);
PROVIDE(exti0_irqhandler = default_handler);
PROVIDE(exti1_irqhandler = default_handler);
PROVIDE(exti2_ts_irqhandler = default_handler);
PROVIDE(exti3_irqhandler = default_handler);
PROVIDE(exti4_irqhandler = default_handler);
PROVIDE(dma1_channel1_irqhandler = default_handler);
PROVIDE(dma1_channel2_irqhandler = default_handler);
PROVIDE(dma1_channel3_irqhandler = default_handler);
PROVIDE(dma1_channel4_irqhandler = default_handler);
PROVIDE(dma1_channel5_irqhandler = default_handler);
PROVIDE(dma1_channel6_irqhandler = default_handler);
PROVIDE(dma1_channel7_irqhandler = default_handler);
PROVIDE(adc1_2_irqhandler = default_handler);
PROVIDE(can1_tx_irqhandler = default_handler);
PROVIDE(can1_rx0_irqhandler = default_handler);
PROVIDE(can1_rx1_irqhandler = default_handler);
PROVIDE(can1_sce_irqhandler = default_handler);
PROVIDE(exti9_5_irqhandler = default_handler);
PROVIDE(tim1_brk_tim15_irqhandler = default_handler);
PROVIDE(tim1_up_tim16_irqhandler = default_handler);
PROVIDE(tim1_trg_com_tim17_irqhandler = default_handler);
PROVIDE(tim1_cc_irqhandler = default_handler);
PROVIDE(tim2_irqhandler = default_handler);
PROVIDE(tim3_irqhandler = default_handler);
PROVIDE(i2c1_ev_irqhandler = default_handler);
PROVIDE(i2c1_er_irqhandler = default_handler);
PROVIDE(spi1_irqhandler = default_handler);
PROVIDE(usart1_irqhandler = default_handler);
PROVIDE(usart2_irqhandler = default_handler);
PROVIDE(usart3_irqhandler = default_handler);
PROVIDE(exti15_10_irqhandler = default_handler);
PROVIDE(rtc_alarm_irqhandler = default_handler);
PROVIDE(tim6_dac_irqhandler = default_handler);
PROVIDE(tim7_dac_irqhandler = default_handler);
PROVIDE(comp2_irqhandler = default_handler);
PROVIDE(comp4_6_irqhandler = default_handler);
PROVIDE(fpu_irqhandler = default_handler); 