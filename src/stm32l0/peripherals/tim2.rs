#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! General-purpose-timers
//!
//! Used by: stm32l0x0, stm32l0x1, stm32l0x2, stm32l0x3

use crate::{RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// control register 1
pub mod CR1 {

    /// Clock division
    pub mod CKD {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: t_DTS = t_CK_INT
            pub const Div1: u32 = 0b00;

            /// 0b01: t_DTS = 2 × t_CK_INT
            pub const Div2: u32 = 0b01;

            /// 0b10: t_DTS = 4 × t_CK_INT
            pub const Div4: u32 = 0b10;
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

            /// 0b0: TIMx_APRR register is not buffered
            pub const Disabled: u32 = 0b0;

            /// 0b1: TIMx_APRR register is buffered
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Center-aligned mode selection
    pub mod CMS {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (2 bits: 0b11 << 5)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: The counter counts up or down depending on the direction bit
            pub const EdgeAligned: u32 = 0b00;

            /// 0b01: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down.
            pub const CenterAligned1: u32 = 0b01;

            /// 0b10: The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up.
            pub const CenterAligned2: u32 = 0b10;

            /// 0b11: The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down.
            pub const CenterAligned3: u32 = 0b11;
        }
    }

    /// Direction
    pub mod DIR {
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

            /// 0b0: Counter used as upcounter
            pub const Up: u32 = 0b0;

            /// 0b1: Counter used as downcounter
            pub const Down: u32 = 0b1;
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: Counter stops counting at the next update event (clearing the CEN bit)
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Update request source
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

            /// 0b0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request
            pub const AnyEvent: u32 = 0b0;

            /// 0b1: Only counter overflow/underflow generates an update interrupt or DMA request
            pub const CounterOnly: u32 = 0b1;
        }
    }

    /// Update disable
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

            /// 0b0: Update event enabled
            pub const Enabled: u32 = 0b0;

            /// 0b1: Update event disabled
            pub const Disabled: u32 = 0b1;
        }
    }

    /// Counter enable
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
            pub const Disabled: u32 = 0b0;

            /// 0b1: Counter enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// control register 2
pub mod CR2 {

    /// TI1 selection
    pub mod TI1S {
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

            /// 0b0: The TIMx_CH1 pin is connected to TI1 input
            pub const Normal: u32 = 0b0;

            /// 0b1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input
            pub const XOR: u32 = 0b1;
        }
    }

    /// Master mode selection
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

            /// 0b000: The UG bit from the TIMx_EGR register is used as trigger output
            pub const Reset: u32 = 0b000;

            /// 0b001: The counter enable signal, CNT_EN, is used as trigger output
            pub const Enable: u32 = 0b001;

            /// 0b010: The update event is selected as trigger output
            pub const Update: u32 = 0b010;

            /// 0b011: The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred
            pub const ComparePulse: u32 = 0b011;

            /// 0b100: OC1REF signal is used as trigger output
            pub const CompareOC1: u32 = 0b100;

            /// 0b101: OC2REF signal is used as trigger output
            pub const CompareOC2: u32 = 0b101;

            /// 0b110: OC3REF signal is used as trigger output
            pub const CompareOC3: u32 = 0b110;

            /// 0b111: OC4REF signal is used as trigger output
            pub const CompareOC4: u32 = 0b111;
        }
    }

    /// Capture/compare DMA selection
    pub mod CCDS {
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

            /// 0b0: CCx DMA request sent when CCx event occurs
            pub const OnCompare: u32 = 0b0;

            /// 0b1: CCx DMA request sent when update event occurs
            pub const OnUpdate: u32 = 0b1;
        }
    }
}

/// slave mode control register
pub mod SMCR {

    /// External trigger polarity
    pub mod ETP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: ETR is noninverted, active at high level or rising edge
            pub const NotInverted: u32 = 0b0;

