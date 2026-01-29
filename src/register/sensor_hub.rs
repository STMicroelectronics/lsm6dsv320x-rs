use crate::Error;
use crate::register::SensorHubState;
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::register;
use st_mems_bus::BusOperation;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum SensHubReg {
    SensorHub1 = 0x2,
    SensorHub2 = 0x3,
    SensorHub3 = 0x4,
    SensorHub4 = 0x5,
    SensorHub5 = 0x6,
    SensorHub6 = 0x7,
    SensorHub7 = 0x8,
    SensorHub8 = 0x9,
    SensorHub9 = 0x0A,
    SensorHub10 = 0x0B,
    SensorHub11 = 0x0C,
    SensorHub12 = 0x0D,
    SensorHub13 = 0x0E,
    SensorHub14 = 0x0F,
    SensorHub15 = 0x10,
    SensorHub16 = 0x11,
    SensorHub17 = 0x12,
    SensorHub18 = 0x13,
    ControllerConfig = 0x14,
    Tgt0Add = 0x15,
    Tgt0Subadd = 0x16,
    Tgt0Config = 0x17,
    Tgt1Add = 0x18,
    Tgt1Subadd = 0x19,
    Tgt1Config = 0x1A,
    Tgt2Add = 0x1B,
    Tgt2Subadd = 0x1C,
    Tgt2Config = 0x1D,
    Tgt3Add = 0x1E,
    Tgt3Subadd = 0x1F,
    Tgt3Config = 0x20,
    DatawriteTgt0 = 0x21,
    StatusController = 0x22,
}

/// SENSOR_HUB_1 (0x02)
///
/// First byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub1, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub1 {
    #[bits(8)]
    pub sensorhub1: u8,
}

/// SENSOR_HUB_2 (0x03)
///
/// Second byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub2, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub2 {
    #[bits(8)]
    pub sensorhub2: u8,
}

/// SENSOR_HUB_3 (0x04)
///
/// Third byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub3, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub3 {
    #[bits(8)]
    pub sensorhub3: u8,
}

/// SENSOR_HUB_4 (0x05)
///
/// Fourth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub4, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub4 {
    #[bits(8)]
    pub sensorhub4: u8,
}

/// SENSOR_HUB_5 (0x06)
///
/// Fifth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub5, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub5 {
    #[bits(8)]
    pub sensorhub5: u8,
}

/// SENSOR_HUB_6 (0x07)
///
/// Sixth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub6, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub6 {
    #[bits(8)]
    pub sensorhub6: u8,
}

/// SENSOR_HUB_7 (0x08)
///
/// Seventh byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub7, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub7 {
    #[bits(8)]
    pub sensorhub7: u8,
}

/// SENSOR_HUB_8 (0x09)
///
/// Eighth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub8, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub8 {
    #[bits(8)]
    pub sensorhub8: u8,
}

/// SENSOR_HUB_9 (0x0A)
///
/// Ninth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub9, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub9 {
    #[bits(8)]
    pub sensorhub9: u8,
}

/// SENSOR_HUB_10 (0x0B)
///
/// Tenth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub10, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub10 {
    #[bits(8)]
    pub sensorhub10: u8,
}

/// SENSOR_HUB_11 (0x0C)
///
/// Eleventh byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub11, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub11 {
    #[bits(8)]
    pub sensorhub11: u8,
}

/// SENSOR_HUB_12 (0x0D)
///
/// Twelfth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub12, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub12 {
    #[bits(8)]
    pub sensorhub12: u8,
}

/// SENSOR_HUB_13 (0x0E)
///
/// Thirteenth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub13, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub13 {
    #[bits(8)]
    pub sensorhub13: u8,
}

/// SENSOR_HUB_14 (0x0F)
///
/// Fourteenth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub14, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub14 {
    #[bits(8)]
    pub sensorhub14: u8,
}

/// SENSOR_HUB_15 (0x10)
///
/// Fifteenth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub15, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub15 {
    #[bits(8)]
    pub sensorhub15: u8,
}

/// SENSOR_HUB_16 (0x11)
///
/// Sixteenth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub16, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub16 {
    #[bits(8)]
    pub sensorhub16: u8,
}

/// SENSOR_HUB_17 (0x12)
///
/// Seventeenth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub17, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub17 {
    #[bits(8)]
    pub sensorhub17: u8,
}

/// SENSOR_HUB_18 (0x13)
///
/// Eighteenth byte associated to external sensors (R).
#[register(address = SensHubReg::SensorHub18, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct SensorHub18 {
    #[bits(8)]
    pub sensorhub18: u8,
}

