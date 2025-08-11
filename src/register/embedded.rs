use crate::Error;
use crate::register::EmbedFuncState;
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::{MultiRegister, named_register, register};
use st_mems_bus::BusOperation;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum EmbReg {
    PageSel = 0x2,
    EmbFuncEnA = 0x4,
    EmbFuncEnB = 0x5,
    EmbFuncExecStatus = 0x7,
    PageAddress = 0x8,
    PageValue = 0x9,
    EmbFuncInt1 = 0x0A,
    FsmInt1 = 0x0B,
    MlcInt1 = 0x0D,
    EmbFuncInt2 = 0x0E,
    FsmInt2 = 0x0F,
    MlcInt2 = 0x11,
    EmbFuncStatus = 0x12,
    FsmStatus = 0x13,
    MlcStatus = 0x15,
    PageRw = 0x17,
    SflpGbiasxL = 0x18,
    SflpGbiasxH = 0x19,
    SflpGbiasyL = 0x1A,
    SflpGbiasyH = 0x1B,
    SflpGbiaszL = 0x1C,
    SflpGbiaszH = 0x1D,
    SflpGravxL = 0x1E,
    SflpGravxH = 0x1F,
    SflpGravyL = 0x20,
    SflpGravyH = 0x21,
    SflpGravzL = 0x22,
    SflpGravzH = 0x23,
    SflpQuatwL = 0x2A,
    SflpQuatwH = 0x2B,
    SflpQuatxL = 0x2C,
    SflpQuatxH = 0x2D,
    SflpQuatyL = 0x2E,
    SflpQuatyH = 0x2F,
    SflpQuatzL = 0x30,
    SflpQuatzH = 0x31,
    SflpGbiasxInitL = 0x32,
    SflpGbiasxInitH = 0x33,
    SflpGbiasyInitL = 0x34,
    SflpGbiasyInitH = 0x35,
    SflpGbiaszInitL = 0x36,
    SflpGbiaszInitH = 0x37,
    EmbFuncFifoEnA = 0x44,
    EmbFuncFifoEnB = 0x45,
    FsmEnable = 0x46,
    FsmLongCounterL = 0x48,
    FsmLongCounterH = 0x49,
    IntAckMask = 0x4B,
    FsmOuts1 = 0x4C,
    FsmOuts2 = 0x4D,
    FsmOuts3 = 0x4E,
    FsmOuts4 = 0x4F,
    FsmOuts5 = 0x50,
    FsmOuts6 = 0x51,
    FsmOuts7 = 0x52,
    FsmOuts8 = 0x53,
    SflpOdr = 0x5E,
    FsmOdr = 0x5F,
    MlcOdr = 0x60,
    StepCounterL = 0x62,
    StepCounterH = 0x63,
    EmbFuncSrc = 0x64,
    EmbFuncInitA = 0x66,
    EmbFuncInitB = 0x67,
    EmbFuncSensorConvEn = 0x6E,
    Mlc1Src = 0x70,
    Mlc2Src = 0x71,
    Mlc3Src = 0x72,
    Mlc4Src = 0x73,
    Mlc5Src = 0x74,
    Mlc6Src = 0x75,
    Mlc7Src = 0x76,
    Mlc8Src = 0x77,
}

/// PAGE_SEL (0x02)
///
/// Selects the advanced features dedicated page (R/W).
#[register(address = EmbReg::PageSel, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PageSel {
    #[bits(4, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used0: u8,
    #[bits(4)]
    /// Page selection bits. Default: 0000.
    pub page_sel: u8,
}

/// EMB_FUNC_EN_A (0x04)
///
/// Enable embedded functions register (R/W).

#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
#[register(address = EmbReg::EmbFuncEnA, access_type = EmbedFuncState, generics = 2)]
pub struct EmbFuncEnA {
    #[bits(1, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used0: u8,
    #[bits(1)]
    /// Enables sensor fusion low-power 6-axis game rotation vector. Default: 0 (disabled).
    pub sflp_game_en: u8,
    #[bits(1, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used2: u8,
    #[bits(1)]
    /// Enables pedometer algorithm. Default: 0 (disabled).
    pub pedo_en: u8,
    #[bits(1)]
    /// Enables tilt calculation algorithm. Default: 0 (disabled).
    pub tilt_en: u8,
    #[bits(1)]
    /// Enables significant motion detection. Default: 0 (disabled).
    pub sign_motion_en: u8,
    #[bits(1, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used1: u8,
    #[bits(1)]
    /// Enables machine learning core executed before FSM programs. Default: 0 (disabled).
    pub mlc_before_fsm_en: u8,
}

/// EMB_FUNC_EN_B (0x05)
///
/// Enable embedded functions register (R/W).
#[register(address = EmbReg::EmbFuncEnB, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncEnB {
    #[bits(1)]
    /// Enables FSM function. Default: 0 (disabled).
    pub fsm_en: u8,
    #[bits(2, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used0: u8,
    #[bits(1)]
    /// Enables FIFO compression function. Default: 0 (disabled).
    pub fifo_compr_en: u8,
    #[bits(1)]
    /// Enables machine learning core executed after FSM programs. Default: 0 (disabled).
    pub mlc_en: u8,
    #[bits(3, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used1: u8,
}

