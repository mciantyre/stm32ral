#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Floating point unit CPACR
//!
//! Used by: stm32l412, stm32l4r9, stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

use crate::RWRegister;
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// Coprocessor access control register
pub mod CPACR {

    /// CP
    pub mod CP {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (4 bits: 0b1111 << 20)
        pub const mask: u32 = 0b1111 << offset;
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
    /// Coprocessor access control register
    pub CPACR: RWRegister<u32>,
}
pub struct ResetValues {
    pub CPACR: u32,
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
