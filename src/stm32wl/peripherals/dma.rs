#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! Direct memory access controller
//!
//! Used by: stm32wl5x_cm0p, stm32wl5x_cm4, stm32wle5

use crate::{RORegister, RWRegister, UnsafeRWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// interrupt status register
pub mod ISR {

    /// transfer error (TE) flag for channel 7
    pub mod TEIF7 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No transfer error on channel x
            pub const NoError: u32 = 0b0;

            /// 0b1: A transfer error occurred on channel x
            pub const Error: u32 = 0b1;
        }
    }

    /// half transfer (HT) flag for channel 7
    pub mod HTIF7 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No half transfer event on channel x
            pub const NotHalf: u32 = 0b0;

            /// 0b1: A half transfer event occurred on channel x
            pub const Half: u32 = 0b1;
        }
    }

    /// transfer complete (TC) flag for channel 7
    pub mod TCIF7 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No transfer complete event on channel x
            pub const NotComplete: u32 = 0b0;

            /// 0b1: A transfer complete event occurred on channel x
            pub const Complete: u32 = 0b1;
        }
    }

    /// global interrupt flag for channel 7
    pub mod GIF7 {
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

            /// 0b0: No TE, HT or TC event on channel x
            pub const NoEvent: u32 = 0b0;

            /// 0b1: A TE, HT or TC event occurred on channel x
            pub const Event: u32 = 0b1;
        }
    }

    /// transfer error (TE) flag for channel 6
    pub mod TEIF6 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer (HT) flag for channel 6
    pub mod HTIF6 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete (TC) flag for channel 6
    pub mod TCIF6 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag for channel 6
    pub mod GIF6 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error (TE) flag for channel 5
    pub mod TEIF5 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer (HT) flag for channel 5
    pub mod HTIF5 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete (TC) flag for channel 5
    pub mod TCIF5 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag for channel 5
    pub mod GIF5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error (TE) flag for channel 4
    pub mod TEIF4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer (HT) flag for channel 4
    pub mod HTIF4 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete (TC) flag for channel 4
    pub mod TCIF4 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag for channel 4
    pub mod GIF4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error (TE) flag for channel 3
    pub mod TEIF3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer (HT) flag for channel 3
    pub mod HTIF3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete (TC) flag for channel 3
    pub mod TCIF3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag for channel 3
    pub mod GIF3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error (TE) flag for channel 2
    pub mod TEIF2 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer (HT) flag for channel 2
    pub mod HTIF2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete (TC) flag for channel 2
    pub mod TCIF2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag for channel 2
    pub mod GIF2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error (TE) flag for channel 1
    pub mod TEIF1 {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer (HT) flag for channel 1
    pub mod HTIF1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete (TC) flag for channel 1
    pub mod TCIF1 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag for channel 1
    pub mod GIF1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }
}

/// interrupt flag clear register
pub mod IFCR {

    /// transfer error flag clear for channel 7
    pub mod TEIF7 {
        /// Offset (27 bits)
        pub const offset: u32 = 27;
        /// Mask (1 bit: 1 << 27)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No transfer error on channel x
            pub const NoError: u32 = 0b0;

