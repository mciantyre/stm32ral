#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Power control
//!
//! Used by: stm32l412, stm32l4r9, stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Power control register 1
pub mod CR1 {

    /// Low-power run
    pub mod LPR {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voltage scaling range selection
    pub mod VOS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Disable backup domain write protection
    pub mod DBP {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-power mode selection
    pub mod LPMS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (3 bits: 0b111 << 0)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power control register 2
pub mod CR2 {

    /// VDDUSB USB supply valid
    pub mod USV {
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

    /// VDDIO2 Independent I/Os supply valid
    pub mod IOSV {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V
    pub mod PVME4 {
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

    /// Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V
    pub mod PVME3 {
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

    /// Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V
    pub mod PVME2 {
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

    /// Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V
    pub mod PVME1 {
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

    /// Power voltage detector level selection
    pub mod PLS {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (3 bits: 0b111 << 1)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power voltage detector enable
    pub mod PVDE {
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
}

/// Power control register 3
pub mod CR3 {

    /// Enable internal wakeup line
    pub mod EWF {
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

    /// Apply pull-up and pull-down configuration
    pub mod APC {
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

    /// SRAM2 retention in Standby mode
    pub mod RRS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Enable Wakeup pin WKUP5
    pub mod EWUP5 {
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

    /// Enable Wakeup pin WKUP4
    pub mod EWUP4 {
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

    /// Enable Wakeup pin WKUP3
    pub mod EWUP3 {
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

    /// Enable Wakeup pin WKUP2
    pub mod EWUP2 {
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

    /// Enable Wakeup pin WKUP1
    pub mod EWUP1 {
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
}

/// Power control register 4
pub mod CR4 {

    /// VBAT battery charging resistor selection
    pub mod VBRS {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// VBAT battery charging enable
    pub mod VBE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup pin WKUP5 polarity
    pub mod WP5 {
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

    /// Wakeup pin WKUP4 polarity
    pub mod WP4 {
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

    /// Wakeup pin WKUP3 polarity
    pub mod WP3 {
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

    /// Wakeup pin WKUP2 polarity
    pub mod WP2 {
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

    /// Wakeup pin WKUP1 polarity
    pub mod WP1 {
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
}

/// Power status register 1
pub mod SR1 {

    /// Wakeup flag internal
    pub mod WUFI {
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

    /// Standby flag
    pub mod CSBF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Wakeup flag 5
    pub mod CWUF5 {
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

    /// Wakeup flag 4
    pub mod CWUF4 {
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

    /// Wakeup flag 3
    pub mod CWUF3 {
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

    /// Wakeup flag 2
    pub mod CWUF2 {
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

    /// Wakeup flag 1
    pub mod CWUF1 {
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
}

/// Power status register 2
pub mod SR2 {

    /// Peripheral voltage monitoring output: VDDA vs. 2.2 V
    pub mod PVMO4 {
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

    /// Peripheral voltage monitoring output: VDDA vs. 1.62 V
    pub mod PVMO3 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V
    pub mod PVMO2 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Peripheral voltage monitoring output: VDDUSB vs. 1.2 V
    pub mod PVMO1 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Power voltage detector output
    pub mod PVDO {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Voltage scaling flag
    pub mod VOSF {
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

    /// Low-power regulator flag
    pub mod REGLPF {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Low-power regulator started
    pub mod REGLPS {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// Power status clear register
pub mod SCR {

    /// Clear standby flag
    pub mod SBF {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Clear wakeup flag 5
    pub mod WUF5 {
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

    /// Clear wakeup flag 4
    pub mod WUF4 {
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

    /// Clear wakeup flag 3
    pub mod WUF3 {
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

    /// Clear wakeup flag 2
    pub mod WUF2 {
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

    /// Clear wakeup flag 1
    pub mod WUF1 {
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
}

/// Power Port A pull-up control register
pub mod PUCRA {

    /// Port A pull-up bit y (y=0..15)
    pub mod PU15 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU10 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-up bit y (y=0..15)
    pub mod PU7 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU6 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU5 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU4 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU3 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU2 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU1 {
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

    /// Port A pull-up bit y (y=0..15)
    pub mod PU0 {
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
}

/// Power Port A pull-down control register
pub mod PDCRA {