/// EMB_FUNC_EXEC_STATUS (0x07)
///
/// Embedded functions execution status register (R).
#[register(address = EmbReg::EmbFuncExecStatus, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncExecStatus {
    #[bits(1)]
    /// 1: no embedded function running. Default: 0.
    pub emb_func_endop: u8,
    #[bits(1)]
    /// 1: execution time exceeded max allowed (overrun). Default: 0.
    pub emb_func_exec_ovr: u8,
    #[bits(6, access = RO)]
    /// Reserved.
    not_used0: u8,
}

/// PAGE_ADDRESS (0x08)
///
/// Address of register to read/write in selected advanced features page (R/W).
#[register(address = EmbReg::PageAddress, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PageAddress {
    #[bits(8)]
    /// Register address within selected page.
    pub page_addr: u8,
}

/// PAGE_VALUE (0x09)
///
/// Data to write or read from selected advanced features page address (R/W).
#[register(address = EmbReg::PageValue, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PageValue {
    #[bits(8)]
    /// Data value for page register.
    pub page_value: u8,
}

/// EMB_FUNC_INT1 (0x0A)
///
/// INT1 pin control register (R/W).
#[register(address = EmbReg::EmbFuncInt1, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncInt1 {
    #[bits(3, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used0: u8,
    #[bits(1)]
    /// Route pedometer step detection interrupt to INT1. Default: 0 (disabled).
    pub int1_step_detector: u8,
    #[bits(1)]
    /// Route tilt interrupt to INT1. Default: 0 (disabled).
    pub int1_tilt: u8,
    #[bits(1)]
    /// Route significant motion interrupt to INT1. Default: 0 (disabled).
    pub int1_sig_mot: u8,
    #[bits(1, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used1: u8,
    #[bits(1)]
    /// Route FSM long counter timeout interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm_lc: u8,
}

/// FSM_INT1 (0x0B)
///
/// INT1 pin control register for FSM interrupts (R/W).
#[register(address = EmbReg::FsmInt1, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmInt1 {
    #[bits(1)]
    /// Route FSM1 interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm1: u8,
    #[bits(1)]
    /// Route FSM2 interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm2: u8,
    #[bits(1)]
    /// Route FSM3 interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm3: u8,
    #[bits(1)]
    /// Route FSM4 interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm4: u8,
    #[bits(1)]
    /// Route FSM5 interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm5: u8,
    #[bits(1)]
    /// Route FSM6 interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm6: u8,
    #[bits(1)]
    /// Route FSM7 interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm7: u8,
    #[bits(1)]
    /// Route FSM8 interrupt to INT1. Default: 0 (disabled).
    pub int1_fsm8: u8,
}

/// MLC_INT1 (0x0D)
///
/// INT1 pin control register for machine learning core interrupts (R/W).
#[register(address = EmbReg::MlcInt1, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcInt1 {
    #[bits(1)]
    /// Route MLC1 interrupt to INT1. Default: 0 (disabled).
    pub int1_mlc1: u8,
    #[bits(1)]
    /// Route MLC2 interrupt to INT1. Default: 0 (disabled).
    pub int1_mlc2: u8,
    #[bits(1)]
    /// Route MLC3 interrupt to INT1. Default: 0 (disabled).
    pub int1_mlc3: u8,
    #[bits(1)]
    /// Route MLC4 interrupt to INT1. Default: 0 (disabled).
    pub int1_mlc4: u8,
    #[bits(1)]
    /// Route MLC5 interrupt to INT1. Default: 0 (disabled).
    pub int1_mlc5: u8,
    #[bits(1)]
    /// Route MLC6 interrupt to INT1. Default: 0 (disabled).
    pub int1_mlc6: u8,
    #[bits(1)]
    /// Route MLC7 interrupt to INT1. Default: 0 (disabled).
    pub int1_mlc7: u8,
    #[bits(1)]
    /// Route MLC8 interrupt to INT1. Default: 0 (disabled).
    pub int1_mlc8: u8,
}

/// EMB_FUNC_INT2 (0x0E)
///
/// INT2 pin control register (R/W).
#[register(address = EmbReg::EmbFuncInt2, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncInt2 {
    #[bits(3, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used0: u8,
    #[bits(1)]
    /// Route pedometer step detection interrupt to INT2. Default: 0 (disabled).
    pub int2_step_detector: u8,
    #[bits(1)]
    /// Route tilt interrupt to INT2. Default: 0 (disabled).
    pub int2_tilt: u8,
    #[bits(1)]
    /// Route significant motion interrupt to INT2. Default: 0 (disabled).
    pub int2_sig_mot: u8,
    #[bits(1, access = RO)]
    /// Reserved, must be 0 for correct operation.
    not_used1: u8,
    #[bits(1)]
    /// Route FSM long counter timeout interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm_lc: u8,
}

