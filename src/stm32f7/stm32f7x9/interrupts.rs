#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_Stream0();
    fn DMA1_Stream1();
    fn DMA1_Stream2();
    fn DMA1_Stream3();
    fn DMA1_Stream4();
    fn DMA1_Stream5();
    fn DMA1_Stream6();
    fn ADC();
    fn CAN1_TX();
    fn CAN1_RX0();
    fn CAN1_RX1();
    fn CAN1_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM9();
    fn TIM1_UP_TIM10();
    fn TIM1_TRG_COM_TIM11();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV();
    fn I2C1_ER();
    fn I2C2_EV();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1();
    fn USART2();
    fn USART3();
    fn EXTI15_10();
    fn RTC_ALARM();
    fn OTG_FS_WKUP();
    fn TIM8_BRK_TIM12();
    fn TIM8_UP_TIM13();
    fn TIM8_TRG_COM_TIM14();
    fn TIM8_CC();
    fn DMA1_Stream7();
    fn FMC();
    fn SDMMC1();
    fn TIM5();
    fn SPI3();
    fn UART4();
    fn UART5();
    fn TIM6_DAC();
    fn TIM7();
    fn DMA2_Stream0();
    fn DMA2_Stream1();
    fn DMA2_Stream2();
    fn DMA2_Stream3();
    fn DMA2_Stream4();
    fn ETH();
    fn ETH_WKUP();
    fn CAN2_TX();
    fn CAN2_RX0();
    fn CAN2_RX1();
    fn CAN2_SCE();
    fn OTG_FS();
    fn DMA2_Stream5();
    fn DMA2_Stream6();
    fn DMA2_Stream7();
    fn USART6();
    fn I2C3_EV();
    fn I2C3_ER();
    fn OTG_HS_EP1_OUT();
    fn OTG_HS_EP1_IN();
    fn OTG_HS_WKUP();
    fn OTG_HS();
    fn DCMI();
    fn CRYP();
    fn HASH_RNG();
    fn FPU();
    fn UART7();
    fn UART8();
    fn SPI4();
    fn SPI5();
    fn SPI6();
    fn SAI1();
    fn LTDC();
    fn LCD_TFT_1();
    fn DMA2D();
    fn SAI2();
    fn QuadSPI();
    fn LPTimer1();
    fn HDMI_CEC();
    fn I2C4_EV();
    fn I2C4_ER();
    fn SPDIFRX();
    fn DSIHOST();
    fn DFSDM1_FLT0();
    fn DFSDM1_FLT1();
    fn DFSDM1_FLT2();
    fn DFSDM1_FLT3();
    fn SDMMC2();
    fn CAN3_TX();
    fn CAN3_RX0();
    fn CAN3_RX1();
    fn CAN3_SCE();
    fn JPEG();
    fn MDIOS();
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
pub static __INTERRUPTS: [Vector; 110] = [
    Vector { _handler: WWDG },
    Vector { _handler: PVD },
    Vector {
        _handler: TAMP_STAMP,
    },
    Vector { _handler: RTC_WKUP },
    Vector { _handler: FLASH },
    Vector { _handler: RCC },
    Vector { _handler: EXTI0 },
    Vector { _handler: EXTI1 },
    Vector { _handler: EXTI2 },
    Vector { _handler: EXTI3 },
    Vector { _handler: EXTI4 },
    Vector {
        _handler: DMA1_Stream0,
    },
    Vector {
        _handler: DMA1_Stream1,
    },
    Vector {
        _handler: DMA1_Stream2,
    },
    Vector {
        _handler: DMA1_Stream3,
    },
    Vector {
        _handler: DMA1_Stream4,
    },
    Vector {
        _handler: DMA1_Stream5,
    },
    Vector {
        _handler: DMA1_Stream6,
    },
    Vector { _handler: ADC },
    Vector { _handler: CAN1_TX },
    Vector { _handler: CAN1_RX0 },
    Vector { _handler: CAN1_RX1 },
    Vector { _handler: CAN1_SCE },
    Vector { _handler: EXTI9_5 },
    Vector {
        _handler: TIM1_BRK_TIM9,
    },
    Vector {
        _handler: TIM1_UP_TIM10,
    },
    Vector {
        _handler: TIM1_TRG_COM_TIM11,
    },
    Vector { _handler: TIM1_CC },
    Vector { _handler: TIM2 },
    Vector { _handler: TIM3 },
    Vector { _handler: TIM4 },
    Vector { _handler: I2C1_EV },
    Vector { _handler: I2C1_ER },
    Vector { _handler: I2C2_EV },
    Vector { _handler: I2C2_ER },
    Vector { _handler: SPI1 },
    Vector { _handler: SPI2 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: USART3 },
    Vector {
        _handler: EXTI15_10,
    },
    Vector {
        _handler: RTC_ALARM,
    },
    Vector {
        _handler: OTG_FS_WKUP,
    },
    Vector {
        _handler: TIM8_BRK_TIM12,
    },
    Vector {
        _handler: TIM8_UP_TIM13,
    },
    Vector {
        _handler: TIM8_TRG_COM_TIM14,
    },
    Vector { _handler: TIM8_CC },
    Vector {
        _handler: DMA1_Stream7,
    },
    Vector { _handler: FMC },
    Vector { _handler: SDMMC1 },
    Vector { _handler: TIM5 },
    Vector { _handler: SPI3 },
    Vector { _handler: UART4 },
    Vector { _handler: UART5 },
    Vector { _handler: TIM6_DAC },
    Vector { _handler: TIM7 },
    Vector {
        _handler: DMA2_Stream0,
    },
    Vector {
        _handler: DMA2_Stream1,
    },
    Vector {
        _handler: DMA2_Stream2,
    },
    Vector {
        _handler: DMA2_Stream3,
    },
    Vector {
        _handler: DMA2_Stream4,
    },
    Vector { _handler: ETH },
    Vector { _handler: ETH_WKUP },
    Vector { _handler: CAN2_TX },
    Vector { _handler: CAN2_RX0 },
    Vector { _handler: CAN2_RX1 },
    Vector { _handler: CAN2_SCE },
    Vector { _handler: OTG_FS },
    Vector {
        _handler: DMA2_Stream5,
    },
    Vector {
        _handler: DMA2_Stream6,
    },
    Vector {
        _handler: DMA2_Stream7,
    },
    Vector { _handler: USART6 },
    Vector { _handler: I2C3_EV },
    Vector { _handler: I2C3_ER },
    Vector {
        _handler: OTG_HS_EP1_OUT,
    },
    Vector {
        _handler: OTG_HS_EP1_IN,
    },
    Vector {
        _handler: OTG_HS_WKUP,
    },
    Vector { _handler: OTG_HS },
    Vector { _handler: DCMI },
    Vector { _handler: CRYP },
    Vector { _handler: HASH_RNG },
    Vector { _handler: FPU },
    Vector { _handler: UART7 },
    Vector { _handler: UART8 },
    Vector { _handler: SPI4 },
    Vector { _handler: SPI5 },
    Vector { _handler: SPI6 },
    Vector { _handler: SAI1 },
    Vector { _handler: LTDC },
    Vector {
        _handler: LCD_TFT_1,
    },
    Vector { _handler: DMA2D },
    Vector { _handler: SAI2 },
    Vector { _handler: QuadSPI },
    Vector { _handler: LPTimer1 },
    Vector { _handler: HDMI_CEC },
    Vector { _handler: I2C4_EV },
    Vector { _handler: I2C4_ER },
    Vector { _handler: SPDIFRX },
    Vector { _handler: DSIHOST },
    Vector {
        _handler: DFSDM1_FLT0,
    },
    Vector {
        _handler: DFSDM1_FLT1,
    },
    Vector {
        _handler: DFSDM1_FLT2,
    },
    Vector {
        _handler: DFSDM1_FLT3,
    },
    Vector { _handler: SDMMC2 },
    Vector { _handler: CAN3_TX },
    Vector { _handler: CAN3_RX0 },
    Vector { _handler: CAN3_RX1 },
    Vector { _handler: CAN3_SCE },
    Vector { _handler: JPEG },
    Vector { _handler: MDIOS },
];

