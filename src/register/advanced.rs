use super::super::{
    BusOperation, DelayNs, EmbAdvFunctions, Error, Lsm6dsv320x, RegisterOperation, bisync,
    register::MainBank,
};

use bitfield_struct::bitfield;

use derive_more::TryFrom;
use st_mem_bank_macro::{MultiRegister, adv_register};

#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum AdvPage {
    _0 = 0x000,
    _1 = 0x100,
    _2 = 0x200,
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum EmbAdv0Reg {
    FsmExtSensitivityL = 0xBA,
    FsmExtSensitivityH = 0xBB,
    FsmExtOffxL = 0xC0,
    FsmExtOffxH = 0xC1,
    FsmExtOffyL = 0xC2,
    FsmExtOffyH = 0xC3,
    FsmExtOffzL = 0xC4,
    FsmExtOffzH = 0xC5,
    FsmExtMatrixXxL = 0xC6,
    FsmExtMatrixXxH = 0xC7,
    FsmExtMatrixXyL = 0xC8,
    FsmExtMatrixXyH = 0xC9,
    FsmExtMatrixXzL = 0xCA,
    FsmExtMatrixXzH = 0xCB,
    FsmExtMatrixYyL = 0xCC,
    FsmExtMatrixYyH = 0xCD,
    FsmExtMatrixYzL = 0xCE,
    FsmExtMatrixYzH = 0xCF,
    FsmExtMatrixZzL = 0xD0,
    FsmExtMatrixZzH = 0xD1,
    ExtCfgA = 0xD4,
    ExtCfgB = 0xD5,
}

#[repr(u16)]
#[derive(Clone, Copy, PartialEq)]
pub enum EmbAdv1Reg {
    XlHgSensitivityL = 0x58,
    XlHgSensitivityH = 0x59,
    FsmLcTimeoutL = 0x7A,
    FsmLcTimeoutH = 0x7B,
    FsmPrograms = 0x7C,
    FsmStartAddL = 0x7E,
    FsmStartAddH = 0x7F,
    PedoCmdReg = 0x83,
    PedoDebStepsConf = 0x84,
    PedoScDeltatL = 0xD0,
    PedoScDeltatH = 0xD1,
    MlcExtSensitivityL = 0xE8,
    MlcExtSensitivityH = 0xE9,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum EmbAdv2Reg {
    Ext3byteSensitivityL = 0x02,
    Ext3byteSensitivityH = 0x03,
    Ext3byteOffsetXl = 0x06,
    Ext3byteOffsetL = 0x07,
    Ext3byteOffsetH = 0x08,
}

/// FSM_EXT_SENSITIVITY (0xBA - 0xBB)
///
/// External sensor sensitivity value for FSM (R/W).
/// 16-bit half-precision floating-point format.
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtSensitivityL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
pub struct FsmExtSensitivity(pub u16);

/// FSM_EXT_SENSITIVITY_L (0xBA)
///
/// Low byte of external sensor sensitivity (R).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtSensitivityL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtSensitivityL {
    #[bits(8)]
    pub fsm_ext_s: u8,
}

/// FSM_EXT_SENSITIVITY_H (0xBB)
///
/// High byte of external sensor sensitivity (R).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtSensitivityH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtSensitivityH {
    #[bits(8)]
    pub fsm_ext_s: u8,
}

/// FSM_EXT_OFFX_L (0xC0)
///
/// Low byte of external sensor X-axis offset (R/W).
/// 16-bit half-precision floating-point format.
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtOffxL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtOffxL {
    #[bits(8)]
    pub fsm_ext_offx: u8,
}

/// FSM_EXT_OFFX_H (0xC1)
///
/// High byte of external sensor X-axis offset (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtOffxH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtOffxH {
    #[bits(8)]
    pub fsm_ext_offx: u8,
}

/// FSM_EXT_OFFY_L (0xC2)
///
/// Low byte of external sensor Y-axis offset (R/W).
/// 16-bit half-precision floating-point format.
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtOffyL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtOffyL {
    #[bits(8)]
    pub fsm_ext_offy: u8,
}

/// FSM_EXT_OFFY_H (0xC3)
///
/// High byte of external sensor Y-axis offset (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtOffyH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtOffyH {
    #[bits(8)]
    pub fsm_ext_offy: u8,
}

/// FSM_EXT_OFFZ_L (0xC4)
///
/// Low byte of external sensor Z-axis offset (R/W).
/// 16-bit half-precision floating-point format.
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtOffzL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtOffzL {
    #[bits(8)]
    pub fsm_ext_offz: u8,
}

