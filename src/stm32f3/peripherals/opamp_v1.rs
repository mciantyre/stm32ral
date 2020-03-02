#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Operational Amplifier
//!
//! Used by: stm32f301, stm32f3x4

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// OPAMP2 control register
pub mod OPAMP2_CSR {

    /// OPAMP2 enable
    pub mod OPAMP2EN {
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

            /// 0b0: OPAMP2 is disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: OPAMP2 is enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// FORCE_VP
    pub mod FORCE_VP {
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

            /// 0b0: Normal operating mode
            pub const Normal: u32 = 0b0;

            /// 0b1: Calibration mode. Non-inverting input connected to calibration reference
            pub const Calibration: u32 = 0b1;
        }
    }

    /// OPAMP Non inverting input selection
    pub mod VP_SEL {
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

            /// 0b01: PB14 used as OPAMP2 non-inverting input
            pub const PB14: u32 = 0b01;

            /// 0b10: PB0 used as OPAMP2 non-inverting input
            pub const PB0: u32 = 0b10;

            /// 0b11: PA7 used as OPAMP2 non-invertign input
            pub const PA7: u32 = 0b11;
        }
    }

    /// OPAMP inverting input selection
    pub mod VM_SEL {
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

            /// 0b00: PC5 (VM0) used as OPAMP2 inverting input
            pub const PC5: u32 = 0b00;

            /// 0b01: PA5 (VM1) used as OPAMP2 inverting input
            pub const PA5: u32 = 0b01;

            /// 0b10: Resistor feedback output (PGA mode)
            pub const PGA: u32 = 0b10;

            /// 0b11: Follower mode
            pub const Follower: u32 = 0b11;
        }
    }

    /// Timer controlled Mux mode enable
    pub mod TCM_EN {
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

            /// 0b0: Timer controlled mux disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Timer controlled mux enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// OPAMP inverting input secondary selection
    pub mod VMS_SEL {
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

            /// 0b0: PC5 (VM0) used as OPAMP2 inverting input when TCM_EN=1
            pub const PC5: u32 = 0b0;

            /// 0b1: PA5 (VM1) used as OPAMP2 inverting input when TCM_EN=1
            pub const PA5: u32 = 0b1;
        }
    }

    /// OPAMP Non inverting input secondary selection
    pub mod VPS_SEL {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (2 bits: 0b11 << 9)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b01: PB14 used as OPAMP2 non-inverting input when TCM_EN=1
            pub const PB14: u32 = 0b01;

            /// 0b10: PB0 used as OPAMP2 non-inverting input when TCM_EN=1
            pub const PB0: u32 = 0b10;

            /// 0b11: PA7 used as OPAMP2 non-invertign input when TCM_EN=1
            pub const PA7: u32 = 0b11;
        }
    }

    /// Calibration mode enable
    pub mod CALON {
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

            /// 0b0: Calibration mode disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Calibration mode enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Calibration selection
    pub mod CALSEL {
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

            /// 0b00: VREFOPAMP=3.3% VDDA
            pub const Percent3_3: u32 = 0b00;

            /// 0b01: VREFOPAMP=10% VDDA
            pub const Percent10: u32 = 0b01;

            /// 0b10: VREFOPAMP=50% VDDA
            pub const Percent50: u32 = 0b10;

            /// 0b11: VREFOPAMP=90% VDDA
            pub const Percent90: u32 = 0b11;
        }
    }

    /// Gain in PGA mode
    pub mod PGA_GAIN {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (4 bits: 0b1111 << 14)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: Gain 2
            pub const Gain2: u32 = 0b0000;

            /// 0b0001: Gain 4
            pub const Gain4: u32 = 0b0001;

            /// 0b0010: Gain 8
            pub const Gain8: u32 = 0b0010;

            /// 0b0100: Gain 16
            pub const Gain16: u32 = 0b0100;

            /// 0b1000: Gain 2, feedback connected to VM0
            pub const Gain2_VM0: u32 = 0b1000;

            /// 0b1001: Gain 4, feedback connected to VM0
            pub const Gain4_VM0: u32 = 0b1001;

            /// 0b1010: Gain 8, feedback connected to VM0
            pub const Gain8_VM0: u32 = 0b1010;

            /// 0b1011: Gain 16, feedback connected to VM0
            pub const Gain16_VM0: u32 = 0b1011;

            /// 0b1100: Gain 2, feedback connected to VM1
            pub const Gain2_VM1: u32 = 0b1100;

            /// 0b1101: Gain 4, feedback connected to VM1
            pub const Gain4_VM1: u32 = 0b1101;

            /// 0b1110: Gain 8, feedback connected to VM1
            pub const Gain8_VM1: u32 = 0b1110;

            /// 0b1111: Gain 16, feedback connected to VM1
            pub const Gain16_VM1: u32 = 0b1111;
        }
    }

    /// User trimming enable
    pub mod USER_TRIM {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: User trimming disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: User trimming enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// Offset trimming value (PMOS)
    pub mod TRIMOFFSETP {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (5 bits: 0b11111 << 19)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// Offset trimming value (NMOS)
    pub mod TRIMOFFSETN {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (5 bits: 0b11111 << 24)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// TSTREF
    pub mod TSTREF {
        /// Offset (29 bits)
        pub const offset: u32 = 29;
        /// Mask (1 bit: 1 << 29)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: VREFOPAMP2 is output
            pub const Output: u32 = 0b0;

            /// 0b1: VREFOPAMP2 is not output
            pub const NotOutput: u32 = 0b1;
        }
    }

    /// OPAMP ouput status flag
    pub mod OUTCAL {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Non-inverting < inverting
            pub const Low: u32 = 0b0;

            /// 0b1: Non-inverting > inverting
            pub const High: u32 = 0b1;
        }
    }

    /// OPAMP lock
    pub mod LOCK {
        /// Offset (31 bits)
        pub const offset: u32 = 31;
        /// Mask (1 bit: 1 << 31)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: Comparator CSR bits are read-write
            pub const Unlocked: u32 = 0b0;

            /// 0b1: Comparator CSR bits are read-only
            pub const Locked: u32 = 0b1;
        }
    }
}
#[repr(C)]
pub struct RegisterBlock {
    _reserved1: [u32; 15],

    /// OPAMP2 control register
    pub OPAMP2_CSR: RWRegister<u32>,
}
pub struct ResetValues {
    pub OPAMP2_CSR: u32,
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
#[cfg(feature = "rtfm")]
unsafe impl Send for Instance {}
