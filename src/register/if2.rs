use crate::Error;
use crate::Lsm6dsv320x;
use bitfield_struct::bitfield;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::{named_register, register};
use st_mems_bus::BusOperation;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum If2Reg {
    WhoAmI = 0x0F,
    StatusRegOis = 0x1E,
    OutTempL = 0x20,
    OutTempH = 0x21,
    OutxLGOis = 0x22,
    OutxHGOis = 0x23,
    OutyLGOis = 0x24,
    OutyHGOis = 0x25,
    OutzLGOis = 0x26,
    OutzHGOis = 0x27,
    OutxLAOis = 0x28,
    OutxHAOis = 0x29,
    OutyLAOis = 0x2A,
    OutyHAOis = 0x2B,
    OutzLAOis = 0x2C,
    OutzHAOis = 0x2D,
    HandshakeCtrl = 0x6E,
    IntOis = 0x6F,
    Ctrl1Ois = 0x70,
    Ctrl2Ois = 0x71,
    Ctrl3Ois = 0x72,
}

/// IF2_WHO_AM_I (0x0F)
///
/// Read-only device identification register for auxiliary interface.
/// Fixed value: 0x73.
#[register(address = If2Reg::WhoAmI, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2WhoAmI {
    /// Device ID (fixed 0x73).
    #[bits(8, default = 0x73)]
    pub id: u8,
}

/// IF2_STATUS_REG_OIS (0x1E)
///
/// Status register for OIS auxiliary interface (read-only).
/// Indicates data availability and gyro settling status.
#[register(address = If2Reg::StatusRegOis, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2StatusRegOis {
    /// Accelerometer data available flag (reset on reading high byte).
    #[bits(1)]
    pub xlda: u8,
    /// Gyroscope data available flag (reset on reading high byte).
    #[bits(1)]
    pub gda: u8,
    /// Gyroscope settling phase indicator (high during settling).
    #[bits(1)]
    pub gyro_settling: u8,
    /// Reserved bits (read-only).
    #[bits(5, access = RO)]
    not_used0: u8,
}

/// IF2_OUT_TEMP_L (0x20)
///
/// Lower byte of temperature sensor output (read-only).
/// Combined with IF2_OUT_TEMP_H to form 16-bit two's complement temperature data.
#[register(address = If2Reg::OutTempL, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2OutTempL {
    /// Temperature output low byte.
    #[bits(8)]
    pub temp: u8,
}

/// IF2_OUT_TEMP_H (0x21)
///
/// Higher byte of temperature sensor output (read-only).
/// Combined with IF2_OUT_TEMP_L to form 16-bit two's complement temperature data.
#[register(address = If2Reg::OutTempH, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2OutTempH {
    /// Temperature output high byte.
    #[bits(8)]
    pub temp: u8,
}

/// IF2_OUTX_L_G_OIS to IF2_OUTZ_H_G_OIS (0x22 - 0x27)
///
/// 16-bit two's complement angular rate output for OIS gyroscope axes (X, Y, Z).
/// Data according to gyroscope full-scale and 7.68 kHz ODR settings.
#[named_register(address = If2Reg::OutxLGOis, access_type = Lsm6dsv320x, generics = 2)]
pub struct If2OutXYZGOIS {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// IF2_OUTX_L_A_OIS to IF2_OUTZ_H_A_OIS (0x28 - 0x2D)
///
/// 16-bit two's complement linear acceleration for OIS accelerometer axes (X, Y, Z).
/// Data are according to the accelerometer full-scale and ODR (7.68 kHz) settings.
#[named_register(address = If2Reg::OutxLGOis, access_type = Lsm6dsv320x, generics = 2)]
pub struct If2OutXYZAOIS {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// IF2_HANDSHAKE_CTRL (0x6E)
///
/// Control register for handshake between auxiliary (IF2) and primary interfaces (R/W).
/// Manages shared register access arbitration.
#[register(address = If2Reg::HandshakeCtrl, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2HandshakeCtrl {
    /// IF2 interface shared register access acknowledge bit.
    #[bits(1)]
    pub if2_shared_ack: u8,
    /// IF2 interface shared register access request bit.
    #[bits(1)]
    pub if2_shared_req: u8,
    /// Reserved bits (read-only).
    #[bits(6, access = RO)]
    not_used0: u8,
}

/// IF2_INT_OIS (0x6F)
///
/// OIS interrupt configuration and self-test register for auxiliary interface.
/// Read-only when primary interface has full control.
#[register(address = If2Reg::IntOis, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2IntOis {
    /// Accelerometer OIS self-test bits (2 bits).
    #[bits(2)]
    pub st_xl_ois: u8,
    /// Gyroscope OIS self-test bits (2 bits).
    #[bits(2)]
    pub st_g_ois: u8,
    /// OIS clamp disable during self-test (1 = clamp disabled).
    #[bits(1)]
    pub st_ois_clampdis: u8,
    /// Reserved bit (read-only).
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Masks OIS data ready signals until filter settling ends.
    #[bits(1)]
    pub drdy_mask_ois: u8,
    /// Enables OIS data ready interrupt on INT2 pin.
    #[bits(1)]
    pub int2_drdy_ois: u8,
}