/// FSM_EXT_OFFZ_H (0xC5)
///
/// High byte of external sensor Z-axis offset (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtOffzH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtOffzH {
    #[bits(8)]
    pub fsm_ext_offz: u8,
}

/// FSM_EXT_MATRIX_XX_L (0xC6)
///
/// Low byte of external sensor transformation matrix row 1 column 1 coefficient (R/W).
/// 16-bit half-precision floating-point format.
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixXxL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixXxL {
    #[bits(8)]
    pub fsm_ext_mat_xx: u8,
}

/// FSM_EXT_MATRIX_XX_H (0xC7)
///
/// High byte of external sensor transformation matrix row 1 column 1 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixXxH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixXxH {
    #[bits(8, default = 0x3C)]
    pub fsm_ext_mat_xx: u8,
}

/// FSM_EXT_MATRIX_XY_L (0xC8)
///
/// Low byte of external sensor transformation matrix row 1 column 2 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixXyL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixXyL {
    #[bits(8)]
    pub fsm_ext_mat_xy: u8,
}

/// FSM_EXT_MATRIX_XY_H (0xC9)
///
/// High byte of external sensor transformation matrix row 1 column 2 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixXyH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixXyH {
    #[bits(8)]
    pub fsm_ext_mat_xy: u8,
}

/// FSM_EXT_MATRIX_XZ_L (0xCA)
///
/// Low byte of external sensor transformation matrix row 1 column 3 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixXzL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixXzL {
    #[bits(8)]
    pub fsm_ext_mat_xz: u8,
}

/// FSM_EXT_MATRIX_XZ_H (0xCB)
///
/// High byte of external sensor transformation matrix row 1 column 3 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixXzH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixXzH {
    #[bits(8)]
    pub fsm_ext_mat_xz: u8,
}

/// FSM_EXT_MATRIX_YY_L (0xCC)
///
/// Low byte of external sensor transformation matrix row 2 column 2 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixYyL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixYyL {
    #[bits(8)]
    pub fsm_ext_mat_yy: u8,
}

/// FSM_EXT_MATRIX_YY_H (0xCD)
///
/// High byte of external sensor transformation matrix row 2 column 2 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixYyH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixYyH {
    #[bits(8, default = 0x3C)]
    pub fsm_ext_mat_yy: u8,
}

/// FSM_EXT_MATRIX_YZ_L (0xCE)
///
/// Low byte of external sensor transformation matrix row 2 column 3 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixYzL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixYzL {
    #[bits(8)]
    pub fsm_ext_mat_yz: u8,
}

/// FSM_EXT_MATRIX_YZ_H (0xCF)
///
/// High byte of external sensor transformation matrix row 2 column 3 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixYzH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixYzH {
    #[bits(8)]
    pub fsm_ext_mat_yz: u8,
}

/// FSM_EXT_MATRIX_ZZ_L (0xD0)
///
/// Low byte of external sensor transformation matrix row 3 column 3 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixZzL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixZzL {
    #[bits(8)]
    pub fsm_ext_mat_zz: u8,
}

/// FSM_EXT_MATRIX_ZZ_H (0xD1)
///
/// High byte of external sensor transformation matrix row 3 column 3 coefficient (R/W).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::FsmExtMatrixZzH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmExtMatrixZzH {
    #[bits(8, default = 0x3C)]
    pub fsm_ext_mat_zz: u8,
}

/// EXT_CFG_A (0xD4)
///
/// External sensor coordinates rotation register (R/W).
/// Controls Z and Y axes rotation to align external sensor to accelerometer/gyroscope axes.
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::ExtCfgA, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct ExtCfgA {
    /// External sensor Z-axis rotation (3 bits).
    #[bits(3, default = 0b101)]
    pub ext_z_axis: u8,
    #[bits(1, access = RO)]
    /// Reserved bit, must be 0.
    not_used0: u8,
    /// External sensor Y-axis rotation (3 bits).
    #[bits(3)]
    pub ext_y_axis: u8,
    #[bits(1, access = RO)]
    /// Reserved bit, must be 0.
    not_used1: u8,
}