            /// 0b1: ETR is inverted, active at low level or falling edge
            pub const Inverted: u32 = 0b1;
        }
    }

    /// External clock enable
    pub mod ECE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: External clock mode 2 disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal.
            pub const Enabled: u32 = 0b1;
        }
    }

    /// External trigger prescaler
    pub mod ETPS {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (2 bits: 0b11 << 12)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: Prescaler OFF
            pub const Div1: u32 = 0b00;

            /// 0b01: ETRP frequency divided by 2
            pub const Div2: u32 = 0b01;

            /// 0b10: ETRP frequency divided by 4
            pub const Div4: u32 = 0b10;

            /// 0b11: ETRP frequency divided by 8
            pub const Div8: u32 = 0b11;
        }
    }

    /// External trigger filter
    pub mod ETF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (4 bits: 0b1111 << 8)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filter, sampling is done at fDTS
            pub const NoFilter: u32 = 0b0000;

            /// 0b0001: fSAMPLING=fCK_INT, N=2
            pub const FCK_INT_N2: u32 = 0b0001;

            /// 0b0010: fSAMPLING=fCK_INT, N=4
            pub const FCK_INT_N4: u32 = 0b0010;

            /// 0b0011: fSAMPLING=fCK_INT, N=8
            pub const FCK_INT_N8: u32 = 0b0011;

            /// 0b0100: fSAMPLING=fDTS/2, N=6
            pub const FDTS_Div2_N6: u32 = 0b0100;

            /// 0b0101: fSAMPLING=fDTS/2, N=8
            pub const FDTS_Div2_N8: u32 = 0b0101;

            /// 0b0110: fSAMPLING=fDTS/4, N=6
            pub const FDTS_Div4_N6: u32 = 0b0110;

            /// 0b0111: fSAMPLING=fDTS/4, N=8
            pub const FDTS_Div4_N8: u32 = 0b0111;

            /// 0b1000: fSAMPLING=fDTS/8, N=6
            pub const FDTS_Div8_N6: u32 = 0b1000;

            /// 0b1001: fSAMPLING=fDTS/8, N=8
            pub const FDTS_Div8_N8: u32 = 0b1001;

            /// 0b1010: fSAMPLING=fDTS/16, N=5
            pub const FDTS_Div16_N5: u32 = 0b1010;

            /// 0b1011: fSAMPLING=fDTS/16, N=6
            pub const FDTS_Div16_N6: u32 = 0b1011;

            /// 0b1100: fSAMPLING=fDTS/16, N=8
            pub const FDTS_Div16_N8: u32 = 0b1100;

            /// 0b1101: fSAMPLING=fDTS/32, N=5
            pub const FDTS_Div32_N5: u32 = 0b1101;

            /// 0b1110: fSAMPLING=fDTS/32, N=6
            pub const FDTS_Div32_N6: u32 = 0b1110;

            /// 0b1111: fSAMPLING=fDTS/32, N=8
            pub const FDTS_Div32_N8: u32 = 0b1111;
        }
    }

    /// Master/Slave mode
    pub mod MSM {
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

            /// 0b0: No action
            pub const NoSync: u32 = 0b0;

            /// 0b1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event.
            pub const Sync: u32 = 0b1;
        }
    }

    /// Trigger selection
    pub mod TS {
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

            /// 0b000: Internal Trigger 0 (ITR0)
            pub const ITR0: u32 = 0b000;

            /// 0b001: Internal Trigger 1 (ITR1)
            pub const ITR1: u32 = 0b001;

            /// 0b010: Internal Trigger 2 (ITR2)
            pub const ITR2: u32 = 0b010;

            /// 0b100: TI1 Edge Detector (TI1F_ED)
            pub const TI1F_ED: u32 = 0b100;

            /// 0b101: Filtered Timer Input 1 (TI1FP1)
            pub const TI1FP1: u32 = 0b101;

            /// 0b110: Filtered Timer Input 2 (TI2FP2)
            pub const TI2FP2: u32 = 0b110;

            /// 0b111: External Trigger input (ETRF)
            pub const ETRF: u32 = 0b111;
        }
    }

    /// Slave mode selection
    pub mod SMS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock.
            pub const Disabled: u32 = 0b000;

            /// 0b001: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level.
            pub const Encoder_Mode_1: u32 = 0b001;

            /// 0b010: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level.
            pub const Encoder_Mode_2: u32 = 0b010;

            /// 0b011: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input.
            pub const Encoder_Mode_3: u32 = 0b011;

            /// 0b100: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers.
            pub const Reset_Mode: u32 = 0b100;

            /// 0b101: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled.
            pub const Gated_Mode: u32 = 0b101;

            /// 0b110: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled.
            pub const Trigger_Mode: u32 = 0b110;

            /// 0b111: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter.
            pub const Ext_Clock_Mode: u32 = 0b111;
        }
    }
}

/// DMA/Interrupt enable register
pub mod DIER {

