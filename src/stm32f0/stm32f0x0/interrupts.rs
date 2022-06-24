#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn RTC();
    fn FLASH();
    fn RCC();
    fn EXTI0_1();
    fn EXTI2_3();
    fn EXTI4_15();
    fn DMA1_CH1();
    fn DMA1_CH2_3();
    fn DMA1_CH4_5();
    fn ADC();
    fn TIM1_BRK_UP_TRG_COM();
    fn TIM1_CC();
    fn TIM3();
    fn TIM6();
    fn TIM14();
    fn TIM15();
    fn TIM16();
    fn TIM17();
    fn I2C1();
    fn I2C2();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3_4_5_6();
    fn USB();
}

#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}

#[cfg(feature = "rt")]
#[doc(hidden)]
#[cfg_attr(target_arch = "arm", link_section = ".vector_table.interrupts")]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector { _handler: RTC },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0_1 },
    Vector { _handler: EXTI2_3 },
    Vector { _handler: EXTI4_15 },
    Vector { _reserved: 0 },
    Vector { _handler: DMA1_CH1 },
    Vector {
        _handler: DMA1_CH2_3,
    },
    Vector {
        _handler: DMA1_CH4_5,
    },
    Vector { _handler: ADC },
    Vector {
        _handler: TIM1_BRK_UP_TRG_COM,
    },
    Vector { _handler: TIM1_CC },
    Vector { _reserved: 0 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM6 },
    Vector { _reserved: 0 },
    Vector { _handler: TIM14 },
    Vector { _handler: TIM15 },
    Vector { _handler: TIM16 },
    Vector { _handler: TIM17 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector {
        _handler: USART3_4_5_6,
    },
    Vector { _reserved: 0 },
    Vector { _handler: USB },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD and VDDIO2 supply comparator interrupt
    PVD = 1,
    /// 2: RTC interrupts
    RTC = 2,
    /// 3: Flash global interrupt
    FLASH = 3,
    /// 4: RCC global interruptr
    RCC = 4,
    /// 5: EXTI Line\[1:0\] interrupts
    EXTI0_1 = 5,
    /// 6: EXTI Line\[3:2\] interrupts
    EXTI2_3 = 6,
    /// 7: EXTI Line15 and EXTI4 interrupts
    EXTI4_15 = 7,
    /// 9: DMA1 channel 1 interrupt
    DMA1_CH1 = 9,
    /// 10: DMA1 channel 2 and 3 interrupt
    DMA1_CH2_3 = 10,
    /// 11: DMA1 channel 4 and 5 interrupt
    DMA1_CH4_5 = 11,
    /// 12: ADC interrupt
    ADC = 12,
    /// 13: TIM1 break, update, trigger and commutation interrupt
    TIM1_BRK_UP_TRG_COM = 13,
    /// 14: TIM1 Capture Compare interrupt
    TIM1_CC = 14,
    /// 16: TIM3 global interrupt
    TIM3 = 16,
    /// 17: TIM6 global interrupt
    TIM6 = 17,
    /// 19: TIM14 global interrupt
    TIM14 = 19,
    /// 20: TIM15 global interrupt
    TIM15 = 20,
    /// 21: TIM16 global interrupt
    TIM16 = 21,
    /// 22: TIM17 global interrupt
    TIM17 = 22,
    /// 23: I2C1 global interrupt
    I2C1 = 23,
    /// 24: I2C2 global interrupt
    I2C2 = 24,
    /// 25: SPI1_global_interrupt
    SPI1 = 25,
    /// 26: SPI2 global interrupt
    SPI2 = 26,
    /// 27: USART1 global interrupt
    USART1 = 27,
    /// 28: USART2 global interrupt
    USART2 = 28,
    /// 29: USART3, USART4, USART5, USART6 global interrupt
    USART3_4_5_6 = 29,
    /// 31: USB global interrupt
    USB = 31,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