    /// Port A pull-down bit y (y=0..15)
    pub mod PD15 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD14 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD13 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD12 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD11 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD10 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD9 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD8 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Port A pull-down bit y (y=0..15)
    pub mod PD7 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD6 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD5 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD4 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD3 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD2 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD1 {
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

    /// Port A pull-down bit y (y=0..15)
    pub mod PD0 {
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
}

/// Power Port B pull-up control register
pub mod PUCRB {
    pub use super::PUCRA::PU0;
    pub use super::PUCRA::PU1;
    pub use super::PUCRA::PU10;
    pub use super::PUCRA::PU11;
    pub use super::PUCRA::PU12;
    pub use super::PUCRA::PU13;
    pub use super::PUCRA::PU14;
    pub use super::PUCRA::PU15;
    pub use super::PUCRA::PU2;
    pub use super::PUCRA::PU3;
    pub use super::PUCRA::PU4;
    pub use super::PUCRA::PU5;
    pub use super::PUCRA::PU6;
    pub use super::PUCRA::PU7;
    pub use super::PUCRA::PU8;
    pub use super::PUCRA::PU9;
}

/// Power Port B pull-down control register
pub mod PDCRB {
    pub use super::PDCRA::PD0;
    pub use super::PDCRA::PD1;
    pub use super::PDCRA::PD10;
    pub use super::PDCRA::PD11;
    pub use super::PDCRA::PD12;
    pub use super::PDCRA::PD13;
    pub use super::PDCRA::PD14;
    pub use super::PDCRA::PD15;
    pub use super::PDCRA::PD2;
    pub use super::PDCRA::PD3;
    pub use super::PDCRA::PD4;
    pub use super::PDCRA::PD5;
    pub use super::PDCRA::PD6;
    pub use super::PDCRA::PD7;
    pub use super::PDCRA::PD8;
    pub use super::PDCRA::PD9;
}

/// Power Port C pull-up control register
pub mod PUCRC {
    pub use super::PUCRA::PU0;
    pub use super::PUCRA::PU1;
    pub use super::PUCRA::PU10;
    pub use super::PUCRA::PU11;
    pub use super::PUCRA::PU12;
    pub use super::PUCRA::PU13;
    pub use super::PUCRA::PU14;
    pub use super::PUCRA::PU15;
    pub use super::PUCRA::PU2;
    pub use super::PUCRA::PU3;
    pub use super::PUCRA::PU4;
    pub use super::PUCRA::PU5;
    pub use super::PUCRA::PU6;
    pub use super::PUCRA::PU7;
    pub use super::PUCRA::PU8;
    pub use super::PUCRA::PU9;
}

/// Power Port C pull-down control register
pub mod PDCRC {
    pub use super::PDCRA::PD0;
    pub use super::PDCRA::PD1;
    pub use super::PDCRA::PD10;
    pub use super::PDCRA::PD11;
    pub use super::PDCRA::PD12;
    pub use super::PDCRA::PD13;
    pub use super::PDCRA::PD14;
    pub use super::PDCRA::PD15;
    pub use super::PDCRA::PD2;
    pub use super::PDCRA::PD3;
    pub use super::PDCRA::PD4;
    pub use super::PDCRA::PD5;
    pub use super::PDCRA::PD6;
    pub use super::PDCRA::PD7;
    pub use super::PDCRA::PD8;
    pub use super::PDCRA::PD9;
}

/// Power Port D pull-up control register
pub mod PUCRD {
    pub use super::PUCRA::PU0;
    pub use super::PUCRA::PU1;
    pub use super::PUCRA::PU10;
    pub use super::PUCRA::PU11;
    pub use super::PUCRA::PU12;
    pub use super::PUCRA::PU13;
    pub use super::PUCRA::PU14;
    pub use super::PUCRA::PU15;
    pub use super::PUCRA::PU2;
    pub use super::PUCRA::PU3;
    pub use super::PUCRA::PU4;
    pub use super::PUCRA::PU5;
    pub use super::PUCRA::PU6;
    pub use super::PUCRA::PU7;
    pub use super::PUCRA::PU8;
    pub use super::PUCRA::PU9;
}

/// Power Port D pull-down control register
pub mod PDCRD {
    pub use super::PDCRA::PD0;
    pub use super::PDCRA::PD1;
    pub use super::PDCRA::PD10;
    pub use super::PDCRA::PD11;
    pub use super::PDCRA::PD12;
    pub use super::PDCRA::PD13;
    pub use super::PDCRA::PD14;
    pub use super::PDCRA::PD15;
    pub use super::PDCRA::PD2;
    pub use super::PDCRA::PD3;
    pub use super::PDCRA::PD4;
    pub use super::PDCRA::PD5;
    pub use super::PDCRA::PD6;
    pub use super::PDCRA::PD7;
    pub use super::PDCRA::PD8;
    pub use super::PDCRA::PD9;
}

/// Power Port E pull-up control register
pub mod PUCRE {
    pub use super::PUCRA::PU0;
    pub use super::PUCRA::PU1;
    pub use super::PUCRA::PU10;
    pub use super::PUCRA::PU11;
    pub use super::PUCRA::PU12;
    pub use super::PUCRA::PU13;
    pub use super::PUCRA::PU14;
    pub use super::PUCRA::PU15;
    pub use super::PUCRA::PU2;
    pub use super::PUCRA::PU3;
    pub use super::PUCRA::PU4;
    pub use super::PUCRA::PU5;
    pub use super::PUCRA::PU6;
    pub use super::PUCRA::PU7;
    pub use super::PUCRA::PU8;
    pub use super::PUCRA::PU9;
}

/// Power Port E pull-down control register
pub mod PDCRE {
    pub use super::PDCRA::PD0;
    pub use super::PDCRA::PD1;
    pub use super::PDCRA::PD10;
    pub use super::PDCRA::PD11;
    pub use super::PDCRA::PD12;
    pub use super::PDCRA::PD13;
    pub use super::PDCRA::PD14;
    pub use super::PDCRA::PD15;
    pub use super::PDCRA::PD2;
    pub use super::PDCRA::PD3;
    pub use super::PDCRA::PD4;
    pub use super::PDCRA::PD5;
    pub use super::PDCRA::PD6;
    pub use super::PDCRA::PD7;
    pub use super::PDCRA::PD8;
    pub use super::PDCRA::PD9;
}

/// Power Port F pull-up control register
pub mod PUCRF {
    pub use super::PUCRA::PU0;
    pub use super::PUCRA::PU1;
    pub use super::PUCRA::PU10;
    pub use super::PUCRA::PU11;
    pub use super::PUCRA::PU12;
    pub use super::PUCRA::PU13;
    pub use super::PUCRA::PU14;
    pub use super::PUCRA::PU15;
    pub use super::PUCRA::PU2;
    pub use super::PUCRA::PU3;
    pub use super::PUCRA::PU4;
    pub use super::PUCRA::PU5;
    pub use super::PUCRA::PU6;
    pub use super::PUCRA::PU7;
    pub use super::PUCRA::PU8;
    pub use super::PUCRA::PU9;
}

/// Power Port F pull-down control register
pub mod PDCRF {
    pub use super::PDCRA::PD0;
    pub use super::PDCRA::PD1;
    pub use super::PDCRA::PD10;
    pub use super::PDCRA::PD11;
    pub use super::PDCRA::PD12;
    pub use super::PDCRA::PD13;
    pub use super::PDCRA::PD14;
    pub use super::PDCRA::PD15;
    pub use super::PDCRA::PD2;
    pub use super::PDCRA::PD3;
    pub use super::PDCRA::PD4;
    pub use super::PDCRA::PD5;
    pub use super::PDCRA::PD6;
    pub use super::PDCRA::PD7;
    pub use super::PDCRA::PD8;
    pub use super::PDCRA::PD9;
}

/// Power Port G pull-up control register
pub mod PUCRG {
    pub use super::PUCRA::PU0;
    pub use super::PUCRA::PU1;
    pub use super::PUCRA::PU10;
    pub use super::PUCRA::PU11;
    pub use super::PUCRA::PU12;
    pub use super::PUCRA::PU13;
    pub use super::PUCRA::PU14;
    pub use super::PUCRA::PU15;
    pub use super::PUCRA::PU2;
    pub use super::PUCRA::PU3;
    pub use super::PUCRA::PU4;
    pub use super::PUCRA::PU5;
    pub use super::PUCRA::PU6;
    pub use super::PUCRA::PU7;
    pub use super::PUCRA::PU8;
    pub use super::PUCRA::PU9;
}

/// Power Port G pull-down control register
pub mod PDCRG {
    pub use super::PDCRA::PD0;
    pub use super::PDCRA::PD1;
    pub use super::PDCRA::PD10;
    pub use super::PDCRA::PD11;
    pub use super::PDCRA::PD12;
    pub use super::PDCRA::PD13;
    pub use super::PDCRA::PD14;
    pub use super::PDCRA::PD15;
    pub use super::PDCRA::PD2;
    pub use super::PDCRA::PD3;
    pub use super::PDCRA::PD4;
    pub use super::PDCRA::PD5;
    pub use super::PDCRA::PD6;
    pub use super::PDCRA::PD7;
    pub use super::PDCRA::PD8;
    pub use super::PDCRA::PD9;
}

/// Power Port H pull-up control register
pub mod PUCRH {