    /// Trigger DMA request enable
    pub mod TDE {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Trigger DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Trigger DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 4 DMA request enable
    pub mod CC4DE {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: CCx DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CCx DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 3 DMA request enable
    pub mod CC3DE {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4DE::RW;
    }

    /// Capture/Compare 2 DMA request enable
    pub mod CC2DE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4DE::RW;
    }

    /// Capture/Compare 1 DMA request enable
    pub mod CC1DE {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4DE::RW;
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

            /// 0b0: Update DMA request disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update DMA request enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Trigger interrupt enable
    pub mod TIE {
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

            /// 0b0: Trigger interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Trigger interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 4 interrupt enable
    pub mod CC4IE {
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

            /// 0b0: CCx interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: CCx interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 3 interrupt enable
    pub mod CC3IE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4IE::RW;
    }

    /// Capture/Compare 2 interrupt enable
    pub mod CC2IE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4IE::RW;
    }

    /// Capture/Compare 1 interrupt enable
    pub mod CC1IE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4IE::RW;
    }

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

            /// 0b0: Update interrupt disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Update interrupt enabled
            pub const Enabled: u32 = 0b1;
        }
    }
}

/// status register
pub mod SR {

    /// Capture/Compare 4 overcapture flag
    pub mod CC4OF {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: The counter value has been captured in TIMx_CCRx register while CCxIF flag was already set
            pub const Overcapture: u32 = 0b1;
        }
        /// Write-only values
        pub mod W {

            /// 0b0: Clear flag
            pub const Clear: u32 = 0b0;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 3 overcapture flag
    pub mod CC3OF {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        pub use super::CC4OF::R;
        pub use super::CC4OF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/compare 2 overcapture flag
    pub mod CC2OF {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        pub use super::CC4OF::R;
        pub use super::CC4OF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 1 overcapture flag
    pub mod CC1OF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        pub use super::CC4OF::R;
        pub use super::CC4OF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Trigger interrupt flag
    pub mod TIF {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b0: No trigger event occurred
            pub const NoTrigger: u32 = 0b0;

            /// 0b1: Trigger interrupt pending
            pub const Trigger: u32 = 0b1;
        }
        pub use super::CC4OF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 4 interrupt flag
    pub mod CC4IF {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
            pub const Match: u32 = 0b1;
        }
        pub use super::CC4OF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 3 interrupt flag
    pub mod CC3IF {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
            pub const Match: u32 = 0b1;
        }
        pub use super::CC4OF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 interrupt flag
    pub mod CC2IF {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
            pub const Match: u32 = 0b1;
        }
        pub use super::CC4OF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/compare 1 interrupt flag
    pub mod CC1IF {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values
        pub mod R {

            /// 0b1: If CC1 is an output: The content of the counter TIMx_CNT matches the content of the TIMx_CCR1 register. If CC1 is an input: The counter value has been captured in TIMx_CCR1 register.
            pub const Match: u32 = 0b1;
        }
        pub use super::CC4OF::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update interrupt flag
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

            /// 0b0: No update occurred
            pub const Clear: u32 = 0b0;

            /// 0b1: Update interrupt pending.
            pub const UpdatePending: u32 = 0b1;
        }
    }
}

/// event generation register
pub mod EGR {

    /// Trigger generation
    pub mod TG {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: The TIF flag is set in TIMx_SR register. Related interrupt or DMA transfer can occur if enabled.
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/compare 4 generation
    pub mod CC4G {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values
        pub mod W {

            /// 0b1: If CC1 is an output: CC1IF flag is set, Corresponding interrupt or DMA request is sent if enabled. If CC1 is an input: The current value of the counter is captured in TIMx_CCR1 register.
            pub const Trigger: u32 = 0b1;
        }
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/compare 3 generation
    pub mod CC3G {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CC4G::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/compare 2 generation
    pub mod CC2G {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CC4G::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/compare 1 generation
    pub mod CC1G {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        pub use super::CC4G::W;
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Update generation
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

            /// 0b1: Re-initializes the timer counter and generates an update of the registers.
            pub const Update: u32 = 0b1;
        }
    }
}

/// CCMR1_Output and CCMR1_Input
/// CCMR1_Output: capture/compare mode register 1 (output mode)
/// CCMR1_Input: capture/compare mode register 1 (input mode)
pub mod CCMR1 {

    /// Output compare 2 clear enable
    pub mod OC2CE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output compare 2 mode
    pub mod OC2M {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
            pub const Frozen: u32 = 0b000;

