#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Firewall
//!
//! Used by: stm32l412, stm32l4r9, stm32l4x1, stm32l4x2, stm32l4x3, stm32l4x5, stm32l4x6

#[cfg(not(feature = "nosync"))]
pub use crate::stm32l4::peripherals::firewall::Instance;
pub use crate::stm32l4::peripherals::firewall::{RegisterBlock, ResetValues};
pub use crate::stm32l4::peripherals::firewall::{CR, CSL, CSSA, NVDSL, NVDSSA, VDSL, VDSSA};

/// Access functions for the FIREWALL peripheral instance
pub mod FIREWALL {
    use super::ResetValues;

    #[cfg(not(feature = "nosync"))]
    use super::Instance;

    #[cfg(not(feature = "nosync"))]
    const INSTANCE: Instance = Instance {
        addr: 0x40011c00,
        _marker: ::core::marker::PhantomData,
    };

    /// Reset values for each field in FIREWALL
    pub const reset: ResetValues = ResetValues {
        CSSA: 0x00000000,
        CSL: 0x00000000,
        NVDSSA: 0x00000000,
        NVDSL: 0x00000000,
        VDSSA: 0x00000000,
        VDSL: 0x00000000,
        CR: 0x00000000,
    };

    #[cfg(not(feature = "nosync"))]
    #[allow(renamed_and_removed_lints)]
    #[allow(private_no_mangle_statics)]
    #[no_mangle]
    static mut FIREWALL_TAKEN: bool = false;

    /// Safe access to FIREWALL
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
            if FIREWALL_TAKEN {
                None
            } else {
                FIREWALL_TAKEN = true;
                Some(INSTANCE)
            }
        })
    }

    /// Release exclusive access to FIREWALL
    ///
    /// This function allows you to return an `Instance` so that it
    /// is available to `take()` again. This function will panic if
    /// you return a different `Instance` or if this instance is not
    /// already taken.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub fn release(inst: Instance) {
        external_cortex_m::interrupt::free(|_| unsafe {
            if FIREWALL_TAKEN && inst.addr == INSTANCE.addr {
                FIREWALL_TAKEN = false;
            } else {
                panic!("Released a peripheral which was not taken");
            }
        });
    }

    /// Unsafely steal FIREWALL
    ///
    /// This function is similar to take() but forcibly takes the
    /// Instance, marking it as taken irregardless of its previous
    /// state.
    #[cfg(not(feature = "nosync"))]
    #[inline]
    pub unsafe fn steal() -> Instance {
        FIREWALL_TAKEN = true;
        INSTANCE
    }
}

/// Raw pointer to FIREWALL
///
/// Dereferencing this is unsafe because you are not ensured unique
/// access to the peripheral, so you may encounter data races with
/// other users of this peripheral. It is up to you to ensure you
/// will not cause data races.
///
/// This constant is provided for ease of use in unsafe code: you can
/// simply call for example `write_reg!(gpio, GPIOA, ODR, 1);`.
pub const FIREWALL: *const RegisterBlock = 0x40011c00 as *const _;
