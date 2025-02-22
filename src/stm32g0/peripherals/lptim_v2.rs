#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Low power timer
//!
//! Used by: stm32g051, stm32g061, stm32g0b1, stm32g0c1

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Interrupt and Status Register
pub mod LPTIM_ISR {

    /// Compare match The CMPM bit is set by hardware to inform application that LPTIM_CNT register value reached the LPTIM_CMP registerâs value.
    pub mod CMPM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Autoreload match ARRM is set by hardware to inform application that LPTIM_CNT registerâs value reached the LPTIM_ARR registerâs value. ARRM flag can be cleared by writing 1 to the ARRMCF bit in the LPTIM_ICR register.
    pub mod ARRM {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger edge event EXTTRIG is set by hardware to inform application that a valid edge on the selected external trigger input has occurred. If the trigger is ignored because the timer has already started, then this flag is not set. EXTTRIG flag can be cleared by writing 1 to the EXTTRIGCF bit in the LPTIM_ICR register.
    pub mod EXTTRIG {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare register update OK CMPOK is set by hardware to inform application that the APB bus write operation to the LPTIM_CMP register has been successfully completed.
    pub mod CMPOK {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Autoreload register update OK ARROK is set by hardware to inform application that the APB bus write operation to the LPTIM_ARR register has been successfully completed. ARROK flag can be cleared by writing 1 to the ARROKCF bit in the LPTIM_ICR register.
    pub mod ARROK {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Counter direction change down to up In Encoder mode, UP bit is set by hardware to inform application that the counter direction has changed from down to up. UP flag can be cleared by writing 1 to the UPCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    pub mod UP {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Counter direction change up to down In Encoder mode, DOWN bit is set by hardware to inform application that the counter direction has changed from up to down. DOWN flag can be cleared by writing 1 to the DOWNCF bit in the LPTIM_ICR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    pub mod DOWN {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Clear Register
pub mod LPTIM_ICR {

    /// Compare match clear flag Writing 1 to this bit clears the CMP flag in the LPTIM_ISR register
    pub mod CMPMCF {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Autoreload match clear flag Writing 1 to this bit clears the ARRM flag in the LPTIM_ISR register
    pub mod ARRMCF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// External trigger valid edge clear flag Writing 1 to this bit clears the EXTTRIG flag in the LPTIM_ISR register
    pub mod EXTTRIGCF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Compare register update OK clear flag Writing 1 to this bit clears the CMPOK flag in the LPTIM_ISR register
    pub mod CMPOKCF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Autoreload register update OK clear flag Writing 1 to this bit clears the ARROK flag in the LPTIM_ISR register
    pub mod ARROKCF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Direction change to UP clear flag Writing 1 to this bit clear the UP flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    pub mod UPCF {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Direction change to down clear flag Writing 1 to this bit clear the DOWN flag in the LPTIM_ISR register. Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    pub mod DOWNCF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Interrupt Enable Register
pub mod LPTIM_IER {

    /// Compare match Interrupt Enable
    pub mod CMPMIE {
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

            /// 0b0: CMPM interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: CMPM interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Autoreload match Interrupt Enable
    pub mod ARRMIE {
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

            /// 0b0: ARRM interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: ARRM interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// External trigger valid edge Interrupt Enable
    pub mod EXTTRIGIE {
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

            /// 0b0: EXTTRIG interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: EXTTRIG interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Compare register update OK Interrupt Enable
    pub mod CMPOKIE {
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

            /// 0b0: CMPOK interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: CMPOK interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Autoreload register update OK Interrupt Enable
    pub mod ARROKIE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ARROK interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: ARROK interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Direction change to UP Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    pub mod UPIE {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: UP interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: UP interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Direction change to down Interrupt Enable Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    pub mod DOWNIE {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DOWN interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DOWN interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Configuration Register
pub mod LPTIM_CFGR {

    /// Clock selector The CKSEL bit selects which clock source the LPTIM will use:
    pub mod CKSEL {
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