/// FSM_INT2 (0x0F)
///
/// INT2 pin control register for FSM interrupts (R/W).
#[register(address = EmbReg::FsmInt2, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmInt2 {
    #[bits(1)]
    /// Route FSM1 interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm1: u8,
    #[bits(1)]
    /// Route FSM2 interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm2: u8,
    #[bits(1)]
    /// Route FSM3 interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm3: u8,
    #[bits(1)]
    /// Route FSM4 interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm4: u8,
    #[bits(1)]
    /// Route FSM5 interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm5: u8,
    #[bits(1)]
    /// Route FSM6 interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm6: u8,
    #[bits(1)]
    /// Route FSM7 interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm7: u8,
    #[bits(1)]
    /// Route FSM8 interrupt to INT2. Default: 0 (disabled).
    pub int2_fsm8: u8,
}

/// MLC_INT2 (0x11)
///
/// INT2 pin control register for machine learning core interrupts (R/W).
#[register(address = EmbReg::MlcInt2, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcInt2 {
    /// Route MLC1 interrupt event to INT2. Default: 0 (disabled).
    #[bits(1)]
    pub int2_mlc1: u8,
    /// Route MLC2 interrupt event to INT2. Default: 0 (disabled).
    #[bits(1)]
    pub int2_mlc2: u8,
    /// Route MLC3 interrupt event to INT2. Default: 0 (disabled).
    #[bits(1)]
    pub int2_mlc3: u8,
    /// Route MLC4 interrupt event to INT2. Default: 0 (disabled).
    #[bits(1)]
    pub int2_mlc4: u8,
    /// Route MLC5 interrupt event to INT2. Default: 0 (disabled).
    #[bits(1)]
    pub int2_mlc5: u8,
    /// Route MLC6 interrupt event to INT2. Default: 0 (disabled).
    #[bits(1)]
    pub int2_mlc6: u8,
    /// Route MLC7 interrupt event to INT2. Default: 0 (disabled).
    #[bits(1)]
    pub int2_mlc7: u8,
    /// Route MLC8 interrupt event to INT2. Default: 0 (disabled).
    #[bits(1)]
    pub int2_mlc8: u8,
}

/// EMB_FUNC_STATUS (0x12)
///
/// Embedded function status register (R).
#[register(address = EmbReg::EmbFuncStatus, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncStatus {
    #[bits(3, access = RO)]
    /// Reserved bits.
    not_used0: u8,
    /// Step detection interrupt status. 1: detected.
    #[bits(1)]
    pub is_step_det: u8,
    /// Tilt detection interrupt status. 1: detected.
    #[bits(1)]
    pub is_tilt: u8,
    /// Significant motion interrupt status. 1: detected.
    #[bits(1)]
    pub is_sigmot: u8,
    #[bits(1, access = RO)]
    /// Reserved bit.
    not_used1: u8,
    /// FSM long counter timeout interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm_lc: u8,
}

/// FSM_STATUS (0x13)
///
/// Finite state machine status register (R).
#[register(address = EmbReg::FsmStatus, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmStatus {
    /// FSM1 interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm1: u8,
    /// FSM2 interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm2: u8,
    /// FSM3 interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm3: u8,
    /// FSM4 interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm4: u8,
    /// FSM5 interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm5: u8,
    /// FSM6 interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm6: u8,
    /// FSM7 interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm7: u8,
    /// FSM8 interrupt status. 1: detected.
    #[bits(1)]
    pub is_fsm8: u8,
}

/// MLC_STATUS (0x15)
///
/// Machine learning core status register (R).
#[register(address = EmbReg::MlcStatus, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcStatus {
    /// MLC1 interrupt status. 1: detected.
    #[bits(1)]
    pub is_mlc1: u8,
    /// MLC2 interrupt status. 1: detected.
    #[bits(1)]
    pub is_mlc2: u8,
    /// MLC3 interrupt status. 1: detected.
    #[bits(1)]
    pub is_mlc3: u8,
    /// MLC4 interrupt status. 1: detected.
    #[bits(1)]
    pub is_mlc4: u8,
    /// MLC5 interrupt status. 1: detected.
    #[bits(1)]
    pub is_mlc5: u8,
    /// MLC6 interrupt status. 1: detected.
    #[bits(1)]
    pub is_mlc6: u8,
    /// MLC7 interrupt status. 1: detected.
    #[bits(1)]
    pub is_mlc7: u8,
    /// MLC8 interrupt status. 1: detected.
    #[bits(1)]
    pub is_mlc8: u8,
}

/// PAGE_RW (0x17)
///
/// Controls read/write mode and interrupt latching for advanced features page (R/W).
#[register(address = EmbReg::PageRw, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PageRw {
    /// Reserved, read-only, must be 0.
    #[bits(5, access = RO)]
    not_used0: u8,
    /// Enables read from selected advanced features page. Default: 0 (disabled).
    #[bits(1)]
    pub page_read: u8,
    /// Enables write to selected advanced features page. Default: 0 (disabled).
    #[bits(1)]
    pub page_write: u8,
    /// Latched interrupt mode for embedded functions. Default: 0 (not latched).
    #[bits(1)]
    pub emb_func_lir: u8,
}