/// EXT_CFG_B (0xD5)
///
/// External sensor X-axis coordinates rotation register (R/W).
/// Controls X-axis rotation to align external sensor to accelerometer/gyroscope axes.
/// Default: 010 (X = X).
#[adv_register(base_address =  AdvPage::_0, address = EmbAdv0Reg::ExtCfgB, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct ExtCfgB {
    /// External sensor X-axis rotation (3 bits).
    #[bits(3, default = 0b010)]
    pub ext_x_axis: u8,
    #[bits(5, access = RO)]
    /// Reserved bits, must be 0.
    not_used0: u8,
}

/// XL_HG_SENSITIVITY (0x58)
///
/// High-g accelerometer sensitivity value register for FSM and MLC (R/W).
/// 16-bit half-precision floating-point format.
/// Default: 0x051F (expressed in hundreds of g).
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::XlHgSensitivityL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct XlHgSensitivity {
    #[bits(16, default = 0x051F)]
    pub xl_hg: u16,
}

/// FSM_LC_TIMEOUT (0x7A)
///
/// FSM long counter timeout value (R/W).
/// 16-bit unsigned integer. When long counter reaches this value, FSM generates interrupt.
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::FsmLcTimeoutL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
pub struct FsmLcTimeout(pub u16);

/// FSM_LC_TIMEOUT_L (0x7A)
///
/// Low byte of FSM long counter timeout value (R).
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::FsmLcTimeoutL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmLcTimeoutL {
    #[bits(8)]
    pub fsm_lc_timeout: u8,
}

/// FSM_LC_TIMEOUT_H (0x7B)
///
/// High byte of FSM long counter timeout value (R).
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::FsmLcTimeoutH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmLcTimeoutH {
    #[bits(8)]
    pub fsm_lc_timeout: u8,
}

/// FSM_PROGRAMS (0x7C)
///
/// Number of FSM programs (R/W).
/// Must be less than or equal to 8. Default: 0.
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::FsmPrograms, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmPrograms {
    #[bits(8)]
    pub fsm_n_prog: u8,
}

/// FSM_START_ADD (0x7E)
///
/// FSM start address register (R/W).
/// First available address is 0x2F0.
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::FsmStartAddL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
pub struct FsmStartAdd(pub u16);

/// FSM_START_ADD_L (0x7E)
///
/// Low byte of FSM start address (R).
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::FsmStartAddL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmStartAddL {
    #[bits(8)]
    pub fsm_start: u8,
}

/// FSM_START_ADD_H (0x7F)
///
/// High byte of FSM start address (R).
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::FsmStartAddH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmStartAddH {
    #[bits(8)]
    pub fsm_start: u8,
}

/// PEDO_CMD_REG (0x83)
///
/// Pedometer configuration register (R/W).
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::PedoCmdReg, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PedoCmdReg {
    #[bits(2, access = RO)]
    /// Reserved bits, must be 0.
    not_used0: u8,
    /// Enables false-positive rejection feature. Default: 0.
    #[bits(1)]
    pub fp_rejection_en: u8,
    /// Enables interrupt only on count overflow event. Default: 0.
    #[bits(1)]
    pub carry_count_en: u8,
    #[bits(4, access = RO)]
    /// Reserved bits, must be 0.
    not_used1: u8,
}

/// PEDO_DEB_STEPS_CONF (0x84)
///
/// Pedometer debounce configuration register (R/W).
/// Minimum number of steps to increment the step counter (debounce).
/// Default: 0x0A.
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::PedoDebStepsConf, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PedoDebStepsConf {
    #[bits(8, default = 0x0A)]
    pub deb_step: u8,
}

/// PEDO_SC_DELTAT (0xD0)
///
/// Time period register for step detection on delta time (R/W).
/// 16-bit value, 1 LSB = 6.4 ms.
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::PedoScDeltatL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
pub struct PedoScDeltat(pub u16);

/// PEDO_SC_DELTAT_L (0xD0)
///
/// Low byte of time period for step detection on delta time.
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::PedoScDeltatL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PedoScDeltatL {
    #[bits(8)]
    pub pd_sc: u8,
}

/// PEDO_SC_DELTAT_H (0xD1)
///
/// High byte of time period for step detection on delta time.
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::PedoScDeltatH, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PedoScDeltatH {
    #[bits(8)]
    pub pd_sc: u8,
}

/// MLC_EXT_SENSITIVITY (0xE8)
///
/// External sensor sensitivity value register for machine learning core (R/W).
/// 16-bit half-precision floating-point format.
/// Default: 0x3C00 (1 gauss/LSB).
#[adv_register(base_address =  AdvPage::_1, address = EmbAdv1Reg::MlcExtSensitivityL, access_type = "Lsm6dsv320x<B, T, MainBank>")]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct MlcExtSensitivity {
    #[bits(16, default = 0x3C00)]
    pub mlc_ext: u16,
}