            /// 0b001: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
            pub const ActiveOnMatch: u32 = 0b001;

            /// 0b010: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
            pub const InactiveOnMatch: u32 = 0b010;

            /// 0b011: OCyREF toggles when TIMx_CNT=TIMx_CCRy
            pub const Toggle: u32 = 0b011;

            /// 0b100: OCyREF is forced low
            pub const ForceInactive: u32 = 0b100;

            /// 0b101: OCyREF is forced high
            pub const ForceActive: u32 = 0b101;

            /// 0b110: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
            pub const PwmMode1: u32 = 0b110;

            /// 0b111: Inversely to PwmMode1
            pub const PwmMode2: u32 = 0b111;
        }
    }

    /// Output compare 2 preload enable
    pub mod OC2PE {
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

            /// 0b0: Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload register on CCR2 enabled. Preload value is loaded into active register on each update event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output compare 2 fast enable
    pub mod OC2FE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 2 selection
    pub mod CC2S {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CC2 channel is configured as output
            pub const Output: u32 = 0b00;
        }
    }

    /// Output compare 1 clear enable
    pub mod OC1CE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output compare 1 mode
    pub mod OC1M {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC2M::RW;
    }

    /// Output compare 1 preload enable
    pub mod OC1PE {
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

            /// 0b0: Preload register on CCR1 disabled. New values written to CCR1 are taken into account immediately
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload register on CCR1 enabled. Preload value is loaded into active register on each update event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output compare 1 fast enable
    pub mod OC1FE {
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

    /// Capture/Compare 1 selection
    pub mod CC1S {
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

            /// 0b00: CC1 channel is configured as output
            pub const Output: u32 = 0b00;
        }
    }

    /// Input capture 2 filter
    pub mod IC2F {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input capture 2 prescaler
    pub mod IC2PSC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input capture 1 filter
    pub mod IC1F {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: No filter, sampling is done at fDTS
            pub const NoFilter: u32 = 0b0000;

            /// 0b0001: fSAMPLING=fCK_INT, N=2
            pub const FCK_INT_N2: u32 = 0b0001;

            /// 0b0010: fSAMPLING=fCK_INT, N=4
            pub const FCK_INT_N4: u32 = 0b0010;

            /// 0b0011: fSAMPLING=fCK_INT, N=8
            pub const FCK_INT_N8: u32 = 0b0011;

            /// 0b0100: fSAMPLING=fDTS/2, N=6
            pub const FDTS_Div2_N6: u32 = 0b0100;

            /// 0b0101: fSAMPLING=fDTS/2, N=8
            pub const FDTS_Div2_N8: u32 = 0b0101;

            /// 0b0110: fSAMPLING=fDTS/4, N=6
            pub const FDTS_Div4_N6: u32 = 0b0110;

            /// 0b0111: fSAMPLING=fDTS/4, N=8
            pub const FDTS_Div4_N8: u32 = 0b0111;

            /// 0b1000: fSAMPLING=fDTS/8, N=6
            pub const FDTS_Div8_N6: u32 = 0b1000;

            /// 0b1001: fSAMPLING=fDTS/8, N=8
            pub const FDTS_Div8_N8: u32 = 0b1001;

            /// 0b1010: fSAMPLING=fDTS/16, N=5
            pub const FDTS_Div16_N5: u32 = 0b1010;

            /// 0b1011: fSAMPLING=fDTS/16, N=6
            pub const FDTS_Div16_N6: u32 = 0b1011;

            /// 0b1100: fSAMPLING=fDTS/16, N=8
            pub const FDTS_Div16_N8: u32 = 0b1100;

            /// 0b1101: fSAMPLING=fDTS/32, N=5
            pub const FDTS_Div32_N5: u32 = 0b1101;

            /// 0b1110: fSAMPLING=fDTS/32, N=6
            pub const FDTS_Div32_N6: u32 = 0b1110;

            /// 0b1111: fSAMPLING=fDTS/32, N=8
            pub const FDTS_Div32_N8: u32 = 0b1111;
        }
    }

    /// Input capture 1 prescaler
    pub mod IC1PSC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: no prescaler, capture is done each time an edge is detected on the capture input
            pub const NoPrescaler: u32 = 0b00;

            /// 0b01: Capture is done once every 2 events
            pub const Two_Events: u32 = 0b01;

            /// 0b10: Capture is done once every 4 events
            pub const Four_Events: u32 = 0b10;