/// SFLP_GBIASX_L to SFLP_GBIASZ_H (0x18 - 0x1D)
///
/// 16-bit two's complement bias for SFLP gyroscope axes (X, Y, Z).
/// Values correspond to a sensitivity of 4.375 mdps/LSB.
#[named_register(address = EmbReg::SflpGbiasxL, access_type = EmbedFuncState, generics = 2)]
pub struct SflpGbiasXYZ {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// SFLP_GRAVX_L to SFLP_GRAVZ_H (0x1E - 0x23)
///
/// 16-bit two's complement sensor fusion low-power gravity output for axes (X, Y, Z).
/// Data values have a sensitivity of 0.061 mg/LSB.
#[named_register(address = EmbReg::SflpGravxL, access_type = EmbedFuncState, generics = 2)]
pub struct SflpGravXYZ {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// SFLP_QUATW_L to SFLP_QUATZ_H (0x2A - 0x2F)
///
/// Sensor fusion low-power game rotation vector quaternion output for components (W, X, Y, Z).
/// Values are in half-precision floating-point format (1 sign bit, 5 exponent bits, 10 fraction bits).
/// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits)
#[named_register(address = EmbReg::SflpQuatwL, access_type = EmbedFuncState, generics = 2)]
pub struct SflpQuatWXYZ {
    pub w: u16,
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

/// SFLP_GBIASX_INIT_L to SFLP_GBIASZ_INIT_H (0x32 - 0x37)
///
/// Sensor fusion low-power gyroscope bias initialization values for axes (X, Y, Z) (R/W).
/// Values are in half-precision floating-point format (1 sign bit, 5 exponent bits, 10 fraction bits).
/// SEEEEEFFFFFFFFFF (S: 1 sign bit; E: 5 exponent bits; F: 10 fraction bits)
#[named_register(address = EmbReg::SflpGbiasxInitL, access_type = EmbedFuncState, generics = 2)]
pub struct SflpGbiasXYZInit {
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

/// EMB_FUNC_FIFO_EN_A (0x44)
///
/// Embedded functions FIFO configuration register A (R/W).
/// Enables batching of various embedded function outputs into FIFO buffer.
#[register(address = EmbReg::EmbFuncFifoEnA, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncFifoEnA {
    #[bits(1, access = RO)]
    /// Reserved bit, must be 0.
    not_used0: u8,
    /// Enables batching of sensor fusion low-power game rotation vector in FIFO. Default: 0.
    #[bits(1)]
    pub sflp_game_fifo_en: u8,
    #[bits(2, access = RO)]
    /// Reserved bits, must be 0.
    not_used1: u8,
    /// Enables batching of sensor fusion low-power gravity values in FIFO. Default: 0.
    #[bits(1)]
    pub sflp_gravity_fifo_en: u8,
    /// Enables batching of sensor fusion low-power gyroscope bias values in FIFO. Default: 0.
    #[bits(1)]
    pub sflp_gbias_fifo_en: u8,
    /// Enables batching of step counter values in FIFO. Default: 0.
    #[bits(1)]
    pub step_counter_fifo_en: u8,
    /// Enables batching of machine learning core results in FIFO. Default: 0.
    #[bits(1)]
    pub mlc_fifo_en: u8,
}

/// EMB_FUNC_FIFO_EN_B (0x45)
///
/// Embedded functions FIFO configuration register B (R/W).
#[register(address = EmbReg::EmbFuncFifoEnB, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncFifoEnB {
    #[bits(1, access = RO)]
    /// Reserved bit, must be 0.
    not_used0: u8,
    /// Enables batching of machine learning core filters and features in FIFO. Default: 0 (disabled).
    #[bits(1)]
    pub mlc_filter_feature_fifo_en: u8,
    /// Enables batching of FSM status and long counter events in FIFO. Default: 0 (disabled).
    #[bits(1)]
    pub fsm_fifo_en: u8,
    #[bits(5, access = RO)]
    /// Reserved bits, must be 0.
    not_used1: u8,
}

/// FSM_ENABLE (0x46)
///
/// Enable finite state machines (FSM) (R/W).
#[register(address = EmbReg::FsmEnable, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmEnable {
    /// Enable FSM1. Default: 0 (disabled).
    #[bits(1)]
    pub fsm1_en: u8,
    /// Enable FSM2. Default: 0 (disabled).
    #[bits(1)]
    pub fsm2_en: u8,
    /// Enable FSM3. Default: 0 (disabled).
    #[bits(1)]
    pub fsm3_en: u8,
    /// Enable FSM4. Default: 0 (disabled).
    #[bits(1)]
    pub fsm4_en: u8,
    /// Enable FSM5. Default: 0 (disabled).
    #[bits(1)]
    pub fsm5_en: u8,
    /// Enable FSM6. Default: 0 (disabled).
    #[bits(1)]
    pub fsm6_en: u8,
    /// Enable FSM7. Default: 0 (disabled).
    #[bits(1)]
    pub fsm7_en: u8,
    /// Enable FSM8. Default: 0 (disabled).
    #[bits(1)]
    pub fsm8_en: u8,
}

/// FSM_LONG_COUNTER_L to FSM_LONG_COUNTER_H (0x48 - 0x49) (R/W)
///
/// FSM long counter status register
/// The long counter value is an unsigned integer value (16-bit format).
#[register(address = EmbReg::FsmLongCounterL, access_type = EmbedFuncState, generics = 2)]
pub struct FsmLongCounter(pub u16);

/// INT_ACK_MASK (0x4B)
///
/// Interrupt acknowledge mask register (R/W).
/// Controls which bits of status registers are not reset when read in latched mode.
#[register(address = EmbReg::IntAckMask, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IntAckMask {
    /// Interrupt acknowledge mask bits for status registers.
    #[bits(8)]
    pub iack_mask: u8,
}