/// CONTROLLER_CONFIG (0x14)
///
/// Sensor hub controller configuration register (R/W).
#[register(address = SensHubReg::ControllerConfig, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct ControllerConfig {
    /// Number of external sensors to be read by sensor hub (2 bits).
    #[bits(2)]
    pub aux_sens_on: u8,
    /// Enables sensor hub I2C controller. Default: 0 (disabled).
    #[bits(1)]
    pub controller_on: u8,
    #[bits(1, access = RO)]
    /// Reserved bit, must be 0.
    not_used0: u8,
    /// I2C interface pass-through mode enable. Default: 0 (disabled).
    #[bits(1)]
    pub pass_through_mode: u8,
    /// Sensor hub trigger signal selection. Default: 0 (accelerometer/gyro data-ready).
    #[bits(1)]
    pub start_config: u8,
    /// Target 0 write operation performed only at first sensor hub cycle. Default: 0.
    #[bits(1)]
    pub write_once: u8,
    /// Resets controller logic and output registers. Must be set to 1 then 0. Default: 0.
    #[bits(1)]
    pub rst_controller_regs: u8,
}

#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TgtAdd {
    /// Read/write operation on sensor idx . 0: write, 1: read.
    #[bits(1)]
    pub rw_0: u8,
    /// I2C target address of sensor idx (7 bits).
    #[bits(7)]
    pub target0_add: u8,
}

/// TGT0_ADD (0x15)
///
/// I2C target address of first external sensor (sensor 0) (R/W).
#[register(address = SensHubReg::Tgt0Add, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt0Add {
    /// Read/write operation on sensor 0. 0: write, 1: read.
    #[bits(1)]
    pub rw_0: u8,
    /// I2C target address of sensor 0 (7 bits).
    #[bits(7)]
    pub target0_add: u8,
}

/// TGT0_SUBADD (0x16)
///
/// Address of register on first external sensor (sensor 0) (R/W).
#[register(address = SensHubReg::Tgt0Subadd, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt0Subadd {
    /// Register address on sensor 0.
    #[bits(8)]
    pub target0_reg: u8,
}

/// TGT0_CONFIG (0x17)
///
/// First external sensor (sensor 0) configuration and sensor hub settings register (R/W).
#[register(address = SensHubReg::Tgt0Config, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt0Config {
    /// Number of read operations on sensor 0 (3 bits). Default: 000.
    #[bits(3)]
    pub target0_numop: u8,
    /// Enables FIFO data batching of first target. Default: 0 (disabled).
    #[bits(1)]
    pub batch_ext_sens_0_en: u8,
    #[bits(1, access = RO)]
    /// Reserved bit, must be 0.
    not_used0: u8,
    /// Rate at which the controller communicates (3 bits). Default: 100 (120 Hz).
    #[bits(3)]
    pub shub_odr: u8,
}

/// TGT1_ADD (0x18)
///
/// I2C target address of the second external sensor (sensor 1) (R/W).
#[register(address = SensHubReg::Tgt1Add, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt1Add {
    /// Enables read operation on sensor 1. Default: 0 (disabled).
    #[bits(1)]
    pub r_1: u8,
    /// I2C target address of sensor 1 (7 bits).
    #[bits(7)]
    pub target1_add: u8,
}

/// TGT1_SUBADD (0x19)
///
/// Address of register on the second external sensor (sensor 1) (R/W).
#[register(address = SensHubReg::Tgt1Subadd, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt1Subadd {
    /// Register address on sensor 1.
    #[bits(8)]
    pub target1_reg: u8,
}

/// TGT1_CONFIG (0x1A)
///
/// Second external sensor (sensor 1) configuration register (R/W).
#[register(address = SensHubReg::Tgt1Config, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt1Config {
    /// Number of read operations on sensor 1 (3 bits). Default: 000.
    #[bits(3)]
    pub target1_numop: u8,
    /// Enables FIFO data batching of second target. Default: 0 (disabled).
    #[bits(1)]
    pub batch_ext_sens_1_en: u8,
    #[bits(4, access = RO)]
    /// Reserved bits, must be 0.
    not_used0: u8,
}

/// TGT2_ADD (0x1B)
///
/// I2C target address of the third external sensor (sensor 2) (R/W).
#[register(address = SensHubReg::Tgt2Add, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt2Add {
    /// Enables read operation on sensor 2. Default: 0 (disabled).
    #[bits(1)]
    pub r_2: u8,
    /// I2C target address of sensor 2 (7 bits).
    #[bits(7)]
    pub target2_add: u8,
}

/// TGT2_SUBADD (0x1C)
///
/// Address of register on the third external sensor (sensor 2) (R/W).
#[register(address = SensHubReg::Tgt2Subadd, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt2Subadd {
    /// Register address on sensor 2.
    #[bits(8)]
    pub target2_reg: u8,
}