            /// 0b0: LPTIM is clocked by internal clock source (APB clock or any of the embedded oscillators)
            pub const B_0x0: u32 = 0b0;

            /// 0b1: LPTIM is clocked by an external clock source through the LPTIM external Input1
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Clock Polarity If LPTIM is clocked by an external clock source: When the LPTIM is clocked by an external clock source, CKPOL bits is used to configure the active edge or edges used by the counter: If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 1 is active. If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 2 is active. Refer to for more details about Encoder mode sub-modes.
    pub mod CKPOL {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (2 bits: 0b11 << 1)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: the rising edge is the active edge used for counting.
            pub const B_0x0: u32 = 0b00;

            /// 0b01: the falling edge is the active edge used for counting
            pub const B_0x1: u32 = 0b01;

            /// 0b10: both edges are active edges. When both external clock signal edges are considered active ones, the LPTIM must also be clocked by an internal clock source with a frequency equal to at least four times the external clock frequency.If the LPTIM is configured in Encoder mode (ENC bit is set), the encoder sub-mode 3 is active.
            pub const B_0x2: u32 = 0b10;

            /// 0b11: not allowed
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Configurable digital filter for external clock The CKFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an external clock signal before it is considered as a valid level transition. An internal clock source must be present to use this feature
    pub mod CKFLT {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (2 bits: 0b11 << 3)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: any external clock signal level change is considered as a valid transition
            pub const B_0x0: u32 = 0b00;

            /// 0b01: external clock signal level change must be stable for at least 2 clock periods before it is considered as valid transition.
            pub const B_0x1: u32 = 0b01;

            /// 0b10: external clock signal level change must be stable for at least 4 clock periods before it is considered as valid transition.
            pub const B_0x2: u32 = 0b10;

            /// 0b11: external clock signal level change must be stable for at least 8 clock periods before it is considered as valid transition.
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Configurable digital filter for trigger The TRGFLT value sets the number of consecutive equal samples that should be detected when a level change occurs on an internal trigger before it is considered as a valid level transition. An internal clock source must be present to use this feature
    pub mod TRGFLT {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: any trigger active level change is considered as a valid trigger
            pub const B_0x0: u32 = 0b00;

            /// 0b01: trigger active level change must be stable for at least 2 clock periods before it is considered as valid trigger.
            pub const B_0x1: u32 = 0b01;

            /// 0b10: trigger active level change must be stable for at least 4 clock periods before it is considered as valid trigger.
            pub const B_0x2: u32 = 0b10;

            /// 0b11: trigger active level change must be stable for at least 8 clock periods before it is considered as valid trigger.
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Clock prescaler The PRESC bits configure the prescaler division factor. It can be one among the following division factors:
    pub mod PRESC {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (3 bits: 0b111 << 9)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: /1
            pub const B_0x0: u32 = 0b000;

            /// 0b001: /2
            pub const B_0x1: u32 = 0b001;

            /// 0b010: /4
            pub const B_0x2: u32 = 0b010;

            /// 0b011: /8
            pub const B_0x3: u32 = 0b011;

            /// 0b100: /16
            pub const B_0x4: u32 = 0b100;

            /// 0b101: /32
            pub const B_0x5: u32 = 0b101;

            /// 0b110: /64
            pub const B_0x6: u32 = 0b110;

            /// 0b111: /128
            pub const B_0x7: u32 = 0b111;
        }
    }

    /// Trigger selector The TRIGSEL bits select the trigger source that will serve as a trigger event for the LPTIM among the below 8 available sources: See for details.
    pub mod TRIGSEL {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (3 bits: 0b111 << 13)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: lptim_ext_trig0
            pub const B_0x0: u32 = 0b000;

            /// 0b001: lptim_ext_trig1
            pub const B_0x1: u32 = 0b001;

            /// 0b010: lptim_ext_trig2
            pub const B_0x2: u32 = 0b010;