/// FSM_OUTS1 (0x4C)
///
/// FSM1 output register (R).
#[register(address = EmbReg::FsmOuts1, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts1 {
    /// FSM1 negative vector event detected.
    #[bits(1)]
    pub fsm1_n_v: u8,
    /// FSM1 positive vector event detected.
    #[bits(1)]
    pub fsm1_p_v: u8,
    /// FSM1 negative Z-axis event detected.
    #[bits(1)]
    pub fsm1_n_z: u8,
    /// FSM1 positive Z-axis event detected.
    #[bits(1)]
    pub fsm1_p_z: u8,
    /// FSM1 negative Y-axis event detected.
    #[bits(1)]
    pub fsm1_n_y: u8,
    /// FSM1 positive Y-axis event detected.
    #[bits(1)]
    pub fsm1_p_y: u8,
    /// FSM1 negative X-axis event detected.
    #[bits(1)]
    pub fsm1_n_x: u8,
    /// FSM1 positive X-axis event detected.
    #[bits(1)]
    pub fsm1_p_x: u8,
}

/// FSM_OUTS2 (0x4D)
///
/// FSM2 output register (R).
#[register(address = EmbReg::FsmOuts2, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts2 {
    /// FSM2 negative vector event detected.
    #[bits(1)]
    pub fsm2_n_v: u8,
    /// FSM2 positive vector event detected.
    #[bits(1)]
    pub fsm2_p_v: u8,
    /// FSM2 negative Z-axis event detected.
    #[bits(1)]
    pub fsm2_n_z: u8,
    /// FSM2 positive Z-axis event detected.
    #[bits(1)]
    pub fsm2_p_z: u8,
    /// FSM2 negative Y-axis event detected.
    #[bits(1)]
    pub fsm2_n_y: u8,
    /// FSM2 positive Y-axis event detected.
    #[bits(1)]
    pub fsm2_p_y: u8,
    /// FSM2 negative X-axis event detected.
    #[bits(1)]
    pub fsm2_n_x: u8,
    /// FSM2 positive X-axis event detected.
    #[bits(1)]
    pub fsm2_p_x: u8,
}

/// FSM_OUTS3 (0x4E)
///
/// FSM3 output register (R).
#[register(address = EmbReg::FsmOuts3, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts3 {
    /// FSM3 negative vector event detected.
    #[bits(1)]
    pub fsm3_n_v: u8,
    /// FSM3 positive vector event detected.
    #[bits(1)]
    pub fsm3_p_v: u8,
    /// FSM3 negative Z-axis event detected.
    #[bits(1)]
    pub fsm3_n_z: u8,
    /// FSM3 positive Z-axis event detected.
    #[bits(1)]
    pub fsm3_p_z: u8,
    /// FSM3 negative Y-axis event detected.
    #[bits(1)]
    pub fsm3_n_y: u8,
    /// FSM3 positive Y-axis event detected.
    #[bits(1)]
    pub fsm3_p_y: u8,
    /// FSM3 negative X-axis event detected.
    #[bits(1)]
    pub fsm3_n_x: u8,
    /// FSM3 positive X-axis event detected.
    #[bits(1)]
    pub fsm3_p_x: u8,
}

/// FSM_OUTS4 (0x4F)
///
/// FSM4 output register (R).
#[register(address = EmbReg::FsmOuts4, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts4 {
    /// FSM4 negative vector event detected.
    #[bits(1)]
    pub fsm4_n_v: u8,
    /// FSM4 positive vector event detected.
    #[bits(1)]
    pub fsm4_p_v: u8,
    /// FSM4 negative Z-axis event detected.
    #[bits(1)]
    pub fsm4_n_z: u8,
    /// FSM4 positive Z-axis event detected.
    #[bits(1)]
    pub fsm4_p_z: u8,
    /// FSM4 negative Y-axis event detected.
    #[bits(1)]
    pub fsm4_n_y: u8,
    /// FSM4 positive Y-axis event detected.
    #[bits(1)]
    pub fsm4_p_y: u8,
    /// FSM4 negative X-axis event detected.
    #[bits(1)]
    pub fsm4_n_x: u8,
    /// FSM4 positive X-axis event detected.
    #[bits(1)]
    pub fsm4_p_x: u8,
}

/// FSM_OUTS5 (0x50)
///
/// FSM5 output register (R).
#[register(address = EmbReg::FsmOuts5, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts5 {
    /// FSM5 negative vector event detected.
    #[bits(1)]
    pub fsm5_n_v: u8,
    /// FSM5 positive vector event detected.
    #[bits(1)]
    pub fsm5_p_v: u8,
    /// FSM5 negative Z-axis event detected.
    #[bits(1)]
    pub fsm5_n_z: u8,
    /// FSM5 positive Z-axis event detected.
    #[bits(1)]
    pub fsm5_p_z: u8,
    /// FSM5 negative Y-axis event detected.
    #[bits(1)]
    pub fsm5_n_y: u8,
    /// FSM5 positive Y-axis event detected.
    #[bits(1)]
    pub fsm5_p_y: u8,
    /// FSM5 negative X-axis event detected.
    #[bits(1)]
    pub fsm5_n_x: u8,
    /// FSM5 positive X-axis event detected.
    #[bits(1)]
    pub fsm5_p_x: u8,
}