/// TGT2_CONFIG (0x1D)
///
/// Third external sensor (sensor 2) configuration register (R/W).
#[register(address = SensHubReg::Tgt2Config, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt2Config {
    /// Number of read operations on sensor 2 (3 bits). Default: 000.
    #[bits(3)]
    pub target2_numop: u8,
    /// Enables FIFO data batching of third target. Default: 0 (disabled).
    #[bits(1)]
    pub batch_ext_sens_2_en: u8,
    #[bits(4, access = RO)]
    /// Reserved bits, must be 0.
    not_used0: u8,
}

/// TGT3_ADD (0x1E)
///
/// I2C target address of the fourth external sensor (sensor 3) (R/W).
#[register(address = SensHubReg::Tgt3Add, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt3Add {
    /// Enables read operation on sensor 3. Default: 0 (disabled).
    #[bits(1)]
    pub r_3: u8,
    /// I2C target address of sensor 3 (7 bits).
    #[bits(7)]
    pub target3_add: u8,
}

/// TGT3_SUBADD (0x1F)
///
/// Address of register on the fourth external sensor (sensor 3) (R/W).
#[register(address = SensHubReg::Tgt3Subadd, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt3Subadd {
    /// Register address on sensor 3.
    #[bits(8)]
    pub target3_reg: u8,
}

/// TGT3_CONFIG (0x20)
///
/// Fourth external sensor (sensor 3) configuration register (R/W).
#[register(address = SensHubReg::Tgt3Config, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Tgt3Config {
    /// Number of read operations on sensor 3 (3 bits). Default: 000.
    #[bits(3)]
    pub target3_numop: u8,
    /// Enables FIFO data batching of fourth target. Default: 0 (disabled).
    #[bits(1)]
    pub batch_ext_sens_3_en: u8,
    #[bits(4, access = RO)]
    /// Reserved bits, must be 0.
    not_used0: u8,
}

/// DATAWRITE_TGT0 (0x21)
///
/// Data to be written into the target 0 device (R/W).
#[register(address = SensHubReg::DatawriteTgt0, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct DatawriteTgt0 {
    #[bits(8)]
    pub target0_dataw: u8,
}

/// STATUS_CONTROLLER (0x22)
///
/// Sensor hub source register (R).
#[register(address = SensHubReg::StatusController, access_type = SensorHubState, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct StatusController {
    /// Sensor hub communication concluded. 1: concluded.
    #[bits(1)]
    pub sens_hub_endop: u8,
    /// Reserved bits.
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Not acknowledge on target 0 communication. 1: NACK occurred.
    #[bits(1)]
    pub target0_nack: u8,
    /// Not acknowledge on target 1 communication. 1: NACK occurred.
    #[bits(1)]
    pub target1_nack: u8,
    /// Not acknowledge on target 2 communication. 1: NACK occurred.
    #[bits(1)]
    pub target2_nack: u8,
    /// Not acknowledge on target 3 communication. 1: NACK occurred.
    #[bits(1)]
    pub target3_nack: u8,
    /// Write once done flag. 1: write operation on target 0 completed.
    #[bits(1)]
    pub wr_once_done: u8,
}

#[derive(Default)]
pub struct ShCfgWrite {
    pub tgt0_add: u8,
    pub tgt0_subadd: u8,
    pub tgt0_data: u8,
}

#[derive(Default)]
pub struct ShCfgRead {
    pub tgt_add: u8,
    pub tgt_subadd: u8,
    pub tgt_len: u8,
}

/// Number of external sensors connected to the sensor hub.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ShTargetConnected {
    /// No external sensors connected.
    #[default]
    _0 = 0x0,
    /// One external sensor connected.
    _01 = 0x1,
    /// Two external sensors connected.
    _012 = 0x2,
    /// Three external sensors connected.
    _0123 = 0x3,
}

/// Sensor hub trigger signal selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ShSyncroMode {
    /// Sensor hub trigger is accelerometer/gyroscope data-ready.
    #[default]
    ShTrigXlGyDrdy = 0x0,
    /// Sensor hub trigger is external from INT2 pin.
    ShTrigInt2 = 0x1,
}

/// Sensor hub write mode for target 0.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ShWriteMode {
    /// Write operation for each sensor hub cycle.
    #[default]
    EachShCycle = 0x0,
    /// Write operation only for the first sensor hub cycle.
    OnlyFirstCycle = 0x1,
}

/// Sensor hub communication data rate.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ShDataRate {
    /// 1.875 Hz data rate.
    _1_875hz = 0x0,
    /// 15 Hz data rate.
    _15hz = 0x1,
    /// 30 Hz data rate.
    _30hz = 0x2,
    /// 60 Hz data rate.
    _60hz = 0x3,
    /// 120 Hz data rate.
    #[default]
    _120hz = 0x4,
    /// 240 Hz data rate.
    _240hz = 0x5,
    /// 480 Hz data rate.
    _480hz = 0x6,
}
