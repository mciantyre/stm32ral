//! stm32ral module for stm32g071

/// Number of priority bits implemented by the NVIC
pub const NVIC_PRIO_BITS: u8 = 2;

/// Interrupt-related magic for this device
pub mod interrupts;
pub use self::interrupts::Interrupt;
pub use self::interrupts::Interrupt as interrupt;

pub use super::instances::dbg_g070_g071_g07x_g081 as dbg;
pub use super::instances::flash;
pub use super::instances::iwdg_g070_g071_g07x_g081 as iwdg;
pub use super::instances::wwdg_g030_g031_g041_g070_g071_g07x_g081 as wwdg;
pub mod rcc;
pub use super::instances::pwr_g070_g071_g07x_g081 as pwr;
pub mod dma;
pub use super::instances::crc_g030_g031_g041_g070_g071_g07x_g081 as crc;
pub use super::instances::dmamux_g070_g071_g07x_g081 as dmamux;
pub use super::instances::exti_g070_g071 as exti;
pub use super::instances::gpio_g030_g031_g041_g070_g071_g07x_g081 as gpio;
pub mod tim15;
pub mod tim16;
pub mod tim17;
pub use super::instances::adc_g070_g071_g07x_g081 as adc;
pub use super::instances::comp;
pub use super::instances::dac_g071_g07x_g081 as dac;
pub use super::instances::hdmi_cec_g071_g07x_g081 as hdmi_cec;
pub use super::instances::i2c_g030_g031_g041_g070_g071_g07x_g081 as i2c;
pub use super::instances::lptim_g031_g041_g071_g07x_g081 as lptim;
pub use super::instances::lpuart_g071_g07x_g081 as lpuart;
pub use super::instances::nvic_g070_g071_g07x_g081 as nvic;
pub use super::instances::rtc_g070_g071_g07x_g081 as rtc;
pub use super::instances::spi_g070_g071_g07x_g081 as spi;
pub use super::instances::syscfg_vrefbuf;
pub use super::instances::tamp_g070_g071_g07x_g081 as tamp;
pub use super::instances::tim14_g030_g031_g041_g070_g071_g081 as tim14;
pub use super::instances::tim1_g070_g071_g081 as tim1;
pub use super::instances::tim2_g030_g031_g041_g071_g081 as tim2;
pub use super::instances::tim3_g030_g031_g041_g070_g071_g081 as tim3;
pub use super::instances::tim6_g070_g071_g07x_g081 as tim6;
pub use super::instances::tim7_g070_g071_g07x_g081 as tim7;
pub use super::instances::ucpd;
pub use super::instances::usart_g071_g07x_g081 as usart;
pub mod mpu;
pub use super::instances::scb;
pub use super::instances::stk;

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
#[allow(non_snake_case)]
pub struct Peripherals {
    pub IWDG: iwdg::Instance,
    pub WWDG: wwdg::Instance,
    pub FLASH: flash::Instance,
    pub DBG: dbg::Instance,
    pub RCC: rcc::Instance,
    pub PWR: pwr::Instance,
    pub DMA: dma::Instance,
    pub DMAMUX: dmamux::Instance,
    pub GPIOA: gpio::Instance,
    pub GPIOB: gpio::Instance,
    pub GPIOC: gpio::Instance,
    pub GPIOD: gpio::Instance,
    pub GPIOF: gpio::Instance,
    pub CRC: crc::Instance,
    pub EXTI: exti::Instance,
    pub TIM16: tim16::Instance,
    pub TIM17: tim17::Instance,
    pub TIM15: tim15::Instance,
    pub USART1: usart::Instance,
    pub USART2: usart::Instance,
    pub USART3: usart::Instance,
    pub USART4: usart::Instance,
    pub SPI1: spi::Instance,
    pub SPI2: spi::Instance,
    pub TIM1: tim1::Instance,
    pub ADC: adc::Instance,
    pub COMP: comp::Instance,
    pub SYSCFG_VREFBUF: syscfg_vrefbuf::Instance,
    pub TAMP: tamp::Instance,
    pub UCPD1: ucpd::Instance,
    pub UCPD2: ucpd::Instance,
    pub LPTIM1: lptim::Instance,
    pub LPTIM2: lptim::Instance,
    pub LPUART: lpuart::Instance,
    pub HDMI_CEC: hdmi_cec::Instance,
    pub DAC: dac::Instance,
    pub I2C1: i2c::Instance,
    pub I2C2: i2c::Instance,
    pub RTC: rtc::Instance,
    pub TIM14: tim14::Instance,
    pub TIM6: tim6::Instance,
    pub TIM7: tim7::Instance,
    pub TIM2: tim2::Instance,
    pub TIM3: tim3::Instance,
    pub NVIC: nvic::Instance,
    pub MPU: mpu::Instance,
    pub STK: stk::Instance,
    pub SCB: scb::Instance,
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
#[allow(non_snake_case)]
pub struct Peripherals {}

#[cfg(all(feature = "rtic", not(feature = "nosync")))]
impl Peripherals {
    pub unsafe fn steal() -> Self {
        Peripherals {
            IWDG: iwdg::IWDG::steal(),
            WWDG: wwdg::WWDG::steal(),
            FLASH: flash::FLASH::steal(),
            DBG: dbg::DBG::steal(),
            RCC: rcc::RCC::steal(),
            PWR: pwr::PWR::steal(),
            DMA: dma::DMA::steal(),
            DMAMUX: dmamux::DMAMUX::steal(),
            GPIOA: gpio::GPIOA::steal(),
            GPIOB: gpio::GPIOB::steal(),
            GPIOC: gpio::GPIOC::steal(),
            GPIOD: gpio::GPIOD::steal(),
            GPIOF: gpio::GPIOF::steal(),
            CRC: crc::CRC::steal(),
            EXTI: exti::EXTI::steal(),
            TIM16: tim16::TIM16::steal(),
            TIM17: tim17::TIM17::steal(),
            TIM15: tim15::TIM15::steal(),
            USART1: usart::USART1::steal(),
            USART2: usart::USART2::steal(),
            USART3: usart::USART3::steal(),
            USART4: usart::USART4::steal(),
            SPI1: spi::SPI1::steal(),
            SPI2: spi::SPI2::steal(),
            TIM1: tim1::TIM1::steal(),
            ADC: adc::ADC::steal(),
            COMP: comp::COMP::steal(),
            SYSCFG_VREFBUF: syscfg_vrefbuf::SYSCFG_VREFBUF::steal(),
            TAMP: tamp::TAMP::steal(),
            UCPD1: ucpd::UCPD1::steal(),
            UCPD2: ucpd::UCPD2::steal(),
            LPTIM1: lptim::LPTIM1::steal(),
            LPTIM2: lptim::LPTIM2::steal(),
            LPUART: lpuart::LPUART::steal(),
            HDMI_CEC: hdmi_cec::HDMI_CEC::steal(),
            DAC: dac::DAC::steal(),
            I2C1: i2c::I2C1::steal(),
            I2C2: i2c::I2C2::steal(),
            RTC: rtc::RTC::steal(),
            TIM14: tim14::TIM14::steal(),
            TIM6: tim6::TIM6::steal(),
            TIM7: tim7::TIM7::steal(),
            TIM2: tim2::TIM2::steal(),
            TIM3: tim3::TIM3::steal(),
            NVIC: nvic::NVIC::steal(),
            MPU: mpu::MPU::steal(),
            STK: stk::STK::steal(),
            SCB: scb::SCB::steal(),
        }
    }
}

#[cfg(all(feature = "rtic", feature = "nosync"))]
impl Peripherals {
    pub fn steal() -> Self {
        Peripherals {}
    }
}