/// FSM_OUTS6 (0x51)
///
/// FSM6 output register (R).
#[register(address = EmbReg::FsmOuts6, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts6 {
    /// FSM6 negative vector event detected.
    #[bits(1)]
    pub fsm6_n_v: u8,
    /// FSM6 positive vector event detected.
    #[bits(1)]
    pub fsm6_p_v: u8,
    /// FSM6 negative Z-axis event detected.
    #[bits(1)]
    pub fsm6_n_z: u8,
    /// FSM6 positive Z-axis event detected.
    #[bits(1)]
    pub fsm6_p_z: u8,
    /// FSM6 negative Y-axis event detected.
    #[bits(1)]
    pub fsm6_n_y: u8,
    /// FSM6 positive Y-axis event detected.
    #[bits(1)]
    pub fsm6_p_y: u8,
    /// FSM6 negative X-axis event detected.
    #[bits(1)]
    pub fsm6_n_x: u8,
    /// FSM6 positive X-axis event detected.
    #[bits(1)]
    pub fsm6_p_x: u8,
}

/// FSM_OUTS7 (0x52)
///
/// FSM7 output register (R).
#[register(address = EmbReg::FsmOuts7, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts7 {
    /// FSM7 negative vector event detected.
    #[bits(1)]
    pub fsm7_n_v: u8,
    /// FSM7 positive vector event detected.
    #[bits(1)]
    pub fsm7_p_v: u8,
    /// FSM7 negative Z-axis event detected.
    #[bits(1)]
    pub fsm7_n_z: u8,
    /// FSM7 positive Z-axis event detected.
    #[bits(1)]
    pub fsm7_p_z: u8,
    /// FSM7 negative Y-axis event detected.
    #[bits(1)]
    pub fsm7_n_y: u8,
    /// FSM7 positive Y-axis event detected.
    #[bits(1)]
    pub fsm7_p_y: u8,
    /// FSM7 negative X-axis event detected.
    #[bits(1)]
    pub fsm7_n_x: u8,
    /// FSM7 positive X-axis event detected.
    #[bits(1)]
    pub fsm7_p_x: u8,
}

/// FSM_OUTS8 (0x53)
///
/// FSM8 output register (R).
#[register(address = EmbReg::FsmOuts8, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOuts8 {
    /// FSM8 negative vector event detected.
    #[bits(1)]
    pub fsm8_n_v: u8,
    /// FSM8 positive vector event detected.
    #[bits(1)]
    pub fsm8_p_v: u8,
    /// FSM8 negative Z-axis event detected.
    #[bits(1)]
    pub fsm8_n_z: u8,
    /// FSM8 positive Z-axis event detected.
    #[bits(1)]
    pub fsm8_p_z: u8,
    /// FSM8 negative Y-axis event detected.
    #[bits(1)]
    pub fsm8_n_y: u8,
    /// FSM8 positive Y-axis event detected.
    #[bits(1)]
    pub fsm8_p_y: u8,
    /// FSM8 negative X-axis event detected.
    #[bits(1)]
    pub fsm8_n_x: u8,
    /// FSM8 positive X-axis event detected.
    #[bits(1)]
    pub fsm8_p_x: u8,
}

#[register(address = EmbReg::SflpOdr, access_type = EmbedFuncState, generics = 2)]

/// SFLP_ODR (0x5E)
///
/// Sensor fusion low-power game algorithm output data rate configuration (R/W).
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SflpOdr {
    /// Reserved bits, must be 0.
    #[bits(3, access = RO)]
    not_used0: u8,
    /// SFLP game algorithm ODR selection (3 bits). Default: 011 (120 Hz).
    #[bits(3)]
    pub sflp_game_odr: u8,
    /// Reserved bits, must be 0.
    #[bits(2, access = RO)]
    not_used1: u8,
}

/// FSM_ODR (0x5F)
///
/// Finite state machine output data rate configuration (R/W).
#[register(address = EmbReg::FsmOdr, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmOdr {
    /// Reserved bits, must be 0.
    #[bits(3, access = RO)]
    not_used0: u8,
    /// FSM ODR selection (3 bits). Default: 001 (30 Hz).
    #[bits(3)]
    pub fsm_odr: u8,
    /// Reserved bits, must be 0.
    #[bits(2, access = RO)]
    not_used1: u8,
}

/// MLC_ODR (0x60)
///
/// Machine learning core output data rate configuration (R/W).
#[register(address = EmbReg::MlcOdr, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcOdr {
    /// Reserved bits, must be 0.
    #[bits(4, access = RO)]
    not_used0: u8,
    /// MLC ODR selection (3 bits). Default: 001 (30 Hz).
    #[bits(3)]
    pub mlc_odr: u8,
    /// Reserved bits, must be 0.
    #[bits(1, access = RO)]
    not_used1: u8,
}

