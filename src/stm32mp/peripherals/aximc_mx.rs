#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! AXIMC_Mx
//!
//! Used by: stm32mp153, stm32mp157

use crate::{RORegister, RWRegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// AXIMC master 0 packing functionality register
pub mod AXIMC_M0_FN_MOD2 {

    /// BYPASS_MERGE
    pub mod BYPASS_MERGE {
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

/// AXIMC master 0 read priority register
pub mod AXIMC_M0_READ_QOS {

    /// AR_QOS
    pub mod AR_QOS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC master 0 write priority register
pub mod AXIMC_M0_WRITE_QOS {

    /// AW_QOS
    pub mod AW_QOS {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC master 0 issuing capability override functionality register
pub mod AXIMC_M0_FN_MOD {

    /// READ_ISS_OVERRIDE
    pub mod READ_ISS_OVERRIDE {
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

    /// WRITE_ISS_OVERRIDE
    pub mod WRITE_ISS_OVERRIDE {
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
}

/// AXIMC master 1 packing functionality register
pub mod AXIMC_M1_FN_MOD2 {
    pub use super::AXIMC_M0_FN_MOD2::BYPASS_MERGE;
}

/// AXIMC master 1 read priority register
pub mod AXIMC_M1_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 1 write priority register
pub mod AXIMC_M1_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 1 issuing capability override functionality register
pub mod AXIMC_M1_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 2 packing functionality register
pub mod AXIMC_M2_FN_MOD2 {
    pub use super::AXIMC_M0_FN_MOD2::BYPASS_MERGE;
}

/// AXIMC master 2 read priority register
pub mod AXIMC_M2_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 2 write priority register
pub mod AXIMC_M2_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 2 issuing capability override functionality register
pub mod AXIMC_M2_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 5 packing functionality register
pub mod AXIMC_M5_FN_MOD2 {
    pub use super::AXIMC_M0_FN_MOD2::BYPASS_MERGE;
}

/// AXIMC master 5 read priority register
pub mod AXIMC_M5_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 5 write priority register
pub mod AXIMC_M5_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 5 issuing capability override functionality register
pub mod AXIMC_M5_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 3 read priority register
pub mod AXIMC_M3_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 3 write priority register
pub mod AXIMC_M3_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 3 packing functionality register
pub mod AXIMC_M3_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 7 read priority register
pub mod AXIMC_M7_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 7 write priority register
pub mod AXIMC_M7_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 7 issuing capability override functionality register
pub mod AXIMC_M7_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 8 read priority register
pub mod AXIMC_M8_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 8 write priority register
pub mod AXIMC_M8_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 8 issuing capability override functionality register
pub mod AXIMC_M8_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 4 packing functionality register
pub mod AXIMC_M4_FN_MOD2 {
    pub use super::AXIMC_M0_FN_MOD2::BYPASS_MERGE;
}

/// AXIMC master 4 read priority register
pub mod AXIMC_M4_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 4 write priority register
pub mod AXIMC_M4_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 4 packing functionality register
pub mod AXIMC_M4_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 9 read priority register
pub mod AXIMC_M9_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 9 write priority register
pub mod AXIMC_M9_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 9 issuing capability override functionality register
pub mod AXIMC_M9_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 10 read priority register
pub mod AXIMC_M10_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 10 write priority register
pub mod AXIMC_M10_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 10 issuing capability override functionality register
pub mod AXIMC_M10_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC master 6 packing functionality register
pub mod AXIMC_M6_FN_MOD2 {
    pub use super::AXIMC_M0_FN_MOD2::BYPASS_MERGE;
}

/// AXIMC master 6 read priority register
pub mod AXIMC_M6_READ_QOS {
    pub use super::AXIMC_M0_READ_QOS::AR_QOS;
}

/// AXIMC master 6 write priority register
pub mod AXIMC_M6_WRITE_QOS {
    pub use super::AXIMC_M0_WRITE_QOS::AW_QOS;
}

/// AXIMC master 6 issuing capability override functionality register
pub mod AXIMC_M6_FN_MOD {
    pub use super::AXIMC_M0_FN_MOD::READ_ISS_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD::WRITE_ISS_OVERRIDE;
}

/// AXIMC peripheral ID4 register
pub mod AXIMC_PERIPH_ID_4 {

    /// JEP106CON
    pub mod JEP106CON {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// K4COUNT
    pub mod K4COUNT {
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
}

/// AXIMC peripheral ID5 register
pub mod AXIMC_PERIPH_ID_5 {

    /// PERIPH_ID_5
    pub mod PERIPH_ID_5 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC peripheral ID6 register
pub mod AXIMC_PERIPH_ID_6 {

    /// PERIPH_ID_6
    pub mod PERIPH_ID_6 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC peripheral ID7 register
pub mod AXIMC_PERIPH_ID_7 {

    /// PERIPH_ID_7
    pub mod PERIPH_ID_7 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC peripheral ID0 register
pub mod AXIMC_PERIPH_ID_0 {

    /// PERIPH_ID_0
    pub mod PERIPH_ID_0 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC peripheral ID1 register
pub mod AXIMC_PERIPH_ID_1 {

    /// PERIPH_ID_1
    pub mod PERIPH_ID_1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC peripheral ID2 register
pub mod AXIMC_PERIPH_ID_2 {

    /// PERIPH_ID_2
    pub mod PERIPH_ID_2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC peripheral ID3 register
pub mod AXIMC_PERIPH_ID_3 {

    /// CUST_MOD_NUM
    pub mod CUST_MOD_NUM {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// REV_AND
    pub mod REV_AND {
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
}

/// AXIMC component ID0 register
pub mod AXIMC_COMP_ID_0 {

    /// PREAMBLE
    pub mod PREAMBLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (8 bits: 0xff << 0)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// AXIMC component ID1 register
pub mod AXIMC_COMP_ID_1 {

    /// PREAMBLE
    pub mod PREAMBLE {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (4 bits: 0b1111 << 0)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// CLASS
    pub mod CLASS {
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
}

/// AXIMC component ID2 register
pub mod AXIMC_COMP_ID_2 {
    pub use super::AXIMC_COMP_ID_0::PREAMBLE;
}

/// AXIMC component ID3 register
pub mod AXIMC_COMP_ID_3 {
    pub use super::AXIMC_COMP_ID_0::PREAMBLE;
}

/// AXIMC master 0 AHB conversion override functionality register
pub mod AXIMC_M0_FN_MOD_AHB {

    /// RD_INC_OVERRIDE
    pub mod RD_INC_OVERRIDE {
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

    /// WR_INC_OVERRIDE
    pub mod WR_INC_OVERRIDE {
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
}

/// AXIMC master 1 AHB conversion override functionality register
pub mod AXIMC_M1_FN_MOD_AHB {
    pub use super::AXIMC_M0_FN_MOD_AHB::RD_INC_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD_AHB::WR_INC_OVERRIDE;
}

/// AXIMC master 2 AHB conversion override functionality register
pub mod AXIMC_M2_FN_MOD_AHB {
    pub use super::AXIMC_M0_FN_MOD_AHB::RD_INC_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD_AHB::WR_INC_OVERRIDE;
}

/// AXIMC master 5 AHB conversion override functionality register
pub mod AXIMC_M5_FN_MOD_AHB {
    pub use super::AXIMC_M0_FN_MOD_AHB::RD_INC_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD_AHB::WR_INC_OVERRIDE;
}

/// AXIMC master 6 AHB conversion override functionality register
pub mod AXIMC_M6_FN_MOD_AHB {
    pub use super::AXIMC_M0_FN_MOD_AHB::RD_INC_OVERRIDE;
    pub use super::AXIMC_M0_FN_MOD_AHB::WR_INC_OVERRIDE;
}

/// AXIMC long burst capability inhibition register
pub mod AXIMC_FN_MOD_LB {

    /// FN_MOD_LB
    pub mod FN_MOD_LB {
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
    /// AXIMC master 0 packing functionality register
    pub AXIMC_M0_FN_MOD2: RWRegister<u32>,

    _reserved1: [u32; 54],

    /// AXIMC master 0 read priority register
    pub AXIMC_M0_READ_QOS: RWRegister<u32>,

    /// AXIMC master 0 issuing capability override functionality register
    pub AXIMC_M0_FN_MOD: RWRegister<u32>,

    /// AXIMC master 0 write priority register
    pub AXIMC_M0_WRITE_QOS: RWRegister<u32>,

    _reserved2: [u32; 966],

    /// AXIMC master 1 packing functionality register
    pub AXIMC_M1_FN_MOD2: RWRegister<u32>,

    _reserved3: [u32; 54],

    /// AXIMC master 1 read priority register
    pub AXIMC_M1_READ_QOS: RWRegister<u32>,

    /// AXIMC master 1 write priority register
    pub AXIMC_M1_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 1 issuing capability override functionality register
    pub AXIMC_M1_FN_MOD: RWRegister<u32>,

    _reserved4: [u32; 954],

    /// AXIMC peripheral ID4 register
    pub AXIMC_PERIPH_ID_4: RORegister<u32>,

    /// AXIMC peripheral ID5 register
    pub AXIMC_PERIPH_ID_5: RORegister<u32>,

    /// AXIMC peripheral ID6 register
    pub AXIMC_PERIPH_ID_6: RORegister<u32>,

    /// AXIMC peripheral ID7 register
    pub AXIMC_PERIPH_ID_7: RORegister<u32>,

    /// AXIMC peripheral ID0 register
    pub AXIMC_PERIPH_ID_0: RORegister<u32>,

    /// AXIMC peripheral ID1 register
    pub AXIMC_PERIPH_ID_1: RORegister<u32>,

    /// AXIMC peripheral ID2 register
    pub AXIMC_PERIPH_ID_2: RORegister<u32>,

    /// AXIMC peripheral ID3 register
    pub AXIMC_PERIPH_ID_3: RORegister<u32>,

    /// AXIMC component ID0 register
    pub AXIMC_COMP_ID_0: RORegister<u32>,

    /// AXIMC component ID1 register
    pub AXIMC_COMP_ID_1: RORegister<u32>,

    /// AXIMC component ID2 register
    pub AXIMC_COMP_ID_2: RORegister<u32>,

    /// AXIMC component ID3 register
    pub AXIMC_COMP_ID_3: RORegister<u32>,

    /// AXIMC master 2 packing functionality register
    pub AXIMC_M2_FN_MOD2: RWRegister<u32>,

    _reserved5: [u32; 54],

    /// AXIMC master 2 read priority register
    pub AXIMC_M2_READ_QOS: RWRegister<u32>,

    /// AXIMC master 2 write priority register
    pub AXIMC_M2_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 2 issuing capability override functionality register
    pub AXIMC_M2_FN_MOD: RWRegister<u32>,

    _reserved6: [u32; 966],

    /// AXIMC master 5 packing functionality register
    pub AXIMC_M5_FN_MOD2: RWRegister<u32>,

    _reserved7: [u32; 54],

    /// AXIMC master 5 read priority register
    pub AXIMC_M5_READ_QOS: RWRegister<u32>,

    /// AXIMC master 5 write priority register
    pub AXIMC_M5_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 5 issuing capability override functionality register
    pub AXIMC_M5_FN_MOD: RWRegister<u32>,

    _reserved8: [u32; 1021],

    /// AXIMC master 3 read priority register
    pub AXIMC_M3_READ_QOS: RWRegister<u32>,

    /// AXIMC master 3 write priority register
    pub AXIMC_M3_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 3 packing functionality register
    pub AXIMC_M3_FN_MOD: RWRegister<u32>,

    _reserved9: [u32; 1021],

    /// AXIMC master 7 read priority register
    pub AXIMC_M7_READ_QOS: RWRegister<u32>,

    /// AXIMC master 7 write priority register
    pub AXIMC_M7_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 7 issuing capability override functionality register
    pub AXIMC_M7_FN_MOD: RWRegister<u32>,

    _reserved10: [u32; 1021],

    /// AXIMC master 8 read priority register
    pub AXIMC_M8_READ_QOS: RWRegister<u32>,

    /// AXIMC master 8 write priority register
    pub AXIMC_M8_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 8 issuing capability override functionality register
    pub AXIMC_M8_FN_MOD: RWRegister<u32>,

    _reserved11: [u32; 1990],

    /// AXIMC master 4 packing functionality register
    pub AXIMC_M4_FN_MOD2: RWRegister<u32>,

    _reserved12: [u32; 54],

    /// AXIMC master 4 read priority register
    pub AXIMC_M4_READ_QOS: RWRegister<u32>,

    /// AXIMC master 4 write priority register
    pub AXIMC_M4_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 4 packing functionality register
    pub AXIMC_M4_FN_MOD: RWRegister<u32>,

    _reserved13: [u32; 1021],

    /// AXIMC master 9 read priority register
    pub AXIMC_M9_READ_QOS: RWRegister<u32>,

    /// AXIMC master 9 write priority register
    pub AXIMC_M9_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 9 issuing capability override functionality register
    pub AXIMC_M9_FN_MOD: RWRegister<u32>,

    _reserved14: [u32; 1021],

    /// AXIMC master 10 read priority register
    pub AXIMC_M10_READ_QOS: RWRegister<u32>,

    /// AXIMC master 10 write priority register
    pub AXIMC_M10_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 10 issuing capability override functionality register
    pub AXIMC_M10_FN_MOD: RWRegister<u32>,

    _reserved15: [u32; 966],

    /// AXIMC master 6 packing functionality register
    pub AXIMC_M6_FN_MOD2: RWRegister<u32>,

    _reserved16: [u32; 54],

    /// AXIMC master 6 read priority register
    pub AXIMC_M6_READ_QOS: RWRegister<u32>,

    /// AXIMC master 6 write priority register
    pub AXIMC_M6_WRITE_QOS: RWRegister<u32>,

    /// AXIMC master 6 issuing capability override functionality register
    pub AXIMC_M6_FN_MOD: RWRegister<u32>,

    _reserved17: [u32; 56272],

    /// AXIMC master 0 AHB conversion override functionality register
    pub AXIMC_M0_FN_MOD_AHB: RWRegister<u32>,

    _reserved18: [u32; 1023],

    /// AXIMC master 1 AHB conversion override functionality register
    pub AXIMC_M1_FN_MOD_AHB: RWRegister<u32>,

    _reserved19: [u32; 1023],

    /// AXIMC master 2 AHB conversion override functionality register
    pub AXIMC_M2_FN_MOD_AHB: RWRegister<u32>,

    _reserved20: [u32; 1023],

    /// AXIMC master 5 AHB conversion override functionality register
    pub AXIMC_M5_FN_MOD_AHB: RWRegister<u32>,

    _reserved21: [u32; 5120],

    /// AXIMC long burst capability inhibition register
    pub AXIMC_FN_MOD_LB: RWRegister<u32>,

    _reserved22: [u32; 3070],

    /// AXIMC master 6 AHB conversion override functionality register
    pub AXIMC_M6_FN_MOD_AHB: RWRegister<u32>,
}
pub struct ResetValues {
    pub AXIMC_M0_FN_MOD2: u32,
    pub AXIMC_M0_READ_QOS: u32,
    pub AXIMC_M0_FN_MOD: u32,
    pub AXIMC_M0_WRITE_QOS: u32,
    pub AXIMC_M1_FN_MOD2: u32,
    pub AXIMC_M1_READ_QOS: u32,
    pub AXIMC_M1_WRITE_QOS: u32,
    pub AXIMC_M1_FN_MOD: u32,
    pub AXIMC_PERIPH_ID_4: u32,
    pub AXIMC_PERIPH_ID_5: u32,
    pub AXIMC_PERIPH_ID_6: u32,
    pub AXIMC_PERIPH_ID_7: u32,
    pub AXIMC_PERIPH_ID_0: u32,
    pub AXIMC_PERIPH_ID_1: u32,
    pub AXIMC_PERIPH_ID_2: u32,
    pub AXIMC_PERIPH_ID_3: u32,
    pub AXIMC_COMP_ID_0: u32,
    pub AXIMC_COMP_ID_1: u32,
    pub AXIMC_COMP_ID_2: u32,
    pub AXIMC_COMP_ID_3: u32,
    pub AXIMC_M2_FN_MOD2: u32,
    pub AXIMC_M2_READ_QOS: u32,
    pub AXIMC_M2_WRITE_QOS: u32,
    pub AXIMC_M2_FN_MOD: u32,
    pub AXIMC_M5_FN_MOD2: u32,
    pub AXIMC_M5_READ_QOS: u32,
    pub AXIMC_M5_WRITE_QOS: u32,
    pub AXIMC_M5_FN_MOD: u32,
    pub AXIMC_M3_READ_QOS: u32,
    pub AXIMC_M3_WRITE_QOS: u32,
    pub AXIMC_M3_FN_MOD: u32,
    pub AXIMC_M7_READ_QOS: u32,
    pub AXIMC_M7_WRITE_QOS: u32,
    pub AXIMC_M7_FN_MOD: u32,
    pub AXIMC_M8_READ_QOS: u32,
    pub AXIMC_M8_WRITE_QOS: u32,
    pub AXIMC_M8_FN_MOD: u32,
    pub AXIMC_M4_FN_MOD2: u32,
    pub AXIMC_M4_READ_QOS: u32,
    pub AXIMC_M4_WRITE_QOS: u32,
    pub AXIMC_M4_FN_MOD: u32,
    pub AXIMC_M9_READ_QOS: u32,
    pub AXIMC_M9_WRITE_QOS: u32,
    pub AXIMC_M9_FN_MOD: u32,
    pub AXIMC_M10_READ_QOS: u32,
    pub AXIMC_M10_WRITE_QOS: u32,
    pub AXIMC_M10_FN_MOD: u32,
    pub AXIMC_M6_FN_MOD2: u32,
    pub AXIMC_M6_READ_QOS: u32,
    pub AXIMC_M6_WRITE_QOS: u32,
    pub AXIMC_M6_FN_MOD: u32,
    pub AXIMC_M0_FN_MOD_AHB: u32,
    pub AXIMC_M1_FN_MOD_AHB: u32,
    pub AXIMC_M2_FN_MOD_AHB: u32,
    pub AXIMC_M5_FN_MOD_AHB: u32,
    pub AXIMC_FN_MOD_LB: u32,
    pub AXIMC_M6_FN_MOD_AHB: u32,
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
