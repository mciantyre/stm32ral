#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Serial audio interface
//!
//! Used by: stm32g431, stm32g441, stm32g471, stm32g473, stm32g474, stm32g483, stm32g484

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g4::peripherals::sai_v1::Instance;
pub use crate::stm32g4::peripherals::sai_v1::{RegisterBlock, ResetValues};
pub use crate::stm32g4::peripherals::sai_v1::{
    CLRFRA, CLRFRB, CR1A, CR1B, CR2A, CR2B, DRA, DRB, FRCRA, FRCRB, IMA, IMB, PDMCR, PDMDLY,
    SLOTRA, SLOTRB, SRA, SRB,
};

/// Access functions for the SAI peripheral instance
pub mod SAI {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40015400,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in SAI
    pub const reset: ResetValues = ResetValues {
        PDMCR: 0x00000000,
        PDMDLY: 0x00000000,
        CR1A: 0x00000040,
        CR2A: 0x00000000,
        FRCRA: 0x00000007,
        SLOTRA: 0x00000000,
        IMA: 0x00000000,
        SRA: 0x00000000,
        CLRFRA: 0x00000000,
        DRA: 0x00000000,
        CR1B: 0x00000040,
        CR2B: 0x00000000,
        FRCRB: 0x00000007,
        SLOTRB: 0x00000000,
        IMB: 0x00000000,
        SRB: 0x00000000,
        CLRFRB: 0x00000000,
        DRB: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut SAI_TAKEN: bool = false;

    /// Safe access to SAI
    ///
    /// This function returns `Some(Instance)` if this instance is not
    /// currently taken, and `None` if it is. This ensures that if you
    /// do get `Some(Instance)`, you are ensured unique access to
    /// the peripheral and there cannot be data races (unless other
    /// code uses `unsafe`, of course). You can then pass the
    /// `Instance` around to other functions as required. When you're
    /// done with it, you can call `release(instance)` to return it.
    ///
    /// `Instance` itself dereferences to a `RegisterBlock`, which
    /// provides access to the peripheral's registers.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn take() -> Option<Instance> {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI_TAKEN {
                None
            } else {
                SAI_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to SAI
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if SAI_TAKEN && inst.addr == INSTANCE.addr {
                SAI_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal SAI
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        SAI_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to SAI
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const SAI: *const RegisterBlock = 0x40015400 as *const _;