            /// 0b11: Capture is done once every 8 events
            pub const Eight_Events: u32 = 0b11;
        }
    }
}

/// CCMR2_Output and CCMR2_Input
/// CCMR2_Output: capture/compare mode register 2 (output mode)
/// CCMR2_Input: capture/compare mode register 2 (input mode)
pub mod CCMR2 {

    /// Output compare 4 clear enable
    pub mod OC4CE {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output compare 4 mode
    pub mod OC4M {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (3 bits: 0b111 << 12)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs
            pub const Frozen: u32 = 0b000;

            /// 0b001: Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register
            pub const ActiveOnMatch: u32 = 0b001;

            /// 0b010: Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register
            pub const InactiveOnMatch: u32 = 0b010;

            /// 0b011: OCyREF toggles when TIMx_CNT=TIMx_CCRy
            pub const Toggle: u32 = 0b011;

            /// 0b100: OCyREF is forced low
            pub const ForceInactive: u32 = 0b100;

            /// 0b101: OCyREF is forced high
            pub const ForceActive: u32 = 0b101;

            /// 0b110: In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active
            pub const PwmMode1: u32 = 0b110;

            /// 0b111: Inversely to PwmMode1
            pub const PwmMode2: u32 = 0b111;
        }
    }

    /// Output compare 4 preload enable
    pub mod OC4PE {
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

            /// 0b0: Preload register on CCR4 disabled. New values written to CCR4 are taken into account immediately
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload register on CCR4 enabled. Preload value is loaded into active register on each update event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output compare 4 fast enable
    pub mod OC4FE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Capture/Compare 4 selection
    pub mod CC4S {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: CC4 channel is configured as output
            pub const Output: u32 = 0b00;
        }
    }

    /// Output compare 3 clear enable
    pub mod OC3CE {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Output compare 3 mode
    pub mod OC3M {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (3 bits: 0b111 << 4)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::OC4M::RW;
    }

    /// Output compare 3 preload enable
    pub mod OC3PE {
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

            /// 0b0: Preload register on CCR3 disabled. New values written to CCR3 are taken into account immediately
            pub const Disabled: u32 = 0b0;

            /// 0b1: Preload register on CCR3 enabled. Preload value is loaded into active register on each update event
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Output compare 3 fast enable
    pub mod OC3FE {
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

    /// Capture/Compare 3 selection
    pub mod CC3S {
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

            /// 0b00: CC3 channel is configured as output
            pub const Output: u32 = 0b00;
        }
    }

    /// Input capture 4 filter
    pub mod IC4F {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (4 bits: 0b1111 << 12)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input capture 4 prescaler
    pub mod IC4PSC {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input capture 3 filter
    pub mod IC3F {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (4 bits: 0b1111 << 4)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Input capture 3 prescaler
    pub mod IC3PSC {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (2 bits: 0b11 << 2)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// capture/compare enable register
pub mod CCER {

    /// Capture/Compare 4 output Polarity
    pub mod CC4NP {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Negative polarity
            pub const Negative: u32 = 0b0;

            /// 0b1: Positive polarity
            pub const Positive: u32 = 0b1;
        }
    }

    /// Capture/Compare 3 output Polarity
    pub mod CC4P {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Noninverted/rising edge
            pub const RisingEdge: u32 = 0b0;

            /// 0b1: Inverted/falling edge
            pub const FallingEdge: u32 = 0b1;
        }
    }

    /// Capture/Compare 4 output enable
    pub mod CC4E {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Capture disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Capture enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Capture/Compare 3 output Polarity
    pub mod CC3NP {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4NP::RW;
    }

    /// Capture/Compare 3 output Polarity
    pub mod CC3P {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4P::RW;
    }

    /// Capture/Compare 3 output enable
    pub mod CC3E {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4E::RW;
    }

    /// Capture/Compare 2 output Polarity
    pub mod CC2NP {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4NP::RW;
    }

    /// Capture/Compare 2 output Polarity
    pub mod CC2P {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4P::RW;
    }

    /// Capture/Compare 2 output enable
    pub mod CC2E {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4E::RW;
    }

    /// Capture/Compare 1 output Polarity
    pub mod CC1NP {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4NP::RW;
    }

    /// Capture/Compare 1 output Polarity
    pub mod CC1P {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4P::RW;
    }

    /// Capture/Compare 1 output enable
    pub mod CC1E {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CC4E::RW;
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

/// DMA control register
pub mod DCR {

