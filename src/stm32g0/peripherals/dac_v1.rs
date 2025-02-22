#![allow(non_snake_case, non_upper_case_globals)]
#![allow(non_camel_case_types)]
//! DAC
//!
//! Used by: stm32g051, stm32g061, stm32g0b1, stm32g0c1

use crate::{RORegister, RWRegister, WORegister};
#[cfg(not(feature = "nosync"))]
use core::marker::PhantomData;

/// DAC control register
pub mod DAC_CR {

    /// DAC channel1 enable This bit is set and cleared by software to enable/disable DAC channel1.
    pub mod EN1 {
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

            /// 0b0: DAC channel1 disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel1 enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel1 trigger enable This bit is set and cleared by software to enable/disable DAC channel1 trigger. Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the DAC_DOR1 register takes only one dac_pclk clock cycle.
    pub mod TEN1 {
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

            /// 0b0: DAC channel1 trigger disabled and data written into the DAC_DHR1 register are transferred one dac_pclk clock cycle later to the DAC_DOR1 register
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred three dac_pclk clock cycles later to the DAC_DOR1 register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel1 trigger selection These bits select the external event used to trigger DAC channel1 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
    pub mod TSEL1 {
        /// Offset (2 bits)
        pub const offset: u32 = 2;
        /// Mask (4 bits: 0b1111 << 2)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: SWTRIG1
            pub const B_0x0: u32 = 0b0000;

            /// 0b0001: dac_ch1_trg1
            pub const B_0x1: u32 = 0b0001;

            /// 0b0010: dac_ch1_trg2
            pub const B_0x2: u32 = 0b0010;

            /// 0b1111: dac_ch1_trg15
            pub const B_0xF: u32 = 0b1111;
        }
    }

    /// DAC channel1 noise/triangle wave generation enable These bits are set and cleared by software. 1x: Triangle wave generation enabled Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
    pub mod WAVE1 {
        /// Offset (6 bits)
        pub const offset: u32 = 6;
        /// Mask (2 bits: 0b11 << 6)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b00: wave generation disabled
            pub const B_0x0: u32 = 0b00;

            /// 0b01: Noise wave generation enabled
            pub const B_0x1: u32 = 0b01;
        }
    }

    /// DAC channel1 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. ≥ 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095
    pub mod MAMP1 {
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

            /// 0b0000: Unmask bit0 of LFSR/ triangle amplitude equal to 1
            pub const B_0x0: u32 = 0b0000;

            /// 0b0001: Unmask bits\[1:0\] of LFSR/ triangle amplitude equal to 3
            pub const B_0x1: u32 = 0b0001;

            /// 0b0010: Unmask bits\[2:0\] of LFSR/ triangle amplitude equal to 7
            pub const B_0x2: u32 = 0b0010;

            /// 0b0011: Unmask bits\[3:0\] of LFSR/ triangle amplitude equal to 15
            pub const B_0x3: u32 = 0b0011;

            /// 0b0100: Unmask bits\[4:0\] of LFSR/ triangle amplitude equal to 31
            pub const B_0x4: u32 = 0b0100;

            /// 0b0101: Unmask bits\[5:0\] of LFSR/ triangle amplitude equal to 63
            pub const B_0x5: u32 = 0b0101;

            /// 0b0110: Unmask bits\[6:0\] of LFSR/ triangle amplitude equal to 127
            pub const B_0x6: u32 = 0b0110;

            /// 0b0111: Unmask bits\[7:0\] of LFSR/ triangle amplitude equal to 255
            pub const B_0x7: u32 = 0b0111;

            /// 0b1000: Unmask bits\[8:0\] of LFSR/ triangle amplitude equal to 511
            pub const B_0x8: u32 = 0b1000;

            /// 0b1001: Unmask bits\[9:0\] of LFSR/ triangle amplitude equal to 1023
            pub const B_0x9: u32 = 0b1001;

