#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Cyclic redundancy check calculation unit
//!
//! Used by: stm32g030, stm32g031, stm32g041, stm32g070, stm32g071, stm32g07x, stm32g081

#[cfg(not(feature = "nosync"))]
pub use crate::stm32g0::peripherals::crc_v1::Instance;
pub use crate::stm32g0::peripherals::crc_v1::{RegisterBlock, ResetValues};
pub use crate::stm32g0::peripherals::crc_v1::{CR, DR, IDR, INIT, POL};

/// Access functions for the CRC peripheral instance
pub mod CRC {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40023000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in CRC
    pub const reset: ResetValues = ResetValues {
        DR: 0xFFFFFFFF,
        IDR: 0x00000000,
        CR: 0x00000000,
        INIT: 0xFFFFFFFF,
        POL: 0x04C11DB7,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut CRC_TAKEN: bool = false;

    /// Safe access to CRC
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
            if CRC_TAKEN {
                None
            } else {
                CRC_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to CRC
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if CRC_TAKEN && inst.addr == INSTANCE.addr {
                CRC_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal CRC
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        CRC_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to CRC
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const CRC: *const RegisterBlock = 0x40023000 as *const _;