    /// DMA burst length
    pub mod DBL {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (5 bits: 0b11111 << 8)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DMA base address
    pub mod DBA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (5 bits: 0b11111 << 0)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DMA address for full transfer
pub mod DMAR {

    /// DMA register for burst accesses
    pub mod DMAB {
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

/// TIM2 option register
pub mod OR {

    /// Timer2 ETR remap
    pub mod ETR_RMP {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b111: TIM2 ETR input is connected to COMP1_OUT
            pub const COMP1_OUT: u32 = 0b111;

            /// 0b110: TIM2 ETR input is connected to COMP2_OUT
            pub const COMP2_OUT: u32 = 0b110;

            /// 0b101: TIM2 ETR input is connected to LSE
            pub const LSE: u32 = 0b101;

            /// 0b011: TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set
            pub const HSI: u32 = 0b011;
        }
    }

    /// Internal trigger
    pub mod TI4_RMP {
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

            /// 0b01: TIM2 TI4 input connected to COMP2_OUT
            pub const COMP2_OUT: u32 = 0b01;

            /// 0b10: TIM2 TI4 input connected to COMP1_OUT
            pub const COMP1_OUT: u32 = 0b10;
        }
    }
}

/// TIMx counter
pub mod CNT {

    /// Low counter value
    pub mod CNT {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// TIMx auto-reload register
pub mod ARR {

    /// Low Auto-reload value
    pub mod ARR {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// capture/compare register
pub mod CCR1 {

    /// Capture/Compare value
    pub mod CCR {
        /// Offset (0 bits)
        pub const offset: u16 = 0;
        /// Mask (16 bits: 0xffff << 0)
        pub const mask: u16 = 0xffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// capture/compare register
pub mod CCR2 {
    pub use super::CCR1::CCR;
}

/// capture/compare register
pub mod CCR3 {
    pub use super::CCR1::CCR;
}

/// capture/compare register
pub mod CCR4 {
    pub use super::CCR1::CCR;
}
#[repr(C)]
pub struct RegisterBlock {
    /// control register 1
    pub CR1: RWRegister<u32>,

    /// control register 2
    pub CR2: RWRegister<u32>,

    /// slave mode control register
    pub SMCR: RWRegister<u32>,

    /// DMA/Interrupt enable register
    pub DIER: RWRegister<u32>,

    /// status register
    pub SR: RWRegister<u32>,

    /// event generation register
    pub EGR: WORegister<u32>,

    /// CCMR1_Output and CCMR1_Input
    /// CCMR1_Output: capture/compare mode register 1 (output mode)
    /// CCMR1_Input: capture/compare mode register 1 (input mode)
    pub CCMR1: RWRegister<u32>,

    /// CCMR2_Output and CCMR2_Input
    /// CCMR2_Output: capture/compare mode register 2 (output mode)
    /// CCMR2_Input: capture/compare mode register 2 (input mode)
    pub CCMR2: RWRegister<u32>,

    /// capture/compare enable register
    pub CCER: RWRegister<u32>,

    /// TIMx counter
    pub CNT: RWRegister<u16>,

    _reserved1: [u16; 1],

    /// prescaler
    pub PSC: RWRegister<u32>,

    /// TIMx auto-reload register
    pub ARR: RWRegister<u16>,

    _reserved2: [u16; 3],

    /// capture/compare register
    pub CCR1: RWRegister<u16>,

    _reserved3: [u16; 1],

    /// capture/compare register
    pub CCR2: RWRegister<u16>,

    _reserved4: [u16; 1],

    /// capture/compare register
    pub CCR3: RWRegister<u16>,

    _reserved5: [u16; 1],

    /// capture/compare register
    pub CCR4: RWRegister<u16>,

    _reserved6: [u16; 3],

    /// DMA control register
    pub DCR: RWRegister<u32>,

    /// DMA address for full transfer
    pub DMAR: RWRegister<u32>,

    /// TIM2 option register
    pub OR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub SMCR: u32,
    pub DIER: u32,
    pub SR: u32,
    pub EGR: u32,
    pub CCMR1: u32,
    pub CCMR2: u32,
    pub CCER: u32,
    pub CNT: u16,
    pub PSC: u32,
    pub ARR: u16,
    pub CCR1: u16,
    pub CCR2: u16,
    pub CCR3: u16,
    pub CCR4: u16,
    pub DCR: u32,
    pub DMAR: u32,
    pub OR: u32,
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