/// IF2_CTRL1_OIS (0x70)
///
/// OIS configuration register 1 for auxiliary interface.
/// Controls SPI mode, OIS accelerometer and gyroscope enable, and simulation mode.
#[register(address = If2Reg::Ctrl1Ois, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2Ctrl1Ois {
    /// Enables SPI 3-wire interface if set; 0 = 4-wire SPI.
    #[bits(1)]
    pub if2_spi_read_en: u8,
    /// Enables gyroscope OIS chain.
    #[bits(1)]
    pub ois_g_en: u8,
    /// Enables accelerometer OIS chain.
    #[bits(1)]
    pub ois_xl_en: u8,
    /// Reserved bits (read-only).
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Enables OIS simulation mode.
    #[bits(1)]
    pub sim_ois: u8,
    /// Reserved bits (read-only).
    #[bits(2, access = RO)]
    not_used1: u8,
}

/// IF2_CTRL2_OIS (0x71)
///
/// OIS configuration register 2 (R/W).
/// Controls gyroscope OIS full-scale and LPF1 bandwidth selection.
#[register(address = If2Reg::Ctrl2Ois, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2Ctrl2Ois {
    /// Gyroscope OIS full-scale selection (3 bits).
    /// Values: 000 reserved, 001 ±250 dps, 010 ±500 dps, 011 ±1000 dps, 100 ±2000 dps.
    #[bits(3)]
    pub fs_g_ois: u8,
    /// Gyroscope OIS digital LPF1 filter bandwidth selection (2 bits).
    #[bits(2)]
    pub lpf1_g_ois_bw: u8,
    /// Reserved bits (read-only).
    #[bits(3, access = RO)]
    not_used0: u8,
}

/// IF2_CTRL3_OIS (0x72)
///
/// OIS configuration register 3 (R/W).
/// Controls accelerometer OIS full-scale and LPF bandwidth selection.
#[register(address = If2Reg::Ctrl3Ois, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct If2Ctrl3Ois {
    /// Accelerometer OIS full-scale selection (2 bits).
    /// Values: 00 ±2 g (default), 01 ±4 g, 10 ±8 g, 11 ±16 g.
    #[bits(2)]
    pub fs_xl_ois: u8,
    /// Reserved bit (read-only).
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Accelerometer OIS channel bandwidth selection (3 bits).
    #[bits(3)]
    pub lpf_xl_ois_bw: u8,
    /// Reserved bits (read-only).
    #[bits(2, access = RO)]
    not_used1: u8,
}

#[derive(Default)]
pub struct FiltOisSettlingMask {
    pub ois_drdy: u8,
}

/// OIS gyroscope self-test selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum OisGySelfTest {
    /// Self-test disabled.
    Disable = 0x0,
    /// Positive sign self-test.
    Positive = 0x1,
    /// Negative sign self-test.
    Negative = 0x2,
    /// Clamp positive self-test.
    ClampPos = 0x5,
    /// Clamp negative self-test.
    ClampNeg = 0x6,
}