/// STEP_COUNTER_L to STEP_COUNTER_H (0x62 - 0x63)
///
/// Step counter output unsigned 16 bit (R).
#[register(address = EmbReg::StepCounterL, access_type = EmbedFuncState, generics = 2)]
pub struct StepCounter(pub u16);

/// EMB_FUNC_SRC (0x64)
///
/// Embedded function source register (R/W).
#[register(address = EmbReg::EmbFuncSrc, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncSrc {
    #[bits(2, access = RO)]
    /// Reserved bits.
    not_used0: u8,
    /// Step counter bit set flag. 1: step count increased.
    #[bits(1)]
    pub stepcounter_bit_set: u8,
    /// Step counter overflow flag. 1: overflow occurred.
    #[bits(1)]
    pub step_overflow: u8,
    /// Step count delta interrupt active flag.
    #[bits(1)]
    pub step_count_delta_ia: u8,
    /// Step detected flag. 1: step detected.
    #[bits(1)]
    pub step_detected: u8,
    #[bits(1, access = RO)]
    /// Reserved bit.
    not_used1: u8,
    /// Pedometer step counter reset. Write 1 to reset.
    #[bits(1)]
    pub pedo_rst_step: u8,
}

/// EMB_FUNC_INIT_A (0x66)
///
/// Embedded functions initialization register A (R/W).
#[register(address = EmbReg::EmbFuncInitA, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncInitA {
    #[bits(1, access = RO)]
    /// Reserved bit.
    not_used0: u8,
    /// SFLP game algorithm initialization request. Default: 0.
    #[bits(1)]
    pub sflp_game_init: u8,
    #[bits(1, access = RO)]
    /// Reserved bit.
    not_used2: u8,
    /// Pedometer step detector initialization request. Default: 0.
    #[bits(1)]
    pub step_det_init: u8,
    /// Tilt algorithm initialization request. Default: 0.
    #[bits(1)]
    pub tilt_init: u8,
    /// Significant motion detection initialization request. Default: 0.
    #[bits(1)]
    pub sig_mot_init: u8,
    #[bits(1, access = RO)]
    /// Reserved bit.
    not_used1: u8,
    /// Machine learning core before FSM initialization request. Default: 0.
    #[bits(1)]
    pub mlc_before_fsm_init: u8,
}

/// EMB_FUNC_INIT_B (0x67)
///
/// Embedded functions initialization register B (R/W).
#[register(address = EmbReg::EmbFuncInitB, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncInitB {
    /// FSM initialization request. Default: 0.
    #[bits(1)]
    pub fsm_init: u8,
    #[bits(1, access = RO)]
    /// Reserved bit.
    not_used0: u8,
    /// High-g peak tracking initialization request. Default: 0.
    #[bits(1)]
    pub pt_init: u8,
    /// FIFO compression initialization request. Default: 0.
    #[bits(1)]
    pub fifo_compr_init: u8,
    /// Machine learning core initialization request. Default: 0.
    #[bits(1)]
    pub mlc_init: u8,
    #[bits(3, access = RO)]
    /// Reserved bits.
    not_used1: u8,
}

/// EMB_FUNC_SENSOR_CONV_EN (0x6E)
///
/// Embedded functions sensor conversion enable/disable register (R/W).
#[register(address = EmbReg::EmbFuncSensorConvEn, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncSensorConvEn {
    /// High-g accelerometer data conversion enable. Default: 1.
    #[bits(1)]
    pub xl_hg_conv_en: u8,
    /// Gyroscope data conversion enable. Default: 1.
    #[bits(1)]
    pub gyro_conv_en: u8,
    /// Temperature data conversion enable. Default: 1.
    #[bits(1)]
    pub temp_conv_en: u8,
    /// External sensor data conversion enable. Default: 1.
    #[bits(1)]
    pub ext_sensor_conv_en: u8,
    #[bits(4, access = RO)]
    /// Reserved bits.
    not_used0: u8,
}

/// MLC1_SRC (0x70)
///
/// Machine learning core source register 1 (R).
#[register(address = EmbReg::Mlc1Src, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Mlc1Src {
    /// Output value of MLC1 decision tree.
    #[bits(8)]
    pub mlc1_src: u8,
}

/// MLC2_SRC (0x71)
///
/// Machine learning core source register 2 (R).
#[register(address = EmbReg::Mlc2Src, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Mlc2Src {
    /// Output value of MLC2 decision tree.
    #[bits(8)]
    pub mlc2_src: u8,
}

/// MLC3_SRC (0x72)
///
/// Machine learning core source register 3 (R).
#[register(address = EmbReg::Mlc3Src, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Mlc3Src {
    /// Output value of MLC3 decision tree.
    #[bits(8)]
    pub mlc3_src: u8,
}

/// MLC4_SRC (0x73)
///
/// Machine learning core source register 4 (R).
#[register(address = EmbReg::Mlc4Src, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Mlc4Src {
    /// Output value of MLC4 decision tree.
    #[bits(8)]
    pub mlc4_src: u8,
}