            /// 0b1010: Unmask bits\[10:0\] of LFSR/ triangle amplitude equal to 2047
            pub const B_0xA: u32 = 0b1010;
        }
    }

    /// DAC channel1 DMA enable This bit is set and cleared by software.
    pub mod DMAEN1 {
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

            /// 0b0: DAC channel1 DMA mode disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel1 DMA mode enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel1 DMA Underrun Interrupt enable This bit is set and cleared by software.
    pub mod DMAUDRIE1 {
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

            /// 0b0: DAC channel1 DMA Underrun Interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel1 DMA Underrun Interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel1 calibration enable This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be written only if bit EN1=0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored.
    pub mod CEN1 {
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

            /// 0b0: DAC channel1 in Normal operating mode
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel1 in calibration mode
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel2 enable This bit is set and cleared by software to enable/disable DAC channel2. Note: These bits are available only on dual-channel DACs. Refer to implementation.
    pub mod EN2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (1 bit: 1 << 16)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel2 disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel2 enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel2 trigger enable This bit is set and cleared by software to enable/disable DAC channel2 trigger Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the DAC_DOR2 register takes only one dac_pclk clock cycle. These bits are available only on dual-channel DACs. Refer to implementation.
    pub mod TEN2 {
        /// Offset (17 bits)
        pub const offset: u32 = 17;
        /// Mask (1 bit: 1 << 17)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel2 trigger disabled and data written into the DAC_DHR2 register are transferred one dac_pclk clock cycle later to the DAC_DOR2 register
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel2 trigger enabled and data from the DAC_DHR2 register are transferred three dac_pclk clock cycles later to the DAC_DOR2 register
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel2 trigger selection These bits select the external event used to trigger DAC channel2 ... Refer to the trigger selection tables in for details on trigger configuration and mapping. Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled). These bits are available only on dual-channel DACs. Refer to implementation.
    pub mod TSEL2 {
        /// Offset (18 bits)
        pub const offset: u32 = 18;
        /// Mask (4 bits: 0b1111 << 18)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0000: SWTRIG2
            pub const B_0x0: u32 = 0b0000;

            /// 0b0001: dac_ch2_trg1
            pub const B_0x1: u32 = 0b0001;

            /// 0b0010: dac_ch2_trg2
            pub const B_0x2: u32 = 0b0010;

            /// 0b1111: dac_ch2_trg15
            pub const B_0xF: u32 = 0b1111;
        }
    }

    /// DAC channel2 noise/triangle wave generation enable These bits are set/reset by software. 1x: Triangle wave generation enabled Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled) These bits are available only on dual-channel DACs. Refer to implementation.
    pub mod WAVE2 {
        /// Offset (22 bits)
        pub const offset: u32 = 22;
        /// Mask (2 bits: 0b11 << 22)
        pub const mask: u32 = 0b11 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::WAVE1::RW;
    }

    /// DAC channel2 mask/amplitude selector These bits are written by software to select mask in wave generation mode or amplitude in triangle generation mode. ≥ 1011: Unmask bits\[11:0\] of LFSR/ triangle amplitude equal to 4095 Note: These bits are available only on dual-channel DACs. Refer to implementation.
    pub mod MAMP2 {
        /// Offset (24 bits)
        pub const offset: u32 = 24;
        /// Mask (4 bits: 0b1111 << 24)
        pub const mask: u32 = 0b1111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::MAMP1::RW;
    }

    /// DAC channel2 DMA enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    pub mod DMAEN2 {
        /// Offset (28 bits)
        pub const offset: u32 = 28;
        /// Mask (1 bit: 1 << 28)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b0: DAC channel2 DMA mode disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel2 DMA mode enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel2 DMA underrun interrupt enable This bit is set and cleared by software. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    pub mod DMAUDRIE2 {
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

            /// 0b0: DAC channel2 DMA underrun interrupt disabled
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel2 DMA underrun interrupt enabled
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel2 calibration enable This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only when the DAC channel is disabled) Otherwise, the write operation is ignored. Note: This bit is available only on dual-channel DACs. Refer to implementation.
    pub mod CEN2 {
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

            /// 0b0: DAC channel2 in Normal operating mode
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DAC channel2 in calibration mode
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// DAC software trigger register
pub mod DAC_SWTRGR {

    /// DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register.
    pub mod SWTRIG1 {
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

            /// 0b0: No trigger
            pub const B_0x0: u32 = 0b0;

            /// 0b1: Trigger
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one dac_pclk clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register. This bit is available only on dual-channel DACs. Refer to implementation.
    pub mod SWTRIG2 {
        /// Offset (1 bits)
        pub const offset: u32 = 1;
        /// Mask (1 bit: 1 << 1)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::SWTRIG1::RW;
    }
}

/// DAC channel1 12-bit right-aligned data holding register
pub mod DAC_DHR12R1 {

    /// DAC channel1 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel1.
    pub mod DACC1DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC channel1 12-bit left aligned data holding register
pub mod DAC_DHR12L1 {

    /// DAC channel1 12-bit left-aligned data These bits are written by software. They specify 12-bit data for DAC channel1.
    pub mod DACC1DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC channel1 8-bit right aligned data holding register
pub mod DAC_DHR8R1 {

    /// DAC channel1 8-bit right-aligned data These bits are written by software. They specify 8-bit data for DAC channel1.
    pub mod DACC1DHR {
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

/// DAC channel2 12-bit right aligned data holding register
pub mod DAC_DHR12R2 {

    /// DAC channel2 12-bit right-aligned data These bits are written by software. They specify 12-bit data for DAC channel2.
    pub mod DACC2DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC channel2 12-bit left aligned data holding register
pub mod DAC_DHR12L2 {

    /// DAC channel2 12-bit left-aligned data These bits are written by software which specify 12-bit data for DAC channel2.
    pub mod DACC2DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC channel2 8-bit right-aligned data holding register
pub mod DAC_DHR8R2 {

    /// DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2.
    pub mod DACC2DHR {
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

/// Dual DAC 12-bit right-aligned data holding register
pub mod DAC_DHR12RD {

    /// DAC channel1 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
    pub mod DACC1DHR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel2 12-bit right-aligned data These bits are written by software which specifies 12-bit data for DAC channel2.
    pub mod DACC2DHR {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (12 bits: 0xfff << 16)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DUAL DAC 12-bit left aligned data holding register
pub mod DAC_DHR12LD {

    /// DAC channel1 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel1.
    pub mod DACC1DHR {
        /// Offset (4 bits)
        pub const offset: u32 = 4;
        /// Mask (12 bits: 0xfff << 4)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel2 12-bit left-aligned data These bits are written by software which specifies 12-bit data for DAC channel2.
    pub mod DACC2DHR {
        /// Offset (20 bits)
        pub const offset: u32 = 20;
        /// Mask (12 bits: 0xfff << 20)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DUAL DAC 8-bit right aligned data holding register
pub mod DAC_DHR8RD {

    /// DAC channel1 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel1.
    pub mod DACC1DHR {
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

    /// DAC channel2 8-bit right-aligned data These bits are written by software which specifies 8-bit data for DAC channel2.
    pub mod DACC2DHR {
        /// Offset (8 bits)
        pub const offset: u32 = 8;
        /// Mask (8 bits: 0xff << 8)
        pub const mask: u32 = 0xff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC channel1 data output register
pub mod DAC_DOR1 {

    /// DAC channel1 data output These bits are read-only, they contain data output for DAC channel1.
    pub mod DACC1DOR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC channel2 data output register
pub mod DAC_DOR2 {

    /// DAC channel2 data output These bits are read-only, they contain data output for DAC channel2.
    pub mod DACC2DOR {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (12 bits: 0xfff << 0)
        pub const mask: u32 = 0xfff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC status register
pub mod DAC_SR {

    /// DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    pub mod DMAUDR1 {
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

            /// 0b0: No DMA underrun error condition occurred for DAC channel1
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel1 calibration offset status This bit is set and cleared by hardware
    pub mod CAL_FLAG1 {
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

            /// 0b0: calibration trimming value is lower than the offset correction value
            pub const B_0x0: u32 = 0b0;

            /// 0b1: calibration trimming value is equal or greater than the offset correction value
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel1 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3 LSI periods of synchronization).
    pub mod BWST1 {
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

            /// 0b0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written
            pub const B_0x0: u32 = 0b0;

            /// 0b1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1). Note: This bit is available only on dual-channel DACs. Refer to implementation.
    pub mod DMAUDR2 {
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

            /// 0b0: No DMA underrun error condition occurred for DAC channel2
            pub const B_0x0: u32 = 0b0;

            /// 0b1: DMA underrun error condition occurred for DAC channel2 (the currently selected trigger is driving DAC channel2 conversion at a frequency higher than the DMA service capability rate).
            pub const B_0x1: u32 = 0b1;
        }
    }

    /// DAC channel2 calibration offset status This bit is set and cleared by hardware Note: This bit is available only on dual-channel DACs. Refer to implementation.
    pub mod CAL_FLAG2 {
        /// Offset (30 bits)
        pub const offset: u32 = 30;
        /// Mask (1 bit: 1 << 30)
        pub const mask: u32 = 1 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        pub use super::CAL_FLAG1::RW;
    }

    /// DAC channel2 busy writing sample time flag This bit is systematically set just after Sample and hold mode enable. It is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization). Note: This bit is available only on dual-channel DACs. Refer to implementation.
    pub mod BWST2 {
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

            /// 0b0: There is no write operation of DAC_SHSR2 ongoing: DAC_SHSR2 can be written
            pub const B_0x0: u32 = 0b0;

            /// 0b1: There is a write operation of DAC_SHSR2 ongoing: DAC_SHSR2 cannot be written
            pub const B_0x1: u32 = 0b1;
        }
    }
}

/// DAC calibration control register
pub mod DAC_CCR {

    /// DAC channel1 offset trimming value
    pub mod OTRIM1 {
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

    /// DAC channel2 offset trimming value These bits are available only on dual-channel DACs. Refer to implementation.
    pub mod OTRIM2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (5 bits: 0b11111 << 16)
        pub const mask: u32 = 0b11111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC mode control register
pub mod DAC_MCR {

    /// DAC channel1 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN1=0 and bit CEN1 =0 in the DAC_CR register). If EN1=1 or CEN1 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel1 mode: DAC channel1 in Normal mode DAC channel1 in sample & hold mode Note: This register can be modified only when EN1=0.
    pub mod MODE1 {
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

            /// 0b000: DAC channel1 is connected to external pin with Buffer enabled
            pub const B_0x0: u32 = 0b000;

            /// 0b001: DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled
            pub const B_0x1: u32 = 0b001;

            /// 0b010: DAC channel1 is connected to external pin with Buffer disabled
            pub const B_0x2: u32 = 0b010;

            /// 0b011: DAC channel1 is connected to on chip peripherals with Buffer disabled
            pub const B_0x3: u32 = 0b011;

            /// 0b100: DAC channel1 is connected to external pin with Buffer enabled
            pub const B_0x4: u32 = 0b100;

            /// 0b101: DAC channel1 is connected to external pin and to on chip peripherals with Buffer enabled
            pub const B_0x5: u32 = 0b101;

            /// 0b110: DAC channel1 is connected to external pin and to on chip peripherals with Buffer disabled
            pub const B_0x6: u32 = 0b110;

            /// 0b111: DAC channel1 is connected to on chip peripherals with Buffer disabled
            pub const B_0x7: u32 = 0b111;
        }
    }

    /// DAC channel2 mode These bits can be written only when the DAC is disabled and not in the calibration mode (when bit EN2=0 and bit CEN2 =0 in the DAC_CR register). If EN2=1 or CEN2 =1 the write operation is ignored. They can be set and cleared by software to select the DAC channel2 mode: DAC channel2 in Normal mode DAC channel2 in Sample and hold mode Note: This register can be modified only when EN2=0. Refer to for the availability of DAC channel2.
    pub mod MODE2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (3 bits: 0b111 << 16)
        pub const mask: u32 = 0b111 << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values
        pub mod RW {

            /// 0b000: DAC channel2 is connected to external pin with Buffer enabled
            pub const B_0x0: u32 = 0b000;

            /// 0b001: DAC channel2 is connected to external pin and to on chip peripherals with buffer enabled
            pub const B_0x1: u32 = 0b001;

            /// 0b010: DAC channel2 is connected to external pin with buffer disabled
            pub const B_0x2: u32 = 0b010;

            /// 0b011: DAC channel2 is connected to on chip peripherals with Buffer disabled
            pub const B_0x3: u32 = 0b011;

            /// 0b100: DAC channel2 is connected to external pin with Buffer enabled
            pub const B_0x4: u32 = 0b100;

            /// 0b101: DAC channel2 is connected to external pin and to on chip peripherals with Buffer enabled
            pub const B_0x5: u32 = 0b101;

            /// 0b110: DAC channel2 is connected to external pin and to on chip peripherals with Buffer disabled
            pub const B_0x6: u32 = 0b110;

            /// 0b111: DAC channel2 is connected to on chip peripherals with Buffer disabled
            pub const B_0x7: u32 = 0b111;
        }
    }
}

/// DAC Sample and Hold sample time register 1
pub mod DAC_SHSR1 {

    /// DAC channel1 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel1 is disabled or also during normal operation. in the latter case, the write can be done only when BWST1 of DAC_SR register is low, If BWST1=1, the write operation is ignored.
    pub mod TSAMPLE1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC Sample and Hold sample time register 2
pub mod DAC_SHSR2 {

    /// DAC channel2 sample time (only valid in Sample and hold mode) These bits can be written when the DAC channel2 is disabled or also during normal operation. in the latter case, the write can be done only when BWST2 of DAC_SR register is low, if BWST2=1, the write operation is ignored.
    pub mod TSAMPLE2 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC Sample and Hold hold time register
pub mod DAC_SHHR {

    /// DAC channel1 hold time (only valid in Sample and hold mode) Hold time= (THOLD\[9:0\]) x LSI clock period Note: This register can be modified only when EN1=0.
    pub mod THOLD1 {
        /// Offset (0 bits)
        pub const offset: u32 = 0;
        /// Mask (10 bits: 0x3ff << 0)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }

    /// DAC channel2 hold time (only valid in Sample and hold mode). Hold time= (THOLD\[9:0\]) x LSI clock period Note: This register can be modified only when EN2=0. These bits are available only on dual-channel DACs. Refer to implementation.
    pub mod THOLD2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (10 bits: 0x3ff << 16)
        pub const mask: u32 = 0x3ff << offset;
        /// Read-only values (empty)
        pub mod R {}
        /// Write-only values (empty)
        pub mod W {}
        /// Read-write values (empty)
        pub mod RW {}
    }
}

/// DAC Sample and Hold refresh time register
pub mod DAC_SHRR {

    /// DAC channel1 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\[7:0\]) x LSI clock period Note: This register can be modified only when EN1=0.
    pub mod TREFRESH1 {
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

    /// DAC channel2 refresh time (only valid in Sample and hold mode) Refresh time= (TREFRESH\[7:0\]) x LSI clock period Note: This register can be modified only when EN2=0. These bits are available only on dual-channel DACs. Refer to implementation.
    pub mod TREFRESH2 {
        /// Offset (16 bits)
        pub const offset: u32 = 16;
        /// Mask (8 bits: 0xff << 16)
        pub const mask: u32 = 0xff << offset;
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
    /// DAC control register
    pub DAC_CR: RWRegister<u32>,

    /// DAC software trigger register
    pub DAC_SWTRGR: WORegister<u32>,

    /// DAC channel1 12-bit right-aligned data holding register
    pub DAC_DHR12R1: RWRegister<u32>,

    /// DAC channel1 12-bit left aligned data holding register
    pub DAC_DHR12L1: RWRegister<u32>,

    /// DAC channel1 8-bit right aligned data holding register
    pub DAC_DHR8R1: RWRegister<u32>,

    /// DAC channel2 12-bit right aligned data holding register
    pub DAC_DHR12R2: RWRegister<u32>,

    /// DAC channel2 12-bit left aligned data holding register
    pub DAC_DHR12L2: RWRegister<u32>,

    /// DAC channel2 8-bit right-aligned data holding register
    pub DAC_DHR8R2: RWRegister<u32>,

    /// Dual DAC 12-bit right-aligned data holding register
    pub DAC_DHR12RD: RWRegister<u32>,

    /// DUAL DAC 12-bit left aligned data holding register
    pub DAC_DHR12LD: RWRegister<u32>,

    /// DUAL DAC 8-bit right aligned data holding register
    pub DAC_DHR8RD: RWRegister<u32>,

    /// DAC channel1 data output register
    pub DAC_DOR1: RORegister<u32>,

    /// DAC channel2 data output register
    pub DAC_DOR2: RORegister<u32>,

    /// DAC status register
    pub DAC_SR: RWRegister<u32>,

    /// DAC calibration control register
    pub DAC_CCR: RWRegister<u32>,

    /// DAC mode control register
    pub DAC_MCR: RWRegister<u32>,

    /// DAC Sample and Hold sample time register 1
    pub DAC_SHSR1: RWRegister<u32>,

    /// DAC Sample and Hold sample time register 2
    pub DAC_SHSR2: RWRegister<u32>,

    /// DAC Sample and Hold hold time register
    pub DAC_SHHR: RWRegister<u32>,

    /// DAC Sample and Hold refresh time register
    pub DAC_SHRR: RWRegister<u32>,
}
pub struct ResetValues {
    pub DAC_CR: u32,
    pub DAC_SWTRGR: u32,
    pub DAC_DHR12R1: u32,
    pub DAC_DHR12L1: u32,
    pub DAC_DHR8R1: u32,
    pub DAC_DHR12R2: u32,
    pub DAC_DHR12L2: u32,
    pub DAC_DHR8R2: u32,
    pub DAC_DHR12RD: u32,
    pub DAC_DHR12LD: u32,
    pub DAC_DHR8RD: u32,
    pub DAC_DOR1: u32,
    pub DAC_DOR2: u32,
    pub DAC_SR: u32,
    pub DAC_CCR: u32,
    pub DAC_MCR: u32,
    pub DAC_SHSR1: u32,
    pub DAC_SHSR2: u32,
    pub DAC_SHHR: u32,
    pub DAC_SHRR: u32,
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