    /// Port H pull-up bit y (y=0..1)
    pub mod PU1 {
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

    /// Port H pull-up bit y (y=0..1)
    pub mod PU0 {
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
}

/// Power Port H pull-down control register
pub mod PDCRH {

    /// Port H pull-down bit y (y=0..1)
    pub mod PD1 {
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

    /// Port H pull-down bit y (y=0..1)
    pub mod PD0 {
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
}
#[repr(C)]
pub struct RegisterBlock {
    /// Power control register 1
    pub CR1: RWRegister<u32>,

    /// Power control register 2
    pub CR2: RWRegister<u32>,

    /// Power control register 3
    pub CR3: RWRegister<u32>,

    /// Power control register 4
    pub CR4: RWRegister<u32>,

    /// Power status register 1
    pub SR1: RORegister<u32>,

    /// Power status register 2
    pub SR2: RORegister<u32>,

    /// Power status clear register
    pub SCR: WORegister<u32>,

    _reserved1: [u32; 1],

    /// Power Port A pull-up control register
    pub PUCRA: RWRegister<u32>,

    /// Power Port A pull-down control register
    pub PDCRA: RWRegister<u32>,

    /// Power Port B pull-up control register
    pub PUCRB: RWRegister<u32>,

    /// Power Port B pull-down control register
    pub PDCRB: RWRegister<u32>,

