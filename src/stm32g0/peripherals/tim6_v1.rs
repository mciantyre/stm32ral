#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Basic timers
//!
//! Used by: stm32g050, stm32g051, stm32g061, stm32g0b0, stm32g0b1, stm32g0c1

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register 1
pub mod CR1 {

    /// Counter enable Note: Gated mode can work only if the CEN bit has been previously set by software. However trigger mode can set the CEN bit automatically by hardware. CEN is cleared automatically in one-pulse mode, when an update event occurs.
    pub mod CEN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counter disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Counter enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Update disable This bit is set and cleared by software to enable/disable UEV event generation. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller Buffered registers are then loaded with their preload values.
    pub mod UDIS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: UEV enabled. The Update (UEV) event is generated by one of the following events:
            pub const B_0x0: u32 = 0b0;

            /// 0b1: UEV disabled. The Update event is not generated, shadow registers keep their value (ARR, PSC). However the counter and the prescaler are reinitialized if the UG bit is set or if a hardware reset is received from the slave mode controller.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Update request source This bit is set and cleared by software to select the UEV event sources. Counter overflow/underflow Setting the UG bit Update generation through the slave mode controller
    pub mod URS {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Any of the following events generates an update interrupt or DMA request if enabled. These events can be:
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Only counter overflow/underflow generates an update interrupt or DMA request if enabled.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// One-pulse mode
    pub mod OPM {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Counter is not stopped at update event
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Counter stops counting at the next update event (clearing the CEN bit).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Auto-reload preload enable
    pub mod ARPE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: TIMx_ARR register is not buffered.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: TIMx_ARR register is buffered.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// UIF status bit remapping
    pub mod UIFREMAP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No remapping. UIF status bit is not copied to TIMx_CNT register bit 31.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Remapping enabled. UIF status bit is copied to TIMx_CNT register bit 31.
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// control register 2
pub mod CR2 {

    /// Master mode selection These bits are used to select the information to be sent in master mode to slave timers for synchronization (TRGO). The combination is as follows: When the Counter Enable signal is controlled by the trigger input, there is a delay on TRGO, except if the master/slave mode is selected (see the MSM bit description in the TIMx_SMCR register). Note: The clock of the slave timer or ADC must be enabled prior to receive events from the master timer, and must not be changed on-the-fly while triggers are received from the master timer.
    pub mod MMS {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Reset - the UG bit from the TIMx_EGR register is used as a trigger output (TRGO). If reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset.
            pub const B_0x0: u32 = 0b000;

            /// 0b001: Enable - the Counter enable signal, CNT_EN, is used as a trigger output (TRGO). It is useful to start several timers at the same time or to control a window in which a slave timer is enabled. The Counter Enable signal is generated by a logic OR between CEN control bit and the trigger input when configured in gated mode.
            pub const B_0x1: u32 = 0b001;

            /// 0b010: Update - The update event is selected as a trigger output (TRGO). For instance a master timer can then be used as a prescaler for a slave timer.
            pub const B_0x2: u32 = 0b010;
        }
    }
}

/// DMA/Interrupt enable register
pub mod DIER {

    /// Update interrupt enable
    pub mod UIE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Update interrupt disabled.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Update interrupt enabled.
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Update DMA request enable
    pub mod UDE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Update DMA request disabled.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Update DMA request enabled.
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// status register
pub mod SR {

    /// Update interrupt flag This bit is set by hardware on an update event. It is cleared by software. At overflow or underflow regarding the repetition counter value and if UDIS = 0 in the TIMx_CR1 register. When CNT is reinitialized by software using the UG bit in the TIMx_EGR register, if URS = 0 and UDIS = 0 in the TIMx_CR1 register.
    pub mod UIF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No update occurred.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Update interrupt pending. This bit is set by hardware when the registers are updated:
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// event generation register
pub mod EGR {

    /// Update generation This bit can be set by software, it is automatically cleared by hardware.
    pub mod UG {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No action.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Re-initializes the timer counter and generates an update of the registers. Note that the prescaler counter is cleared too (but the prescaler ratio is not affected).
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// counter
pub mod CNT {

    /// Counter value
    pub mod CNT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// UIF Copy This bit is a read-only copy of the UIF bit of the TIMx_ISR register. If the UIFREMAP bit in TIMx_CR1 is reset, bit 31 is reserved and read as 0.
    pub mod UIFCPY {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// prescaler
pub mod PSC {

    /// Prescaler value
    pub mod PSC {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// auto-reload register
pub mod ARR {

    /// Prescaler value
    pub mod ARR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u32 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register 1
    pub CR1: RWRegister<u32>,

    /// control register 2
    pub CR2: RWRegister<u32>,

    _reserved1: [u32; 1],

    /// DMA/Interrupt enable register
    pub DIER: RWRegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,

    /// event generation register
    pub EGR: WORegister<u32>,

    _reserved2: [u32; 3],

    /// counter
    pub CNT: RWRegister<u32>,

    /// prescaler
    pub PSC: RWRegister<u32>,

    /// auto-reload register
    pub ARR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub DIER: u32,
    pub SR: u32,
    pub EGR: u32,
    pub CNT: u32,
    pub PSC: u32,
    pub ARR: u32,
}
#[cfg(not(feature = "nosync"))]
pub struct Instance {
    pub(crate) addr: u32,
    pub(crate) _marker: PhantomData<*const RegisterBlock>,
}
#[cfg(not(feature = "nosync"))]
impl ::core::ops::Deref for Instance {
    type Target = RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &RegisterBlock {
        unsafe { &*(self.addr as *const _) }
    }
}
#[cfg(feature = "rtic")]
unsafe impl Send for Instance {}