#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
/// EXT_FORMAT (0x00)
///
/// External sensor data format selection for FSM and MLC (R/W).
/// 0: 2-byte format (default), 1: 3-byte format.
pub struct ExtFormat {
    #[bits(1, access = RO)]
    not_used0: u8,
    #[bits(1)]
    pub ext_format_sel: u8,
    #[bits(6, access = RO)]
    not_used1: u8,
}

#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
/// EXT_3BYTE_SENSITIVITY_L (0x02)
///
/// Low byte of external sensor sensitivity for 3-byte output data (R/W).
pub struct Ext3byteSensitivityL {
    #[bits(8)]
    pub ext_3byte_s: u8,
}

#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
/// EXT_3BYTE_SENSITIVITY_H (0x03)
///
/// High byte of external sensor sensitivity for 3-byte output data (R/W).
pub struct Ext3byteSensitivityH {
    #[bits(8, default = 0x0C)]
    pub ext_3byte_s: u8,
}

#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
/// EXT_3BYTE_OFFSET_XL (0x06)
///
/// Low byte of external sensor offset for 3-byte output data (R/W).
pub struct Ext3byteOffsetXl {
    #[bits(8)]
    pub ext_3byteoff: u8,
}

#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
/// EXT_3BYTE_OFFSET_L (0x07)
///
/// Middle byte of external sensor offset for 3-byte output data (R/W).
pub struct Ext3byteOffsetL {
    #[bits(8, default = 0x54)]
    pub ext_3byte_off: u8,
}

#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
/// EXT_3BYTE_OFFSET_H (0x08)
///
/// High byte of external sensor offset for 3-byte output data (R/W).
pub struct Ext3byteOffsetH {
    #[bits(8, default = 0x3F)]
    pub ext_3byte_off: u8,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct XlFsmExtSensOffset {
    pub z: u16,
    pub y: u16,
    pub x: u16,
}

/// External sensor Z-axis orientation for FSM and MLC.
///
/// Defines how external sensor Z-axis is mapped to device axes.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, Debug, TryFrom)]
#[try_from(repr)]
pub enum FsmExtSensZOrient {
    /// Z axis equals Y axis (default).
    #[default]
    ZEqY = 0x0,
    /// Z axis equals negative Y axis.
    ZEqMinY = 0x1,
    /// Z axis equals X axis.
    ZEqX = 0x2,
    /// Z axis equals negative X axis.
    ZEqMinX = 0x3,
    /// Z axis equals negative Z axis.
    ZEqMinZ = 0x4,
    /// Z axis equals Z axis.
    ZEqZ = 0x5,
}

/// External sensor Y-axis orientation for FSM and MLC.
///
/// Defines how external sensor Y-axis is mapped to device axes.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, Debug, TryFrom)]
#[try_from(repr)]
pub enum FsmExtSensYOrient {
    /// Y axis equals Y axis (default).
    #[default]
    YEqY = 0x0,
    /// Y axis equals negative Y axis.
    YEqMinY = 0x1,
    /// Y axis equals X axis.
    YEqX = 0x2,
    /// Y axis equals negative X axis.
    YEqMinX = 0x3,
    /// Y axis equals negative Z axis.
    YEqMinZ = 0x4,
    /// Y axis equals Z axis.
    YEqZ = 0x5,
}

/// External sensor X-axis orientation for FSM and MLC.
///
/// Defines how external sensor X-axis is mapped to device axes.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, Debug, TryFrom)]
#[try_from(repr)]
pub enum FsmExtSensXOrient {
    /// X axis equals Y axis (default).
    #[default]
    XEqY = 0x0,
    /// X axis equals negative Y axis.
    XEqMinY = 0x1,
    /// X axis equals X axis.
    XEqX = 0x2,
    /// X axis equals negative X axis.
    XEqMinX = 0x3,
    /// X axis equals negative Z axis.
    XEqMinZ = 0x4,
    /// X axis equals Z axis.
    XEqZ = 0x5,
}

#[derive(Clone, Copy, Default, Debug, PartialEq, MultiRegister)]
pub struct XlFsmExtSensMatrix {
    pub xx: u16,
    pub xy: u16,
    pub xz: u16,
    pub yy: u16,
    pub yz: u16,
    pub zz: u16,
}