            /// 0b011: lptim_ext_trig3
            pub const B_0x3: u32 = 0b011;

            /// 0b100: lptim_ext_trig4
            pub const B_0x4: u32 = 0b100;

            /// 0b101: lptim_ext_trig5
            pub const B_0x5: u32 = 0b101;

            /// 0b110: lptim_ext_trig6
            pub const B_0x6: u32 = 0b110;

            /// 0b111: lptim_ext_trig7
            pub const B_0x7: u32 = 0b111;
        }
    }

    /// Trigger enable and polarity The TRIGEN bits controls whether the LPTIM counter is started by an external trigger or not. If the external trigger option is selected, three configurations are possible for the trigger active edge:
    pub mod TRIGEN {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (2 bits: 0b11 << 17)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: software trigger (counting start is initiated by software)
            pub const B_0x0: u32 = 0b00;

            /// 0b01: rising edge is the active edge
            pub const B_0x1: u32 = 0b01;

            /// 0b10: falling edge is the active edge
            pub const B_0x2: u32 = 0b10;

            /// 0b11: both edges are active edges
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// Timeout enable The TIMOUT bit controls the Timeout feature
    pub mod TIMOUT {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: A trigger event arriving when the timer is already started will be ignored
            pub const B_0x0: u32 = 0b0;

            /// 0b1: A trigger event arriving when the timer is already started will reset and restart the counter
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Waveform shape The WAVE bit controls the output shape
    pub mod WAVE {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Deactivate Set-once mode, PWM or One Pulse waveform depending on how the timer was started, CNTSTRT for PWM or SNGSTRT for One Pulse waveform.
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Activate the Set-once mode
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Waveform shape polarity The WAVEPOL bit controls the output polarity
    pub mod WAVPOL {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: The LPTIM output reflects the compare results between LPTIM_CNT and LPTIM_CMP registers
            pub const B_0x0: u32 = 0b0;

            /// 0b1: The LPTIM output reflects the inverse of the compare results between LPTIM_CNT and LPTIM_CMP registers
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Registers update mode The PRELOAD bit controls the LPTIM_ARR and the LPTIM_CMP registers update modality
    pub mod PRELOAD {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Registers are updated after each APB bus write access
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Registers are updated at the end of the current LPTIM period
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// counter mode enabled The COUNTMODE bit selects which clock source is used by the LPTIM to clock the counter:
    pub mod COUNTMODE {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: the counter is incremented following each internal clock pulse
            pub const B_0x0: u32 = 0b0;

            /// 0b1: the counter is incremented following each valid clock pulse on the LPTIM external Input1
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// Encoder mode enable The ENC bit controls the Encoder mode Note: If the LPTIM does not support encoder mode feature, this bit is reserved. Please refer to .
    pub mod ENC {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (1 bit: 1 << 24)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Encoder mode disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Encoder mode enabled
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// Control Register
pub mod LPTIM_CR {

    /// LPTIM enable The ENABLE bit is set and cleared by software.
    pub mod ENABLE {
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

