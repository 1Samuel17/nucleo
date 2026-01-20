
unsafe extern "C" {
    fn mem_manage_handler();
    fn busfault_handler();
    fn usagefault_handler();
    fn svcall_handler();
    fn pendsv_handler();
    fn systick_handler();
    fn wwdg_irqhandler();
    fn pvd_irqhandler();
    fn tamp_stamp_irqhandler();
    fn rtc_wkup_irqhandler();
    fn flash_irqhandler();
    fn rcc_irqhandler();
    fn exti0_irqhandler();
    fn exti1_irqhandler();
    fn exti2_ts_irqhandler();
    fn exti3_irqhandler();
    fn exti4_irqhandler();
    fn dma1_channel1_irqhandler();
    fn dma1_channel2_irqhandler();
    fn dma1_channel3_irqhandler();
    fn dma1_channel4_irqhandler();
    fn dma1_channel5_irqhandler();
    fn dma1_channel6_irqhandler();
    fn dma1_channel7_irqhandler();
    fn adc1_2_irqhandler();
    fn can1_tx_irqhandler();
    fn can1_rx0_irqhandler();
    fn can1_rx1_irqhandler();
    fn can1_sce_irqhandler();
    fn exti9_5_irqhandler();
    fn tim1_brk_tim15_irqhandler();
    fn tim1_up_tim16_irqhandler();
    fn tim1_trg_com_tim17_irqhandler();
    fn tim1_cc_irqhandler();
    fn tim2_irqhandler();
    fn tim3_irqhandler();
    fn i2c1_ev_irqhandler();
    fn i2c1_er_irqhandler();
    fn spi1_irqhandler();
    fn usart1_irqhandler();
    fn usart2_irqhandler();
    fn usart3_irqhandler();
    fn exti15_10_irqhandler();
    fn rtc_alarm_irqhandler();
    fn tim6_dac_irqhandler();
    fn tim7_dac_irqhandler();
    fn comp2_irqhandler();
    fn comp4_6_irqhandler();
    fn fpu_irqhandler();       
}

#[used]
#[unsafe(link_section = ".isr_vector")]
static VECTOR_TABLE: [Option<unsafe extern "C" fn()>; 97] = [
    Some(reset_handler),
    Some(nmi_handler),
    Some(hardfault_handler),
    Some(mem_manage_handler),
    Some(busfault_handler),
    Some(usagefault_handler),
    None,
    None,
    None,
    None,
    Some(svcall_handler),
    None,
    None,
    Some(pendsv_handler),
    Some(systick_handler),
    Some(wwdg_irqhandler),
    Some(pvd_irqhandler),
    Some(tamp_stamp_irqhandler),
    Some(rtc_wkup_irqhandler),
    Some(flash_irqhandler),
    Some(rcc_irqhandler),
    Some(exti0_irqhandler),
    Some(exti1_irqhandler),
    Some(exti2_ts_irqhandler),
    Some(exti3_irqhandler),
    Some(exti4_irqhandler),
    Some(dma1_channel1_irqhandler),
    Some(dma1_channel2_irqhandler),
    Some(dma1_channel3_irqhandler),
    Some(dma1_channel4_irqhandler),
    Some(dma1_channel5_irqhandler),
    Some(dma1_channel6_irqhandler),
    Some(dma1_channel7_irqhandler),
    Some(adc1_2_irqhandler),
    Some(can1_tx_irqhandler),
    Some(can1_rx0_irqhandler),
    Some(can1_rx1_irqhandler),
    Some(can1_sce_irqhandler),
    Some(exti9_5_irqhandler),
    Some(tim1_brk_tim15_irqhandler),
    Some(tim1_up_tim16_irqhandler),
    Some(tim1_trg_com_tim17_irqhandler),
    Some(tim1_cc_irqhandler),
    Some(tim2_irqhandler),
    Some(tim3_irqhandler),
    None,
    Some(i2c1_ev_irqhandler),
    Some(i2c1_er_irqhandler),
    None,
    None,
    Some(spi1_irqhandler),
    None,
    Some(usart1_irqhandler),
    Some(usart2_irqhandler),
    Some(usart3_irqhandler),
    Some(exti15_10_irqhandler),
    Some(rtc_alarm_irqhandler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(tim6_dac_irqhandler),
    Some(tim7_dac_irqhandler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(comp2_irqhandler),
    Some(comp4_6_irqhandler),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(fpu_irqhandler),       
];

#[unsafe(no_mangle)]
extern "C" fn hardfault_handler() { loop {} }

#[unsafe(no_mangle)]
extern "C" fn nmi_handler() { loop {} }

#[unsafe(no_mangle)]
extern "C" fn default_handler() { loop {} }


unsafe extern "C" {
    static _sidata: u32;
    static mut _sdata: u32;
    static mut _edata: u32;
    static mut _sbss: u32;
    static mut _ebss: u32;
}

#[unsafe(no_mangle)]
extern "C" fn reset_handler() { 

    unsafe {

        let mut source_is_flash = &_sidata  as *const u32;
        let mut  dest_is_ram = &raw mut _sdata as *mut u32;
        let data_end_in_ran = &raw mut _edata as *mut u32;

        while dest_is_ram < data_end_in_ran {
            *dest_is_ram = *source_is_flash;
            dest_is_ram = dest_is_ram.add(1);
            source_is_flash = source_is_flash.add(1);
        }

        let mut bss = &raw mut _sbss as *mut u32;
        let bss_end = &raw mut _ebss as *mut u32;

        while bss < bss_end {
            *bss = 0;
            bss = bss.add(1);
        
        }   
    }
    
    crate::main(); 
}