/// MLC5_SRC (0x74)
///
/// Machine learning core source register 5 (R).
#[register(address = EmbReg::Mlc5Src, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Mlc5Src {
    /// Output value of MLC5 decision tree.
    #[bits(8)]
    pub mlc5_src: u8,
}

/// MLC6_SRC (0x75)
///
/// Machine learning core source register 6 (R).
#[register(address = EmbReg::Mlc6Src, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Mlc6Src {
    /// Output value of MLC6 decision tree.
    #[bits(8)]
    pub mlc6_src: u8,
}

/// MLC7_SRC (0x76)
///
/// Machine learning core source register 7 (R).
#[register(address = EmbReg::Mlc7Src, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Mlc7Src {
    /// Output value of MLC7 decision tree.
    #[bits(8)]
    pub mlc7_src: u8,
}

/// MLC8_SRC (0x77)
///
/// Machine learning core source register 8 (R).
#[register(address = EmbReg::Mlc8Src, access_type = EmbedFuncState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Mlc8Src {
    /// Output value of MLC8 decision tree.
    #[bits(8)]
    pub mlc8_src: u8,
}

#[derive(Default, MultiRegister)]
pub struct MlcOut {
    pub mlc1_src: u8,
    pub mlc2_src: u8,
    pub mlc3_src: u8,
    pub mlc4_src: u8,
    pub mlc5_src: u8,
    pub mlc6_src: u8,
    pub mlc7_src: u8,
    pub mlc8_src: u8,
}

#[derive(Default)]
pub struct EmbFuncConv {
    pub xl_hg_conv_en: u8,
    pub gyro_conv_en: u8,
    pub temp_conv_en: u8,
    pub ext_sensor_conv_en: u8,
}

#[derive(Default)]
pub struct FifoSflpRaw {
    pub game_rotation: u8,
    pub gravity: u8,
    pub gbias: u8,
}

#[derive(Default)]
pub struct FsmMode {
    pub fsm1_en: u8,
    pub fsm2_en: u8,
    pub fsm3_en: u8,
    pub fsm4_en: u8,
    pub fsm5_en: u8,
    pub fsm6_en: u8,
    pub fsm7_en: u8,
    pub fsm8_en: u8,
}

#[derive(Default, MultiRegister)]
pub struct FsmOut {
    pub fsm_outs1: u8,
    pub fsm_outs2: u8,
    pub fsm_outs3: u8,
    pub fsm_outs4: u8,
    pub fsm_outs5: u8,
    pub fsm_outs6: u8,
    pub fsm_outs7: u8,
    pub fsm_outs8: u8,
}

#[derive(Default)]
pub struct StpcntMode {
    pub step_counter_enable: u8,
    pub false_step_rej: u8,
}

#[derive(Default)]
pub struct SflpGbias {
    pub gbias_x: f32,
    pub gbias_y: f32,
    pub gbias_z: f32,
}

#[derive(Default)]
pub struct Quaternion {
    pub quat_w: f32,
    pub quat_x: f32,
    pub quat_y: f32,
    pub quat_z: f32,
}

/// Finite State Machine (FSM) output data rate configuration.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FsmDataRate {
    /// 15 Hz output data rate (default).
    #[default]
    _15hz = 0x0,
    /// 30 Hz output data rate.
    _30hz = 0x1,
    /// 60 Hz output data rate.
    _60hz = 0x2,
    /// 120 Hz output data rate.
    _120hz = 0x3,
    /// 240 Hz output data rate.
    _240hz = 0x4,
    /// 480 Hz output data rate.
    _480hz = 0x5,
    /// 960 Hz output data rate.
    _960hz = 0x6,
}

/// Machine Learning Core (MLC) mode.
///
/// - `Off`: MLC disabled.
/// - `On`: MLC enabled after FSM programs.
/// - `OnBeforeFsm`: MLC enabled before FSM programs.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum MlcMode {
    /// MLC disabled.
    Off = 0x0,
    /// MLC enabled after FSM programs.
    On = 0x1,
    /// MLC enabled before FSM programs.
    OnBeforeFsm = 0x2,
}

/// Machine Learning Core (MLC) output data rate configuration.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum MlcDataRate {
    /// 15 Hz output data rate (default).
    #[default]
    _15hz = 0x0,
    /// 30 Hz output data rate.
    _30hz = 0x1,
    /// 60 Hz output data rate.
    _60hz = 0x2,
    /// 120 Hz output data rate.
    _120hz = 0x3,
    /// 240 Hz output data rate.
    _240hz = 0x4,
    /// 480 Hz output data rate.
    _480hz = 0x5,
    /// 960 Hz output data rate.
    _960hz = 0x6,
}

/// Sensor fusion low-power (SFLP) algorithm output data rate.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum SflpDataRate {
    /// 15 Hz output data rate.
    #[default]
    _15hz = 0x0,
    /// 30 Hz output data rate.
    _30hz = 0x1,
    /// 60 Hz output data rate.
    _60hz = 0x2,
    /// 120 Hz output data rate.
    _120hz = 0x3,
    /// 240 Hz output data rate.
    _240hz = 0x4,
    /// 480 Hz output data rate.
    _480hz = 0x5,
}