            /// 0b0: LPTIM is disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: LPTIM is enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// LPTIM start in Single mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00â), setting this bit starts the LPTIM in single pulse mode. If the software start is disabled (TRIGEN\[1:0\] different than '00â), setting this bit starts the LPTIM in single pulse mode as soon as an external trigger is detected. If this bit is set when the LPTIM is in continuous counting mode, then the LPTIM will stop at the following match between LPTIM_ARR and LPTIM_CNT registers. This bit can only be set when the LPTIM is enabled. It will be automatically reset by hardware.
    pub mod SNGSTRT {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Timer start in Continuous mode This bit is set by software and cleared by hardware. In case of software start (TRIGEN\[1:0\] = '00â), setting this bit starts the LPTIM in Continuous mode. If the software start is disabled (TRIGEN\[1:0\] different than '00â), setting this bit starts the timer in Continuous mode as soon as an external trigger is detected. If this bit is set when a single pulse mode counting is ongoing, then the timer will not stop at the next match between the LPTIM_ARR and LPTIM_CNT registers and the LPTIM counter keeps counting in Continuous mode. This bit can be set only when the LPTIM is enabled. It will be automatically reset by hardware.
    pub mod CNTSTRT {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Counter reset This bit is set by software and cleared by hardware. When set to '1' this bit will trigger a synchronous reset of the LPTIM_CNT counter register. Due to the synchronous nature of this reset, it only takes place after a synchronization delay of 3 LPTimer core clock cycles (LPTimer core clock may be different from APB clock). COUNTRST must never be set to '1' by software before it is already cleared to '0' by hardware. Software should consequently check that COUNTRST bit is already cleared to '0' before attempting to set it to '1'.
    pub mod COUNTRST {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Reset after read enable This bit is set and cleared by software. When RSTARE is set to '1', any read access to LPTIM_CNT register will asynchronously reset LPTIM_CNT register content.
    pub mod RSTARE {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Compare Register
pub mod LPTIM_CMP {

    /// Compare value
    pub mod CMP {
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

/// Autoreload Register
pub mod LPTIM_ARR {

    /// Auto reload value
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

/// Counter Register
pub mod LPTIM_CNT {

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
}

/// LPTIM configuration register 2
pub mod LPTIM_CFGR2 {

    /// LPTIM input 1 selection The IN1SEL bits control the LPTIM Input 1 multiplexer, which connects LPTIM Input 1 to one of the available inputs. For connection details refer to .
    pub mod IN1SEL {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (2 bits: 0b11 << 0)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: lptim_in1_mux0
            pub const B_0x0: u32 = 0b00;

            /// 0b01: lptim_in1_mux1
            pub const B_0x1: u32 = 0b01;

            /// 0b10: lptim_in1_mux2
            pub const B_0x2: u32 = 0b10;

            /// 0b11: lptim_in1_mux3
            pub const B_0x3: u32 = 0b11;
        }
    }

    /// LPTIM input 2 selection The IN2SEL bits control the LPTIM Input 2 multiplexer, which connect LPTIM Input 2 to one of the available inputs. For connection details refer to . Note: If the LPTIM does not support encoder mode feature, these bits are reserved. Please refer to .
    pub mod IN2SEL {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (2 bits: 0b11 << 4)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: lptim_in2_mux0
            pub const B_0x0: u32 = 0b00;

            /// 0b01: lptim_in2_mux1
            pub const B_0x1: u32 = 0b01;

            /// 0b10: lptim_in2_mux2
            pub const B_0x2: u32 = 0b10;

            /// 0b11: lptim_in2_mux3
            pub const B_0x3: u32 = 0b11;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    /// Interrupt and Status Register
    pub LPTIM_ISR: RORegister<u32>,

    /// Interrupt Clear Register
    pub LPTIM_ICR: WORegister<u32>,

    /// Interrupt Enable Register
    pub LPTIM_IER: RWRegister<u32>,

    /// Configuration Register
    pub LPTIM_CFGR: RWRegister<u32>,

    /// Control Register
    pub LPTIM_CR: RWRegister<u32>,

    /// Compare Register
    pub LPTIM_CMP: RWRegister<u32>,

    /// Autoreload Register
    pub LPTIM_ARR: RWRegister<u32>,

    /// Counter Register
    pub LPTIM_CNT: RORegister<u32>,

    _reserved1: [u32; 1],

    /// LPTIM configuration register 2
    pub LPTIM_CFGR2: RWRegister<u32>,
}
pub struct ResetValues {
    pub LPTIM_ISR: u32,
    pub LPTIM_ICR: u32,
    pub LPTIM_IER: u32,
    pub LPTIM_CFGR: u32,
    pub LPTIM_CR: u32,
    pub LPTIM_CMP: u32,
    pub LPTIM_ARR: u32,
    pub LPTIM_CNT: u32,
    pub LPTIM_CFGR2: u32,
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
