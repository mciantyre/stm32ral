//! Show that we compute reserved peripheral memory properly.

#[cfg(any(
    feature = "stm32l0x0",
    feature = "stm32l0x1",
    feature = "stm32l0x2",
    feature = "stm32l0x3",
))]
#[test]
fn reserved_stm32l0_tim2x() {
    // Consistent for TIM2 and TIM21/TIM22.
    const TIMX_CCR1_OFFSET: u32 = 0x34;
    // Table 3 in the SIM32L0x0 reference manual.
    const TIM21_ADDRESS: u32 = 0x4001_0800;
    const TIM22_ADDRESS: u32 = 0x4001_1400;
    const TIM2_ADDRESS: u32 = 0x4000_0000;

    let tim2 = unsafe { &*stm32ral::tim2::TIM2 };
    assert_eq!(
        core::ptr::addr_of!((*tim2).CCR1) as u32,
        TIM2_ADDRESS + TIMX_CCR1_OFFSET
    );

    let tim21 = unsafe { &*stm32ral::tim21::TIM21 };
    assert_eq!(
        core::ptr::addr_of!((*tim21).CCR1) as u32,
        TIM21_ADDRESS + TIMX_CCR1_OFFSET
    );

    let tim22 = unsafe { &*stm32ral::tim22::TIM22 };
    assert_eq!(
        core::ptr::addr_of!((*tim22).CCR1) as u32,
        TIM22_ADDRESS + TIMX_CCR1_OFFSET
    );
}