            /// 0b1: A transfer error occurred on channel x
            pub const Error: u32 = 0b1;
        }
    }

    /// half transfer flag clear for channel 7
    pub mod HTIF7 {
        /// Offset (26 bits)
        pub const offset: u32 = 26;
        /// Mask (1 bit: 1 << 26)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No half transfer event on channel x
            pub const NotHalf: u32 = 0b0;

            /// 0b1: A half transfer event occurred on channel x
            pub const Half: u32 = 0b1;
        }
    }

    /// transfer complete flag clear for channel 7
    pub mod TCIF7 {
        /// Offset (25 bits)
        pub const offset: u32 = 25;
        /// Mask (1 bit: 1 << 25)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: No transfer complete event on channel x
            pub const NotComplete: u32 = 0b0;

            /// 0b1: A transfer complete event occurred on channel x
            pub const Complete: u32 = 0b1;
        }
    }

    /// global interrupt flag clear for channel 7
    pub mod GIF7 {
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

            /// 0b0: No TE, HT or TC event on channel x
            pub const NoEvent: u32 = 0b0;

            /// 0b1: A TE, HT or TC event occurred on channel x
            pub const Event: u32 = 0b1;
        }
    }

    /// transfer error flag clear for channel 6
    pub mod TEIF6 {
        /// Offset (23 bits)
        pub const offset: u32 = 23;
        /// Mask (1 bit: 1 << 23)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer flag clear for channel 6
    pub mod HTIF6 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (1 bit: 1 << 22)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete flag clear for channel 6
    pub mod TCIF6 {
        /// Offset (21 bits)
        pub const offset: u32 = 21;
        /// Mask (1 bit: 1 << 21)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag clear for channel 6
    pub mod GIF6 {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (1 bit: 1 << 20)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error flag clear for channel 5
    pub mod TEIF5 {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer flag clear for channel 5
    pub mod HTIF5 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete flag clear for channel 5
    pub mod TCIF5 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag clear for channel 5
    pub mod GIF5 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error flag clear for channel 4
    pub mod TEIF4 {
        /// Offset (15 bits)
        pub const offset: u32 = 15;
        /// Mask (1 bit: 1 << 15)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer flag clear for channel 4
    pub mod HTIF4 {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete flag clear for channel 4
    pub mod TCIF4 {
        /// Offset (13 bits)
        pub const offset: u32 = 13;
        /// Mask (1 bit: 1 << 13)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag clear for channel 4
    pub mod GIF4 {
        /// Offset (12 bits)
        pub const offset: u32 = 12;
        /// Mask (1 bit: 1 << 12)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error flag clear for channel 3
    pub mod TEIF3 {
        /// Offset (11 bits)
        pub const offset: u32 = 11;
        /// Mask (1 bit: 1 << 11)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer flag clear for channel 3
    pub mod HTIF3 {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (1 bit: 1 << 10)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete flag clear for channel 3
    pub mod TCIF3 {
        /// Offset (9 bits)
        pub const offset: u32 = 9;
        /// Mask (1 bit: 1 << 9)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag clear for channel 3
    pub mod GIF3 {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (1 bit: 1 << 8)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error flag clear for channel 2
    pub mod TEIF2 {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TEIF7::RW;
    }

    /// half transfer flag clear for channel 2
    pub mod HTIF2 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::HTIF7::RW;
    }

    /// transfer complete flag clear for channel 2
    pub mod TCIF2 {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::TCIF7::RW;
    }

    /// global interrupt flag clear for channel 2
    pub mod GIF2 {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (1 bit: 1 << 4)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::GIF7::RW;
    }

    /// transfer error flag clear for channel 1
    pub mod TEIF1 {
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

            /// 0b1: Clear the corresponding TEIFx flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// half transfer flag clear for channel 1
    pub mod HTIF1 {
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

            /// 0b1: Clear the corresponding HTIFx flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// transfer complete flag clear for channel 1
    pub mod TCIF1 {
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

            /// 0b1: Clear the corresponding TCIFx flag
            pub const Clear: u32 = 0b1;
        }
    }

    /// global interrupt flag clear for channel 1
    pub mod GIF1 {
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

            /// 0b1: Clear the corresponding CGIFx flag
            pub const Clear: u32 = 0b1;
        }
    }
}

/// channel x configuration register
pub mod CCR1 {

    /// rivileged mode
    pub mod PRIV {
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

            /// 0b0: Disabled
            pub const Disabled: u32 = 0b0;

            /// 0b1: Enabled
            pub const Enabled: u32 = 0b1;
        }
    }

    /// ecurity of the DMA transfer to the destination
    pub mod DSEC {
        /// Offset (19 bits)
        pub const offset: u32 = 19;
        /// Mask (1 bit: 1 << 19)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ecurity of the DMA transfer from the source
    pub mod SSEC {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (1 bit: 1 << 18)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// ecure mode
    pub mod SECM {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// memory-to-memory mode
    pub mod MEM2MEM {
        /// Offset (14 bits)
        pub const offset: u32 = 14;
        /// Mask (1 bit: 1 << 14)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRIV::RW;
    }

    /// priority level
    pub mod PL {
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

            /// 0b00: Low
            pub const Low: u32 = 0b00;

            /// 0b01: Medium
            pub const Medium: u32 = 0b01;

            /// 0b10: High
            pub const High: u32 = 0b10;

            /// 0b11: Very high
            pub const VeryHigh: u32 = 0b11;
        }
    }

    /// memory size
    pub mod MSIZE {
        /// Offset (10 bits)
        pub const offset: u32 = 10;
        /// Mask (2 bits: 0b11 << 10)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: 8 bits
            pub const Bits8: u32 = 0b00;

            /// 0b01: 16 bits
            pub const Bits16: u32 = 0b01;

            /// 0b10: 32 bits
            pub const Bits32: u32 = 0b10;
        }
    }

    /// peripheral size
    pub mod PSIZE {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (2 bits: 0b11 << 8)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MSIZE::RW;
    }

    /// memory increment mode
    pub mod MINC {
        /// Offset (7 bits)
        pub const offset: u32 = 7;
        /// Mask (1 bit: 1 << 7)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRIV::RW;
    }

    /// peripheral increment mode
    pub mod PINC {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (1 bit: 1 << 6)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRIV::RW;
    }

    /// circular mode
    pub mod CIRC {
        /// Offset (5 bits)
        pub const offset: u32 = 5;
        /// Mask (1 bit: 1 << 5)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRIV::RW;
    }

    /// data transfer direction
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

            /// 0b0: Read from peripheral
            pub const Peripheral: u32 = 0b0;

            /// 0b1: Read from memory
            pub const Memory: u32 = 0b1;
        }
    }

    /// transfer error interrupt enable
    pub mod TEIE {
        /// Offset (3 bits)
        pub const offset: u32 = 3;
        /// Mask (1 bit: 1 << 3)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRIV::RW;
    }

    /// half transfer interrupt enable
    pub mod HTIE {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (1 bit: 1 << 2)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRIV::RW;
    }

    /// transfer complete interrupt enable
    pub mod TCIE {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRIV::RW;
    }

    /// channel enable
    pub mod EN {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (1 bit: 1 << 0)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::PRIV::RW;
    }
}

/// channel x number of data to transfer register
pub mod CNDTR1 {

    /// number of data to transfer (0 to 218 - 1)
    pub mod NDT {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (18 bits: 0x3ffff << 0)
        pub const mask: u32 = 0x3ffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel x peripheral address register
pub mod CPAR1 {

    /// peripheral address
    pub mod PA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel x memory address register
pub mod CMAR1 {

    /// peripheral address
    pub mod MA {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (32 bits: 0xffffffff << 0)
        pub const mask: u32 = 0xffffffff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// channel x configuration register
pub mod CCR2 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x number of data to transfer register
pub mod CNDTR2 {
    pub use super::CNDTR1::NDT;
}

/// channel x peripheral address register
pub mod CPAR2 {
    pub use super::CPAR1::PA;
}

/// channel x memory address register
pub mod CMAR2 {
    pub use super::CMAR1::MA;
}

/// channel x configuration register
pub mod CCR3 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x number of data to transfer register
pub mod CNDTR3 {
    pub use super::CNDTR1::NDT;
}

/// channel x peripheral address register
pub mod CPAR3 {
    pub use super::CPAR1::PA;
}

/// channel x memory address register
pub mod CMAR3 {
    pub use super::CMAR1::MA;
}

/// channel x configuration register
pub mod CCR4 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x number of data to transfer register
pub mod CNDTR4 {
    pub use super::CNDTR1::NDT;
}

/// channel x peripheral address register
pub mod CPAR4 {
    pub use super::CPAR1::PA;
}

/// channel x memory address register
pub mod CMAR4 {
    pub use super::CMAR1::MA;
}

/// channel x configuration register
pub mod CCR5 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x number of data to transfer register
pub mod CNDTR5 {
    pub use super::CNDTR1::NDT;
}

/// channel x peripheral address register
pub mod CPAR5 {
    pub use super::CPAR1::PA;
}

/// channel x memory address register
pub mod CMAR5 {
    pub use super::CMAR1::MA;
}

/// channel x configuration register
pub mod CCR6 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x number of data to transfer register
pub mod CNDTR6 {
    pub use super::CNDTR1::NDT;
}

/// channel x peripheral address register
pub mod CPAR6 {
    pub use super::CPAR1::PA;
}

/// channel x memory address register
pub mod CMAR6 {
    pub use super::CMAR1::MA;
}

/// channel x configuration register
pub mod CCR7 {
    pub use super::CCR1::CIRC;
    pub use super::CCR1::DIR;
    pub use super::CCR1::DSEC;
    pub use super::CCR1::EN;
    pub use super::CCR1::HTIE;
    pub use super::CCR1::MEM2MEM;
    pub use super::CCR1::MINC;
    pub use super::CCR1::MSIZE;
    pub use super::CCR1::PINC;
    pub use super::CCR1::PL;
    pub use super::CCR1::PRIV;
    pub use super::CCR1::PSIZE;
    pub use super::CCR1::SECM;
    pub use super::CCR1::SSEC;
    pub use super::CCR1::TCIE;
    pub use super::CCR1::TEIE;
}

/// channel x number of data to transfer register
pub mod CNDTR7 {
    pub use super::CNDTR1::NDT;
}

/// channel x peripheral address register
pub mod CPAR7 {
    pub use super::CPAR1::PA;
}

/// channel x memory address register
pub mod CMAR7 {
    pub use super::CMAR1::MA;
}
#[repr(C)]
pub struct RegisterBlock {
    /// interrupt status register
    pub ISR: RORegister<u32>,

    /// interrupt flag clear register
    pub IFCR: WORegister<u32>,

    /// channel x configuration register
    pub CCR1: RWRegister<u32>,

    /// channel x number of data to transfer register
    pub CNDTR1: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR1: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CMAR1: UnsafeRWRegister<u32>,

    _reserved1: [u32; 1],

    /// channel x configuration register
    pub CCR2: RWRegister<u32>,

    /// channel x number of data to transfer register
    pub CNDTR2: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR2: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CMAR2: UnsafeRWRegister<u32>,

    _reserved2: [u32; 1],

    /// channel x configuration register
    pub CCR3: RWRegister<u32>,

    /// channel x number of data to transfer register
    pub CNDTR3: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR3: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CMAR3: UnsafeRWRegister<u32>,

    _reserved3: [u32; 1],

    /// channel x configuration register
    pub CCR4: RWRegister<u32>,

    /// channel x number of data to transfer register
    pub CNDTR4: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR4: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CMAR4: UnsafeRWRegister<u32>,

    _reserved4: [u32; 1],

    /// channel x configuration register
    pub CCR5: RWRegister<u32>,

    /// channel x number of data to transfer register
    pub CNDTR5: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR5: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CMAR5: UnsafeRWRegister<u32>,

    _reserved5: [u32; 1],

    /// channel x configuration register
    pub CCR6: RWRegister<u32>,

    /// channel x number of data to transfer register
    pub CNDTR6: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR6: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CMAR6: UnsafeRWRegister<u32>,

    _reserved6: [u32; 1],

    /// channel x configuration register
    pub CCR7: RWRegister<u32>,

    /// channel x number of data to transfer register
    pub CNDTR7: RWRegister<u32>,

    /// channel x peripheral address register
    pub CPAR7: UnsafeRWRegister<u32>,

    /// channel x memory address register
    pub CMAR7: UnsafeRWRegister<u32>,
}
pub struct ResetValues {
    pub ISR: u32,
    pub IFCR: u32,
    pub CCR1: u32,
    pub CNDTR1: u32,
    pub CPAR1: u32,
    pub CMAR1: u32,
    pub CCR2: u32,
    pub CNDTR2: u32,
    pub CPAR2: u32,
    pub CMAR2: u32,
    pub CCR3: u32,
    pub CNDTR3: u32,
    pub CPAR3: u32,
    pub CMAR3: u32,
    pub CCR4: u32,
    pub CNDTR4: u32,
    pub CPAR4: u32,
    pub CMAR4: u32,
    pub CCR5: u32,
    pub CNDTR5: u32,
    pub CPAR5: u32,
    pub CMAR5: u32,
    pub CCR6: u32,
    pub CNDTR6: u32,
    pub CPAR6: u32,
    pub CMAR6: u32,
    pub CCR7: u32,
    pub CNDTR7: u32,
    pub CPAR7: u32,
    pub CMAR7: u32,
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