    /// Power Port C pull-up control register
    pub PUCRC: RWRegister<u32>,

    /// Power Port C pull-down control register
    pub PDCRC: RWRegister<u32>,

    /// Power Port D pull-up control register
    pub PUCRD: RWRegister<u32>,

    /// Power Port D pull-down control register
    pub PDCRD: RWRegister<u32>,

    /// Power Port E pull-up control register
    pub PUCRE: RWRegister<u32>,

    /// Power Port E pull-down control register
    pub PDCRE: RWRegister<u32>,

    /// Power Port F pull-up control register
    pub PUCRF: RWRegister<u32>,

    /// Power Port F pull-down control register
    pub PDCRF: RWRegister<u32>,

    /// Power Port G pull-up control register
    pub PUCRG: RWRegister<u32>,

    /// Power Port G pull-down control register
    pub PDCRG: RWRegister<u32>,

    /// Power Port H pull-up control register
    pub PUCRH: RWRegister<u32>,

    /// Power Port H pull-down control register
    pub PDCRH: RWRegister<u32>,
}
pub struct ResetValues {
    pub CR1: u32,
    pub CR2: u32,
    pub CR3: u32,
    pub CR4: u32,
    pub SR1: u32,
    pub SR2: u32,
    pub SCR: u32,
    pub PUCRA: u32,
    pub PDCRA: u32,
    pub PUCRB: u32,
    pub PDCRB: u32,
    pub PUCRC: u32,
    pub PDCRC: u32,
    pub PUCRD: u32,
    pub PDCRD: u32,
    pub PUCRE: u32,
    pub PDCRE: u32,
    pub PUCRF: u32,
    pub PDCRF: u32,
    pub PUCRG: u32,
    pub PDCRG: u32,
    pub PUCRH: u32,
    pub PDCRH: u32,
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