/// Available interrupts for this device
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Interrupt {
    /// 0: Window Watchdog interrupt
    WWDG = 0,
    /// 1: PVD through EXTI line detection interrupt
    PVD = 1,
    /// 2: Tamper and TimeStamp interrupts through the EXTI line
    TAMP_STAMP = 2,
    /// 3: RTC Tamper or TimeStamp /CSS on LSE through EXTI line 19 interrupts
    RTC_WKUP = 3,
    /// 4: Flash global interrupt
    FLASH = 4,
    /// 5: RCC global interrupt
    RCC = 5,
    /// 6: EXTI Line0 interrupt
    EXTI0 = 6,
    /// 7: EXTI Line1 interrupt
    EXTI1 = 7,
    /// 8: EXTI Line2 interrupt
    EXTI2 = 8,
    /// 9: EXTI Line3 interrupt
    EXTI3 = 9,
    /// 10: EXTI Line4 interrupt
    EXTI4 = 10,
    /// 11: DMA1 Stream0 global interrupt
    DMA1_Stream0 = 11,
    /// 12: DMA1 Stream1 global interrupt
    DMA1_Stream1 = 12,
    /// 13: DMA1 Stream2 global interrupt
    DMA1_Stream2 = 13,
    /// 14: DMA1 Stream3 global interrupt
    DMA1_Stream3 = 14,
    /// 15: DMA1 Stream4 global interrupt
    DMA1_Stream4 = 15,
    /// 16: DMA1 Stream5 global interrupt
    DMA1_Stream5 = 16,
    /// 17: DMA1 Stream6 global interrupt
    DMA1_Stream6 = 17,
    /// 18: ADC1 global interrupt
    ADC = 18,
    /// 19: CAN1 TX interrupts
    CAN1_TX = 19,
    /// 20: CAN1 RX0 interrupts
    CAN1_RX0 = 20,
    /// 21: CAN1 RX1 interrupts
    CAN1_RX1 = 21,
    /// 22: CAN1 SCE interrupt
    CAN1_SCE = 22,
    /// 23: EXTI Line\[9:5\] interrupts
    EXTI9_5 = 23,
    /// 24: TIM1 Break interrupt and TIM9 global interrupt
    TIM1_BRK_TIM9 = 24,
    /// 25: TIM1_UP_TIM10
    TIM1_UP_TIM10 = 25,
    /// 26: TIM1 Trigger and Commutation interrupts and TIM11 global interrupt
    TIM1_TRG_COM_TIM11 = 26,
    /// 27: TIM1_CC
    TIM1_CC = 27,
    /// 28: TIM2 global interrupt
    TIM2 = 28,
    /// 29: TIM3 global interrupt
    TIM3 = 29,
    /// 30: TIM4 global interrupt
    TIM4 = 30,
    /// 31: I2C1 event interrupt
    I2C1_EV = 31,
    /// 32: I2C1 error interrupt
    I2C1_ER = 32,
    /// 33: I2C2 event interrupt
    I2C2_EV = 33,
    /// 34: I2C2 error interrupt
    I2C2_ER = 34,
    /// 35: SPI1 global interrupt
    SPI1 = 35,
    /// 36: SPI2 global interrupt
    SPI2 = 36,
    /// 37: USART1 global interrupt
    USART1 = 37,
    /// 38: USART2 global interrupt
    USART2 = 38,
    /// 39: USART3 global interrupt
    USART3 = 39,
    /// 40: EXTI Line\[15:10\] interrupts
    EXTI15_10 = 40,
    /// 41: RTC alarms through EXTI line 18 interrupts
    RTC_ALARM = 41,
    /// 42: USB On-The-Go FS Wakeup through EXTI line interrupt
    OTG_FS_WKUP = 42,
    /// 43: TIM8 Break interrupt and TIM12 global interrupt
    TIM8_BRK_TIM12 = 43,
    /// 44: TIM8 Update interrupt and TIM13 global interrupt
    TIM8_UP_TIM13 = 44,
    /// 45: TIM8 Trigger and Commutation interrupts and TIM14 global interrupt
    TIM8_TRG_COM_TIM14 = 45,
    /// 46: TIM8 Capture Compare interrupt
    TIM8_CC = 46,
    /// 47: DMA1 Stream7 global interrupt
    DMA1_Stream7 = 47,
    /// 48: FMC global interrupt
    FMC = 48,
    /// 49: SDMMC1 global interrupt
    SDMMC1 = 49,
    /// 50: TIM5 global interrupt
    TIM5 = 50,
    /// 51: SPI3 global interrupt
    SPI3 = 51,
    /// 52: UART4 global interrupt
    UART4 = 52,
    /// 53: UART5 global interrupt
    UART5 = 53,
    /// 54: TIM6 global interrupt, DAC1 and DAC2 underrun error interrupt
    TIM6_DAC = 54,
    /// 55: TIM7 global interrupt
    TIM7 = 55,
    /// 56: DMA2 Stream0 global interrupt
    DMA2_Stream0 = 56,
    /// 57: DMA2 Stream1 global interrupt
    DMA2_Stream1 = 57,
    /// 58: DMA2 Stream2 global interrupt
    DMA2_Stream2 = 58,
    /// 59: DMA2 Stream3 global interrupt
    DMA2_Stream3 = 59,
    /// 60: DMA2 Stream4 global interrupt
    DMA2_Stream4 = 60,
    /// 61: Ethernet global interrupt
    ETH = 61,
    /// 62: Ethernet Wakeup through EXTI line
    ETH_WKUP = 62,
    /// 63: CAN2 TX interrupts
    CAN2_TX = 63,
    /// 64: CAN2 RX0 interrupts
    CAN2_RX0 = 64,
    /// 65: CAN2 RX1 interrupts
    CAN2_RX1 = 65,
    /// 66: CAN2 SCE interrupt
    CAN2_SCE = 66,
    /// 67: USB On The Go FS global interrupt
    OTG_FS = 67,
    /// 68: DMA2 Stream5 global interrupt
    DMA2_Stream5 = 68,
    /// 69: DMA2 Stream6 global interrupt
    DMA2_Stream6 = 69,
    /// 70: DMA2 Stream7 global interrupt
    DMA2_Stream7 = 70,
    /// 71: USART6 global interrupt
    USART6 = 71,
    /// 72: I2C3 event interrupt
    I2C3_EV = 72,
    /// 73: I2C3 error interrupt
    I2C3_ER = 73,
    /// 74: USB On The Go HS End Point 1 Out global interrupt
    OTG_HS_EP1_OUT = 74,
    /// 75: USB On The Go HS End Point 1 In global interrupt
    OTG_HS_EP1_IN = 75,
    /// 76: USB On The Go HS Wakeup through EXTI interrupt
    OTG_HS_WKUP = 76,
    /// 77: USB On The Go HS global interrupt
    OTG_HS = 77,
    /// 78: DCMI global interrupt
    DCMI = 78,
    /// 79: CRYP
    CRYP = 79,
    /// 80: Hash and Rng global interrupt
    HASH_RNG = 80,
    /// 81: Floating point unit interrupt
    FPU = 81,
    /// 82: UART7 global interrupt
    UART7 = 82,
    /// 83: UART 8 global interrupt
    UART8 = 83,
    /// 84: SPI 4 global interrupt
    SPI4 = 84,
    /// 85: SPI 5 global interrupt
    SPI5 = 85,
    /// 86: SPI 6 global interrupt
    SPI6 = 86,
    /// 87: SAI1 global interrupt
    SAI1 = 87,
    /// 88: LTDC global interrupt
    LTDC = 88,
    /// 89: LCD-TFT global error interrupt
    LCD_TFT_1 = 89,
    /// 90: DMA2D global interrupt
    DMA2D = 90,
    /// 91: SAI2 global interrupt
    SAI2 = 91,
    /// 92: QuadSPI global interrupt
    QuadSPI = 92,
    /// 93: LP Timer1 global interrupt
    LPTimer1 = 93,
    /// 94: HDMI-CEC global interrupt
    HDMI_CEC = 94,
    /// 95: I2C4 event interrupt
    I2C4_EV = 95,
    /// 96: I2C4 error interrupt
    I2C4_ER = 96,
    /// 97: SPDIFRX global interrupt
    SPDIFRX = 97,
    /// 98: DSI host global interrupt
    DSIHOST = 98,
    /// 99: DFSDM1 Filter 0 global interrupt
    DFSDM1_FLT0 = 99,
    /// 100: DFSDM1 Filter 1 global interrupt
    DFSDM1_FLT1 = 100,
    /// 101: DFSDM1 Filter 2 global interrupt
    DFSDM1_FLT2 = 101,
    /// 102: DFSDM1 Filter 3 global interrupt
    DFSDM1_FLT3 = 102,
    /// 103: SDMMC2 global interrupt
    SDMMC2 = 103,
    /// 104: CAN3 TX interrupt
    CAN3_TX = 104,
    /// 105: CAN3 RX0 interrupt
    CAN3_RX0 = 105,
    /// 106: CAN3 RX1 interrupt
    CAN3_RX1 = 106,
    /// 107: CAN3 SCE interrupt
    CAN3_SCE = 107,
    /// 108: JPEG global interrupt
    JPEG = 108,
    /// 109: MDIO slave global interrupt
    MDIOS = 109,
}
unsafe impl external_cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
