#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! WWDG1
//!
//! Used by: stm32mp153, stm32mp157

#[cfg(not(feature = "nosync"))]
pub use crate::stm32mp::peripherals::wwdg1::Instance;
pub use crate::stm32mp::peripherals::wwdg1::{RegisterBlock, ResetValues};
pub use crate::stm32mp::peripherals::wwdg1::{
    WWDG_CFR, WWDG_CR, WWDG_HWCFGR, WWDG_IPIDR, WWDG_SIDR, WWDG_SR, WWDG_VERR,
};

/// Access functions for the WWDG1 peripheral instance
pub mod WWDG1 {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x4000a000,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in WWDG1
    pub const reset: ResetValues = ResetValues {
        WWDG_CR: 0x0000007F,
        WWDG_CFR: 0x0000007F,
        WWDG_SR: 0x00000000,
        WWDG_HWCFGR: 0x00000FFF,
        WWDG_VERR: 0x00000021,
        WWDG_IPIDR: 0x00120051,
        WWDG_SIDR: 0xA3C5DD01,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut WWDG1_TAKEN: bool = false;

    /// Safe access to WWDG1
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
            if WWDG1_TAKEN {
                None
            } else {
                WWDG1_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to WWDG1
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if WWDG1_TAKEN && inst.addr == INSTANCE.addr {
                WWDG1_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal WWDG1
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        WWDG1_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to WWDG1
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const WWDG1: *const RegisterBlock = 0x4000a000 as *const _;
