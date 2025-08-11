use crate::Error;
use crate::Lsm6dsv320x;
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::{named_register, register};
use st_mems_bus::BusOperation;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Reg {
    FuncCfgAccess = 0x1,
    PinCtrl = 0x2,
    IfCfg = 0x3,
    OdrTrigCfg = 0x6,
    FifoCtrl1 = 0x7,
    FifoCtrl2 = 0x8,
    FifoCtrl3 = 0x9,
    FifoCtrl4 = 0x0A,
    CounterBdrReg1 = 0x0B,
    CounterBdrReg2 = 0x0C,
    Int1Ctrl = 0x0D,
    Int2Ctrl = 0x0E,
    WhoAmI = 0x0F,
    Ctrl1 = 0x10,
    Ctrl2 = 0x11,
    Ctrl3 = 0x12,
    Ctrl4 = 0x13,
    Ctrl5 = 0x14,
    Ctrl6 = 0x15,
    Ctrl7 = 0x16,
    Ctrl8 = 0x17,
    Ctrl9 = 0x18,
    Ctrl10 = 0x19,
    CtrlStatus = 0x1A,
    FifoStatus1 = 0x1B,
    FifoStatus2 = 0x1C,
    AllIntSrc = 0x1D,
    StatusReg = 0x1E,
    OutTempL = 0x20,
    OutTempH = 0x21,
    OutxLG = 0x22,
    OutxHG = 0x23,
    OutyLG = 0x24,
    OutyHG = 0x25,
    OutzLG = 0x26,
    OutzHG = 0x27,
    OutxLA = 0x28,
    OutxHA = 0x29,
    OutyLA = 0x2A,
    OutyHA = 0x2B,
    OutzLA = 0x2C,
    OutzHA = 0x2D,
    UiOutxLGOisEis = 0x2E,
    UiOutxHGOisEis = 0x2F,
    UiOutyLGOisEis = 0x30,
    UiOutyHGOisEis = 0x31,
    UiOutzLGOisEis = 0x32,
    UiOutzHGOisEis = 0x33,
    UiOutxLAOisHg = 0x34,
    UiOutxHAOisHg = 0x35,
    UiOutyLAOisHg = 0x36,
    UiOutyHAOisHg = 0x37,
    UiOutzLAOisHg = 0x38,
    UiOutzHAOisHg = 0x39,
    Timestamp0 = 0x40,
    Timestamp1 = 0x41,
    Timestamp2 = 0x42,
    Timestamp3 = 0x43,
    UiStatusRegOis = 0x44,
    WakeUpSrc = 0x45,
    TapSrc = 0x46,
    D6dSrc = 0x47,
    StatusControllerMainpage = 0x48,
    EmbFuncStatusMainpage = 0x49,
    FsmStatusMainpage = 0x4A,
    MlcStatusMainpage = 0x4B,
    HgWakeUpSrc = 0x4C,
    Ctrl2XlHg = 0x4D,
    Ctrl1XlHg = 0x4E,
    InternalFreq = 0x4F,
    FunctionsEnable = 0x50,
    HgFunctionsEnable = 0x52,
    HgWakeUpThs = 0x53,
    InactivityDur = 0x54,
    InactivityThs = 0x55,
    TapCfg0 = 0x56,
    TapCfg1 = 0x57,
    TapCfg2 = 0x58,
    TapThs6d = 0x59,
    TapDur = 0x5A,
    WakeUpThs = 0x5B,
    WakeUpDur = 0x5C,
    FreeFall = 0x5D,
    Md1Cfg = 0x5E,
    Md2Cfg = 0x5F,
    HaodrCfg = 0x62,
    EmbFuncCfg = 0x63,
    UiHandshakeCtrl = 0x64,
    UiIf2Shared0 = 0x65,
    UiIf2Shared1 = 0x66,
    UiIf2Shared2 = 0x67,
    UiIf2Shared3 = 0x68,
    UiIf2Shared4 = 0x69,
    UiIf2Shared5 = 0x6A,
    CtrlEis = 0x6B,
    XlHgXOfsUsr = 0x6C,
    XlHgYOfsUsr = 0x6D,
    XlHgZOfsUsr = 0x6E,
    UiIntOis = 0x6F,
    UiCtrl1Ois = 0x70,
    UiCtrl2Ois = 0x71,
    UiCtrl3Ois = 0x72,
    XOfsUsr = 0x73,
    YOfsUsr = 0x74,
    ZOfsUsr = 0x75,
    FifoDataOutTag = 0x78,
    FifoDataOutXL = 0x79,
    FifoDataOutXH = 0x7A,
    FifoDataOutYL = 0x7B,
    FifoDataOutYH = 0x7C,
    FifoDataOutZL = 0x7D,
    FifoDataOutZH = 0x7E,
}

/// FuncCfgAccess (0x01)
///
/// Enable embedded functions register (R/W)
#[register(address = Reg::FuncCfgAccess, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FuncCfgAccess {
    /// If 1 enables OIS chain full control from the primary interface. Default: 0.
    #[bits(1)]
    pub ois_ctrl_from_ui: u8,
    /// Resets auxiliary interface control registers from primary interface (not auto-cleared). Default: 0.
    #[bits(1)]
    pub if2_reset: u8,
    /// Software power-on reset. Default: 0.
    #[bits(1)]
    pub sw_por: u8,
    /// Enables FSM control of CTRL registers. Default: 0.
    #[bits(1)]
    pub fsm_wr_ctrl_en: u8,
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Enables sensor hub register access. Default: 0.
    #[bits(1)]
    pub shub_reg_access: u8,
    /// Enables embedded functions register access. Default: 0.
    #[bits(1)]
    pub emb_func_reg_access: u8,
}

/// PinCtrl (0x02)
///
/// SDO, OCS_aux, SDO_aux pins pull-up control (R/W). Not reset by software reset.
#[register(address = Reg::PinCtrl, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct PinCtrl {
    /// Drive strength for interrupt pads. Default: 11 (highest strength for VDDIO < 2 V).
    #[bits(2)]
    pub io_pad_strength: u8,
    #[bits(3, access = RO)]
    not_used0: u8,
    /// Selects action after "reset whole chip" I3C pattern; 1 = global reset (POR reset). Default: 1.
    #[bits(1)]
    pub ibhr_por_en: u8,
    /// Enables pull-up on SDO pin. Default: 0 (disabled).
    #[bits(1)]
    pub sdo_pu_en: u8,
    /// Disables pull-up on OCS_aux and SDO_aux pins. Default: 0 (pull-up enabled).
    #[bits(1)]
    pub ois_pu_dis: u8,
}

/// IfCfg (0x03)
///
/// Interface configuration register (R/W). Not reset by software reset.
#[register(address = Reg::IfCfg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IfCfg {
    /// Disables I²C and MIPI I3C interfaces. Default: 0 (enabled).
    #[bits(1)]
    pub i2c_i3c_disable: u8,
    #[bits(1, access = RO)]
    not_used0: u8,
    /// SPI serial interface mode selection; 0=4-wire, 1=3-wire. Default: 0.
    #[bits(1)]
    pub sim: u8,
    /// Push-pull/open-drain selection on INT1 and INT2 pins; 0=push-pull, 1=open-drain. Default: 0.
    #[bits(1)]
    pub pp_od: u8,
    /// Interrupt output pins active level; 0=active high, 1=active low. Default: 0.
    #[bits(1)]
    pub h_lactive: u8,
    /// Enables antispike filters on SCL and SDA lines. Default: 0 (managed by protocol).
    #[bits(1)]
    pub asf_ctrl: u8,
    /// Enables internal pull-up on auxiliary I²C line. Default: 0 (disabled).
    #[bits(1)]
    pub shub_pu_en: u8,
    /// Enables pull-up on SDA pin. Default: 0 (disabled).
    #[bits(1)]
    pub sda_pu_en: u8,
}

/// OdrTrigCfg (0x06)
///
/// ODR-triggered mode configuration register (R/W).
#[register(address = Reg::OdrTrigCfg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct OdrTrigCfg {
    /// Number of data generated in reference period when ODR-triggered mode is set.
    /// Allowed values: 0 (default) or 4 to 255.
    #[bits(8)]
    pub odr_trig_nodr: u8,
}

/// FifoCtrl1 (0x07)
///
/// FIFO watermark threshold (R/W). 1 LSB = TAG (1 byte) + 1 sensor (6 bytes).
#[register(address = Reg::FifoCtrl1, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoCtrl1 {
    /// FIFO watermark threshold; watermark flag set when FIFO bytes ≥ threshold.
    #[bits(8)]
    pub wtm: u8,
}

/// FifoCtrl2 (0x08)
///
/// FIFO control register 2 (R/W).
#[register(address = Reg::FifoCtrl2, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoCtrl2 {
    #[bits(1, access = RO)]
    not_used2: u8,
    /// Compression algorithm uncompressed data rate configuration.
    /// 0=default (no forced uncompressed data), 1=every 8 batch rate, 2=every 16, 3=every 32.
    #[bits(2)]
    pub uncompr_rate: u8,
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Enables ODR CHANGE virtual sensor batching in FIFO. Default: 0 (disabled).
    #[bits(1)]
    pub odr_chg_en: u8,
    #[bits(1, access = RO)]
    not_used1: u8,
    /// Enables FIFO compression algorithm runtime. Default: 0 (disabled).
    #[bits(1)]
    pub fifo_compr_rt_en: u8,
    /// Enables FIFO stop at watermark threshold. Default: 0 (disabled).
    #[bits(1)]
    pub stop_on_wtm: u8,
}

/// FifoCtrl3 (0x09)
///
/// FIFO batch data rate configuration for accelerometer and gyroscope (R/W).
#[register(address = Reg::FifoCtrl3, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoCtrl3 {
    /// Batch data rate for accelerometer (BDR_XL_[3:0]). 0000=not batched (default).
    #[bits(4)]
    pub bdr_xl: u8,
    /// Batch data rate for gyroscope (BDR_GY_[3:0]). 0000=not batched (default).
    #[bits(4)]
    pub bdr_gy: u8,
}

/// FifoCtrl4 (0x0A)
///
/// FIFO control register 4 (R/W).
#[register(address = Reg::FifoCtrl4, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoCtrl4 {
    /// FIFO mode selection (3 bits). Default: 000 (bypass mode).
    #[bits(3)]
    pub fifo_mode: u8,
    /// Enables FIFO batching of enhanced EIS gyroscope output values. Default: 0 (disabled).
    #[bits(1)]
    pub g_eis_fifo_en: u8,
    /// Batch data rate for temperature data (ODR_T_BATCH_[1:0]). Default: 00 (not batched).
    #[bits(2)]
    pub odr_t_batch: u8,
    /// Decimation for timestamp batching in FIFO (DEC_TS_BATCH_[1:0]). Default: 00 (no batching).
    #[bits(2)]
    pub dec_ts_batch: u8,
}

/// CounterBdrReg1 - CounterBdrReg2 (0x0B - 0x0C)
///
/// Counter batch data rate register 1 (R/W).
#[register(address = Reg::CounterBdrReg1, access_type = Lsm6dsv320x, generics = 2, order = Inverse)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct CounterBdr {
    /// Trigger for internal batch event counter (2 bits).
    #[bits(10)]
    pub cnt_bdr_th: u16,
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Enables batching accelerometer high-g data in FIFO. Default: 0 (disabled).
    #[bits(1)]
    pub xl_hg_batch_en: u8,
    #[bits(1, access = RO)]
    not_used2: u8,
    /// Trigger selection for internal batch event counter (2 bits).
    #[bits(2)]
    pub trig_counter_bdr: u8,
    #[bits(1, access = RO)]
    not_used1: u8,
}

/// CounterBdrReg1 (0x0B)
///
/// Counter batch data rate register 1 (R/W).
#[register(address = Reg::CounterBdrReg1, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct CounterBdrReg1 {
    /// Trigger for internal batch event counter (2 bits).
    #[bits(2)]
    pub cnt_bdr_th: u8,
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Enables batching accelerometer high-g data in FIFO. Default: 0 (disabled).
    #[bits(1)]
    pub xl_hg_batch_en: u8,
    #[bits(1, access = RO)]
    not_used2: u8,
    /// Trigger selection for internal batch event counter (2 bits).
    #[bits(2)]
    pub trig_counter_bdr: u8,
    #[bits(1, access = RO)]
    not_used1: u8,
}

/// CounterBdrReg2 (0x0C)
///
/// Counter batch data rate register 2 (R/W).
#[register(address = Reg::CounterBdrReg2, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct CounterBdrReg2 {
    /// Lower 8 bits of threshold for internal batch event counter.
    #[bits(8)]
    pub cnt_bdr_th: u8,
}

/// Int1Ctrl (0x0D)
///
/// INT1 pin control register (R/W). Enables signals routed to INT1 pin.
#[register(address = Reg::Int1Ctrl, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Int1Ctrl {
    /// Enables accelerometer data-ready interrupt on INT1 pin. Default: 0.
    #[bits(1)]
    pub int1_drdy_xl: u8,
    /// Enables gyroscope data-ready interrupt on INT1 pin. Default: 0.
    #[bits(1)]
    pub int1_drdy_g: u8,
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Enables FIFO threshold interrupt on INT1 pin. Default: 0.
    #[bits(1)]
    pub int1_fifo_th: u8,
    /// Enables FIFO overrun interrupt on INT1 pin. Default: 0.
    #[bits(1)]
    pub int1_fifo_ovr: u8,
    /// Enables FIFO full flag interrupt on INT1 pin. Default: 0.
    #[bits(1)]
    pub int1_fifo_full: u8,
    /// Enables COUNTER_BDR_IA interrupt on INT1 pin. Default: 0.
    #[bits(1)]
    pub int1_cnt_bdr: u8,
    #[bits(1, access = RO)]
    not_used1: u8,
}

/// Int2Ctrl (0x0E)
///
/// INT2 pin control register (R/W). Enables signals routed to INT2 pin.
#[register(address = Reg::Int2Ctrl, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Int2Ctrl {
    /// Enables accelerometer data-ready interrupt on INT2 pin. Default: 0.
    #[bits(1)]
    pub int2_drdy_xl: u8,
    /// Enables gyroscope data-ready interrupt on INT2 pin. Default: 0.
    #[bits(1)]
    pub int2_drdy_g: u8,
    /// Enables gyroscope EIS data-ready interrupt on INT2 pin. Default: 0.
    #[bits(1)]
    pub int2_drdy_g_eis: u8,
    /// Enables FIFO threshold interrupt on INT2 pin. Default: 0.
    #[bits(1)]
    pub int2_fifo_th: u8,
    /// Enables FIFO overrun interrupt on INT2 pin. Default: 0.
    #[bits(1)]
    pub int2_fifo_ovr: u8,
    /// Enables FIFO full flag interrupt on INT2 pin. Default: 0.
    #[bits(1)]
    pub int2_fifo_full: u8,
    /// Enables COUNTER_BDR_IA interrupt on INT2 pin. Default: 0.
    #[bits(1)]
    pub int2_cnt_bdr: u8,
    /// Enables embedded functions end of operations signal routing to INT2 pin. Default: 0.
    #[bits(1)]
    pub int2_emb_func_endop: u8,
}

/// WhoAmI (0x0F)
///
/// Read-only device identification register; fixed value 0x73.
#[register(address = Reg::WhoAmI, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WhoAmI {
    /// Device ID; fixed value 0x73.
    #[bits(8)]
    pub id: u8,
}
/// CTRL1 (0x10)
///
/// Accelerometer control register 1 (R/W)
#[register(address = Reg::Ctrl1, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl1 {
    /// Accelerometer output data rate selection (4 bits)
    #[bits(4)]
    pub odr_xl: u8,
    /// Accelerometer operating mode selection (3 bits)
    #[bits(3)]
    pub op_mode_xl: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
}

/// CTRL2 (0x11)
///
/// Gyroscope control register 2 (R/W)
#[register(address = Reg::Ctrl2, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl2 {
    /// Gyroscope output data rate selection (4 bits)
    #[bits(4)]
    pub odr_g: u8,
    /// Gyroscope operating mode selection (3 bits)
    #[bits(3)]
    pub op_mode_g: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
}

/// CTRL3 (0x12)
///
/// Control register 3 (R/W)
#[register(address = Reg::Ctrl3, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl3 {
    /// Software reset; resets all control registers to default, auto-cleared
    #[bits(1)]
    pub sw_reset: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Register address auto-increment during multi-byte access (I2C, SPI, I3C)
    #[bits(1)]
    pub if_inc: u8,
    /// Reserved bits, read-only
    #[bits(3, access = RO)]
    not_used1: u8,
    /// Block data update; output registers not updated until LSB and MSB read
    #[bits(1)]
    pub bdu: u8,
    /// Reboots memory content; auto-cleared
    #[bits(1)]
    pub boot: u8,
}

/// CTRL4 (0x13)
///
/// Control register 4 (R/W)
#[register(address = Reg::Ctrl4, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl4 {
    /// INT2 pin input trigger polarity for embedded functions; 0=active low, 1=active high
    #[bits(1)]
    pub int2_in_lh: u8,
    /// Enables pulsed data-ready mode; 0=latched, 1=pulsed (65 µs pulse)
    #[bits(1)]
    pub drdy_pulsed: u8,
    /// Enables temperature sensor data-ready interrupt on INT2 pin
    #[bits(1)]
    pub int2_drdy_temp: u8,
    /// Masks data-ready signals until filter settling ends; 0=disabled, 1=enabled
    #[bits(1)]
    pub drdy_mask: u8,
    /// Enables routing embedded functions interrupt signals to INT1 pin
    #[bits(1)]
    pub int2_on_int1: u8,
    /// Reserved bits, read-only
    #[bits(3, access = RO)]
    not_used0: u8,
}

/// CTRL5 (0x14)
///
/// Control register 5 (R/W)
#[register(address = Reg::Ctrl5, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl5 {
    /// Enables INT pin when I3C is enabled; 0=disabled, 1=enabled
    #[bits(1)]
    pub int_en_i3c: u8,
    /// Bus available time selection for IBI (in-band interrupt)
    #[bits(2)]
    pub bus_act_sel: u8,
    /// Reserved bits, read-only
    #[bits(4, access = RO)]
    not_used0: u8,
    /// Part of auxiliary device static address (bit 0)
    #[bits(1)]
    pub if2_ta0_pid: u8,
}

/// CTRL6 (0x15)
///
/// Control register 6 (R/W)
#[register(address = Reg::Ctrl6, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl6 {
    /// Gyroscope full-scale selection (3 bits)
    #[bits(3)]
    pub fs_g: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO, default = 1)]
    not_used1: u8,
    /// Gyroscope low-pass filter (LPF1) bandwidth selection (3 bits)
    #[bits(3)]
    pub lpf1_g_bw: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
}

/// CTRL7 (0x16)
///
/// Control register 7 (R/W)
#[register(address = Reg::Ctrl7, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl7 {
    /// Enables gyroscope digital LPF1 filter; bandwidth selected in CTRL6
    #[bits(1)]
    pub lpf1_g_en: u8,
    /// Reserved bits, read-only
    #[bits(5, access = RO)]
    not_used0: u8,
    /// Enables gyroscope high-g accelerometer data-ready interrupt on INT2 pin
    #[bits(1)]
    pub int2_drdy_xl_hg: u8,
    /// Enables high-g accelerometer data-ready interrupt on INT1 pin
    #[bits(1)]
    pub int1_drdy_xl_hg: u8,
}

/// CTRL8 (0x17)
///
/// Control register 8 (R/W)
#[register(address = Reg::Ctrl8, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl8 {
    /// Accelerometer full-scale selection (2 bits)
    #[bits(2)]
    pub fs_xl: u8,
    /// Reserved bits, read-only
    #[bits(3, access = RO)]
    not_used0: u8,
    /// Accelerometer LPF2 and HP filter configuration and cutoff setting (3 bits)
    #[bits(3)]
    pub hp_lpf2_xl_bw: u8,
}

/// CTRL9 (0x18)
///
/// Control register 9 (R/W)
#[register(address = Reg::Ctrl9, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl9 {
    /// Enables accelerometer user offset correction block on output registers
    #[bits(1)]
    pub usr_off_on_out: u8,
    /// Weight of accelerometer user offset bits; 0=2^-10 g/LSB, 1=2^-6 g/LSB
    #[bits(1)]
    pub usr_off_w: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Enables accelerometer high-resolution selection (LPF2 second filtering stage)
    #[bits(1)]
    pub lpf2_xl_en: u8,
    /// Accelerometer slope filter / high-pass filter selection; 0=low-pass, 1=high-pass
    #[bits(1)]
    pub hp_slope_xl_en: u8,
    /// Enables accelerometer LPF2 and HPF fast-settling mode during exit from power-down
    #[bits(1)]
    pub xl_fastsettl_mode: u8,
    /// Enables accelerometer high-pass filter reference mode
    #[bits(1)]
    pub hp_ref_mode_xl: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used1: u8,
}

/// CTRL10 (0x19)
///
/// Control register 10 (R/W)
#[register(address = Reg::Ctrl10, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl10 {
    /// Accelerometer self-test selection (2 bits)
    #[bits(2)]
    pub st_xl: u8,
    /// Gyroscope self-test selection (2 bits)
    #[bits(2)]
    pub st_g: u8,
    /// Reserved bits, read-only
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Enables debug mode for embedded functions
    #[bits(1)]
    pub emb_func_debug: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used1: u8,
}

/// CTRL_STATUS (0x1A)
///
/// Control status register (R)
#[register(address = Reg::CtrlStatus, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct CtrlStatus {
    /// Reserved bits, read-only
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Flag indicating current controller of device configuration registers; 0=standard interface writable, 1=FSM control
    #[bits(1)]
    pub fsm_wr_ctrl_status: u8,
    /// Reserved bits, read-only
    #[bits(5, access = RO)]
    not_used1: u8,
}

/// FIFO_STATUS1 - FIFO_STATUS2 (0x1B - 0x1C)
///
/// FIFO status register 1, 2 (R)
#[register(address = Reg::FifoStatus1, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct FifoStatusReg {
    /// Number of unread sensor data (TAG + 6 bytes) stored in FIFO [8:0]
    #[bits(9)]
    pub diff_fifo: u16,
    /// Reserved bits, read-only
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Latched FIFO overrun status; reset when register read
    #[bits(1)]
    pub fifo_ovr_latched: u8,
    /// Counter batch data rate interrupt active flag
    #[bits(1)]
    pub counter_bdr_ia: u8,
    /// FIFO full interrupt active flag
    #[bits(1)]
    pub fifo_full_ia: u8,
    /// FIFO overrun interrupt active flag
    #[bits(1)]
    pub fifo_ovr_ia: u8,
    /// FIFO watermark interrupt active flag
    #[bits(1)]
    pub fifo_wtm_ia: u8,
}

/// ALL_INT_SRC (0x1D)
///
/// Source register for all interrupts (R)
#[register(address = Reg::AllIntSrc, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct AllIntSrc {
    /// Free-fall interrupt active flag
    #[bits(1)]
    pub ff_ia: u8,
    /// Wake-up interrupt active flag
    #[bits(1)]
    pub wu_ia: u8,
    /// Tap interrupt active flag
    #[bits(1)]
    pub tap_ia: u8,
    /// High-g interrupt active flag
    #[bits(1)]
    pub hg_ia: u8,
    /// 6D orientation interrupt active flag
    #[bits(1)]
    pub d6d_ia: u8,
    /// Sleep change interrupt active flag
    #[bits(1)]
    pub sleep_change_ia: u8,
    /// Sensor hub interrupt active flag
    #[bits(1)]
    pub shub_ia: u8,
    /// Embedded functions interrupt active flag
    #[bits(1)]
    pub emb_func_ia: u8,
}

/// STATUS_REG (0x1E)
///
/// Status register (R)
#[register(address = Reg::StatusReg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct StatusReg {
    /// Accelerometer new data available flag
    #[bits(1)]
    pub xlda: u8,
    /// Gyroscope new data available flag
    #[bits(1)]
    pub gda: u8,
    /// Temperature new data available flag
    #[bits(1)]
    pub tda: u8,
    /// High-g accelerometer new data available flag
    #[bits(1)]
    pub xlhgda: u8,
    /// Enhanced EIS gyroscope new data available flag
    #[bits(1)]
    pub gda_eis: u8,
    /// OIS chain new data available flag
    #[bits(1)]
    pub ois_drdy: u8,
    /// Reserved bits, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Timestamp overflow alert flag
    #[bits(1)]
    pub timestamp_endcount: u8,
}
/// OutTemp (0x20, 0x21)
///
/// Temperature sensor output data (16-bit two's complement)
#[register(address = Reg::OutTempL, access_type = Lsm6dsv320x, generics = 2)]
pub struct OutTemp(pub i16);

/// OutXYZG (0x22 - 0x27)
///
/// Gyroscope X, Y, Z axis angular rate output (3 x 16-bit two's complement)
#[named_register(address = Reg::OutxLG, access_type = Lsm6dsv320x, generics = 2)]
pub struct OutXYZG {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// OutXYZA (0x28 - 0x2D)
///
/// Accelerometer X, Y, Z axis linear acceleration output (3 x 16-bit two's complement)
#[named_register(address = Reg::OutxLA, access_type = Lsm6dsv320x, generics = 2)]
pub struct OutXYZA {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// UiOutXYZGOisEis (0x2E - 0x33)
///
/// Gyroscope OIS/EIS output for X, Y, Z axes (3 x 16-bit two's complement)
#[named_register(address = Reg::UiOutxLGOisEis, access_type = Lsm6dsv320x, generics = 2)]
pub struct UiOutXYZGOisEis {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// UiOutXYZAOisHg (0x34 - 0x39)
///
/// Accelerometer OIS/high-g output for X, Y, Z axes (3 x 16-bit two's complement)
#[named_register(address = Reg::UiOutxLAOisHg, access_type = Lsm6dsv320x, generics = 2)]
pub struct UiOutXYZAOisHg {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// TIMESTAMP (0x40 - 0x43)
///
/// The value is expressed as a 32-bit word and the bit resolution is 21.7 µs(typical)
#[register(address = Reg::Timestamp0, access_type = Lsm6dsv320x, generics = 2)]
pub struct Timestamp(pub u32);

/// UI_STATUS_REG_OIS (0x44)
///
/// OIS status register (R)
#[register(address = Reg::UiStatusRegOis, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiStatusRegOis {
    /// Accelerometer OIS data available flag
    #[bits(1)]
    pub xlda_ois: u8,
    /// Gyroscope OIS data available flag
    #[bits(1)]
    pub gda_ois: u8,
    /// Gyroscope output settling phase flag
    #[bits(1)]
    pub gyro_settling: u8,
    /// Reserved bits, read-only
    #[bits(5, access = RO)]
    not_used0: u8,
}

/// WAKE_UP_SRC (0x45)
///
/// Wake-up interrupt source register (R)
#[register(address = Reg::WakeUpSrc, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpSrc {
    /// Wake-up event detection status on Z-axis
    #[bits(1)]
    pub z_wu: u8,
    /// Wake-up event detection status on Y-axis
    #[bits(1)]
    pub y_wu: u8,
    /// Wake-up event detection status on X-axis
    #[bits(1)]
    pub x_wu: u8,
    /// Wake-up event detection status
    #[bits(1)]
    pub wu_ia: u8,
    /// Sleep status bit; 0=activity, 1=inactivity
    #[bits(1)]
    pub sleep_state: u8,
    /// Free-fall event detection status
    #[bits(1)]
    pub ff_ia: u8,
    /// Detects change event in activity/inactivity status
    #[bits(1)]
    pub sleep_change_ia: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
}

/// TAP_SRC (0x46)
///
/// Tap source register (R)
#[register(address = Reg::TapSrc, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapSrc {
    /// Tap event detection status on Z-axis
    #[bits(1)]
    pub z_tap: u8,
    /// Tap event detection status on Y-axis
    #[bits(1)]
    pub y_tap: u8,
    /// Tap event detection status on X-axis
    #[bits(1)]
    pub x_tap: u8,
    /// Sign of acceleration detected by tap event; 0=positive, 1=negative
    #[bits(1)]
    pub tap_sign: u8,
    /// Double-tap event detection status
    #[bits(1)]
    pub double_tap: u8,
    /// Single-tap event detection status
    #[bits(1)]
    pub single_tap: u8,
    /// Tap event detection status
    #[bits(1)]
    pub tap_ia: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
}

/// D6D_SRC (0x47)
///
/// Portrait, landscape, face-up and face-down source register (R)
#[register(address = Reg::D6dSrc, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct D6dSrc {
    /// X-axis low event (under threshold)
    #[bits(1)]
    pub xl: u8,
    /// X-axis high event (over threshold)
    #[bits(1)]
    pub xh: u8,
    /// Y-axis low event (under threshold)
    #[bits(1)]
    pub yl: u8,
    /// Y-axis high event (over threshold)
    #[bits(1)]
    pub yh: u8,
    /// Z-axis low event (under threshold)
    #[bits(1)]
    pub zl: u8,
    /// Z-axis high event (over threshold)
    #[bits(1)]
    pub zh: u8,
    /// Interrupt active for change position portrait, landscape, face-up, face-down
    #[bits(1)]
    pub d6d_ia: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
}

/// STATUS_CONTROLLER_MAINPAGE (0x48)
///
/// Sensor hub source register (R)
#[register(address = Reg::StatusControllerMainpage, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct StatusControllerMainpage {
    /// Sensor hub communication status; 0=not concluded, 1=concluded
    #[bits(1)]
    pub sens_hub_endop: u8,
    /// Reserved bits, read-only
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Not acknowledge on target 0 communication
    #[bits(1)]
    pub target0_nack: u8,
    /// Not acknowledge on target 1 communication
    #[bits(1)]
    pub target1_nack: u8,
    /// Not acknowledge on target 2 communication
    #[bits(1)]
    pub target2_nack: u8,
    /// Not acknowledge on target 3 communication
    #[bits(1)]
    pub target3_nack: u8,
    /// Write operation on target 0 completed
    #[bits(1)]
    pub wr_once_done: u8,
}

/// EMB_FUNC_STATUS_MAINPAGE (0x49)
///
/// Embedded function status register (R)
#[register(address = Reg::EmbFuncStatusMainpage, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncStatusMainpage {
    /// Reserved bits, read-only
    #[bits(3, access = RO)]
    not_used0: u8,
    /// Interrupt status bit for step detection (1: interrupt detected)
    #[bits(1)]
    pub is_step_det: u8,
    /// Interrupt status bit for tilt detection (1: interrupt detected)
    #[bits(1)]
    pub is_tilt: u8,
    /// Interrupt status bit for significant motion detection (1: interrupt detected)
    #[bits(1)]
    pub is_sigmot: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used1: u8,
    /// Interrupt status bit for FSM long counter timeout interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm_lc: u8,
}

/// FSM_STATUS_MAINPAGE (0x4A)
///
/// Finite state machine status register (R)
#[register(address = Reg::FsmStatusMainpage, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FsmStatusMainpage {
    /// Interrupt status bit for FSM1 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm1: u8,
    /// Interrupt status bit for FSM2 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm2: u8,
    /// Interrupt status bit for FSM3 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm3: u8,
    /// Interrupt status bit for FSM4 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm4: u8,
    /// Interrupt status bit for FSM5 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm5: u8,
    /// Interrupt status bit for FSM6 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm6: u8,
    /// Interrupt status bit for FSM7 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm7: u8,
    /// Interrupt status bit for FSM8 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_fsm8: u8,
}

/// MLC_STATUS_MAINPAGE (0x4B)
///
/// Machine learning core status register (R)
#[register(address = Reg::MlcStatusMainpage, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct MlcStatusMainpage {
    /// Interrupt status bit for MLC1 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_mlc1: u8,
    /// Interrupt status bit for MLC2 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_mlc2: u8,
    /// Interrupt status bit for MLC3 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_mlc3: u8,
    /// Interrupt status bit for MLC4 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_mlc4: u8,
    /// Interrupt status bit for MLC5 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_mlc5: u8,
    /// Interrupt status bit for MLC6 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_mlc6: u8,
    /// Interrupt status bit for MLC7 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_mlc7: u8,
    /// Interrupt status bit for MLC8 interrupt event (1: interrupt detected)
    #[bits(1)]
    pub is_mlc8: u8,
}

/// HG_WAKE_UP_SRC (0x4C)
///
/// High-g wake-up source register (R)
#[register(address = Reg::HgWakeUpSrc, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct HgWakeUpSrc {
    /// High-g wake-up event detection status on Z-axis
    #[bits(1)]
    pub hg_z_wu: u8,
    /// High-g wake-up event detection status on Y-axis
    #[bits(1)]
    pub hg_y_wu: u8,
    /// High-g wake-up event detection status on X-axis
    #[bits(1)]
    pub hg_x_wu: u8,
    /// High-g wake-up event detection status
    #[bits(1)]
    pub hg_wu_ia: u8,
    /// High-g wake-up change event detection status
    #[bits(1)]
    pub hg_wu_change_ia: u8,
    /// High-g shock function state
    #[bits(1)]
    pub hg_shock_state: u8,
    /// High-g shock state change event detection status
    #[bits(1)]
    pub hg_shock_change_ia: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
}

/// CTRL2_XL_HG (0x4D)
///
/// Control register 2 high-g accelerometer (R/W)
#[register(address = Reg::Ctrl2XlHg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl2XlHg {
    /// High-g accelerometer self-test selection (2 bits)
    #[bits(2)]
    pub xl_hg_st: u8,
    /// Reserved bits, read-only
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Drives data with user offset correction to high-g wake-up and shock
    #[bits(1)]
    pub hg_usr_off_on_wu: u8,
    /// Reserved bits, read-only
    #[bits(3, access = RO)]
    not_used1: u8,
}

/// CTRL1_XL_HG (0x4E)
///
/// Control register 1 high-g accelerometer (R/W)
#[register(address = Reg::Ctrl1XlHg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Ctrl1XlHg {
    /// High-g accelerometer full-scale selection (3 bits)
    #[bits(3)]
    pub fs_xl_hg: u8,
    /// High-g accelerometer output data rate selection (3 bits)
    #[bits(3)]
    pub odr_xl_hg: u8,
    /// Enables high-g accelerometer user offset functionality on output registers
    #[bits(1)]
    pub hg_usr_off_on_out: u8,
    /// Enables read of high-g accelerometer channel from output registers
    #[bits(1)]
    pub xl_hg_regout_en: u8,
}

/// INTERNAL_FREQ_FINE (0x4F)
///
/// Internal frequency register (R)
#[register(address = Reg::InternalFreq, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct InternalFreq {
    /// Difference in percentage of effective ODR and timestamp rate w.r.t typical (8-bit two's complement)
    #[bits(8)]
    pub freq_fine: u8,
}

/// FUNCTIONS_ENABLE (0x50)
///
/// Enable interrupt functions register (R/W)
#[register(address = Reg::FunctionsEnable, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FunctionsEnable {
    /// Enables activity/inactivity (sleep) function (2 bits)
    #[bits(2)]
    pub inact_en: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
    /// When set, reading ALL_INT_SRC does not reset latched interrupt signals
    #[bits(1)]
    pub dis_rst_lir_all_int: u8,
    /// Reserved bits, read-only
    #[bits(2, access = RO)]
    not_used1: u8,
    /// Enables timestamp counter
    #[bits(1)]
    pub timestamp_en: u8,
    /// Enables basic interrupts (6D/4D, free-fall, wake-up, tap, activity/inactivity)
    #[bits(1)]
    pub interrupts_enable: u8,
}

/// HG_FUNCTIONS_ENABLE (0x52)
///
/// Enable high-g functions register (R/W)
#[register(address = Reg::HgFunctionsEnable, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct HgFunctionsEnable {
    /// High-g duration to exit from shock state (4 bits)
    #[bits(4)]
    pub hg_shock_dur: u8,
    /// Routes high-g wake-up event to INT1
    #[bits(1)]
    pub int1_hg_wu: u8,
    /// Routes high-g wake-up event to INT2
    #[bits(1)]
    pub int2_hg_wu: u8,
    /// Selects type of wake-up interrupt to be sent to INT pins
    #[bits(1)]
    pub hg_wu_change_int_sel: u8,
    /// Enables high-g interrupt generator (high-g wake-up and shock)
    #[bits(1)]
    pub hg_interrupts_enable: u8,
}

/// HG_WAKE_UP_THS (0x53)
///
/// High-g wake-up threshold register (R/W)
#[register(address = Reg::HgWakeUpThs, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct HgWakeUpThs {
    /// High-g wake-up threshold; resolution 1 g/LSB if FS_XL_HG ≤ ±256 g, else 1.25 g/LSB
    #[bits(8)]
    pub hg_wk_ths: u8,
}

/// INACTIVITY_DUR (0x54)
///
/// Activity/inactivity configuration register (R/W)
#[register(address = Reg::InactivityDur, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct InactivityDur {
    /// Duration in transition from inactivity to activity (2 bits)
    #[bits(2)]
    pub inact_dur: u8,
    /// Selects ODR_XL target during inactivity (2 bits)
    #[bits(2)]
    pub xl_inact_odr: u8,
    /// Weight of 1 LSB of wake-up and activity/inactivity threshold (3 bits)
    #[bits(3)]
    pub wu_inact_ths_w: u8,
    /// Activity/inactivity interrupt mode configuration
    #[bits(1)]
    pub sleep_status_on_int: u8,
}

/// INACTIVITY_THS (0x55)
///
/// Activity/inactivity threshold setting register (R/W)
#[register(address = Reg::InactivityThs, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct InactivityThs {
    /// Activity/inactivity threshold (6 bits)
    #[bits(6)]
    pub inact_ths: u8,
    /// Routes high-g shock event to INT1
    #[bits(1)]
    pub int1_hg_shock_change: u8,
    /// Routes high-g shock event to INT2
    #[bits(1)]
    pub int2_hg_shock_change: u8,
}

/// TAP_CFG0 (0x56)
///
/// Tap configuration register 0 (R/W)
#[register(address = Reg::TapCfg0, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg0 {
    /// Latched interrupt; 0: interrupt request not latched, 1: latched
    #[bits(1)]
    pub lir: u8,
    /// Enables Z direction in tap recognition; 0: disabled, 1: enabled
    #[bits(1)]
    pub tap_z_en: u8,
    /// Enables Y direction in tap recognition; 0: disabled, 1: enabled
    #[bits(1)]
    pub tap_y_en: u8,
    /// Enables X direction in tap recognition; 0: disabled, 1: enabled
    #[bits(1)]
    pub tap_x_en: u8,
    /// HPF or slope filter selection on wake-up and activity/inactivity functions; 0: slope filter, 1: HPF
    #[bits(1)]
    pub slope_fds: u8,
    /// Masks execution trigger of basic interrupt functions when accelerometer data are settling; 0: disabled, 1: enabled
    #[bits(1)]
    pub hw_func_mask_xl_settl: u8,
    /// LPF2 filter on 6D function selection; 0: ODR/2 low-pass filtered data, 1: LPF2 output data
    #[bits(1)]
    pub low_pass_on_6d: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used1: u8,
}

/// TAP_CFG1 (0x57)
///
/// Tap configuration register 1 (R/W)
#[register(address = Reg::TapCfg1, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg1 {
    /// X-axis tap recognition threshold (5 bits); 1 LSB = FS_XL / 2^5
    #[bits(5)]
    pub tap_ths_x: u8,
    /// Selection of axis priority for tap detection (3 bits)
    #[bits(3)]
    pub tap_priority: u8,
}

/// TAP_CFG2 (0x58)
///
/// Tap configuration register 2 (R/W)
#[register(address = Reg::TapCfg2, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapCfg2 {
    /// Y-axis tap recognition threshold (5 bits); 1 LSB = FS_XL / 2^5
    #[bits(5)]
    pub tap_ths_y: u8,
    /// Reserved bits, read-only
    #[bits(3, access = RO)]
    not_used0: u8,
}

/// TAP_THS_6D (0x59)
///
/// Portrait/landscape position and tap function threshold register (R/W)
#[register(address = Reg::TapThs6d, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapThs6d {
    /// Z-axis tap recognition threshold (5 bits); 1 LSB = FS_XL / 2^5
    #[bits(5)]
    pub tap_ths_z: u8,
    /// Threshold for 4D/6D function (2 bits)
    #[bits(2)]
    pub sixd_ths: u8,
    /// Enables 4D orientation detection; 0: disabled, 1: enabled
    #[bits(1)]
    pub d4d_en: u8,
}

/// TAP_DUR (0x5A)
///
/// Tap recognition function setting register (R/W)
#[register(address = Reg::TapDur, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct TapDur {
    /// Maximum duration of overthreshold event (2 bits); 1 LSB corresponds to 4/ODR_XL time
    #[bits(2)]
    pub shock: u8,
    /// Expected quiet time after a tap detection (2 bits); 1 LSB corresponds to 2/ODR_XL time
    #[bits(2)]
    pub quiet: u8,
    /// Duration of maximum time gap for double-tap recognition (4 bits); 1 LSB corresponds to 16/ODR_XL time
    #[bits(4)]
    pub dur: u8,
}

/// WAKE_UP_THS (0x5B)
///
/// Single/double-tap selection and wake-up configuration (R/W)
#[register(address = Reg::WakeUpThs, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpThs {
    /// Wake-up threshold (6 bits); resolution depends on WU_INACT_THS_W_[2:0] in INACTIVITY_DUR
    #[bits(6)]
    pub wk_ths: u8,
    /// Drives low-pass filtered data with user offset correction to wake-up and activity/inactivity functions
    #[bits(1)]
    pub usr_off_on_wu: u8,
    /// Enables single/double-tap event; 0: single-tap only, 1: both single and double-tap
    #[bits(1)]
    pub single_double_tap: u8,
}

/// WAKE_UP_DUR (0x5C)
///
/// Free-fall, wake-up, and sleep mode functions duration setting register (R/W)
#[register(address = Reg::WakeUpDur, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WakeUpDur {
    /// Duration to go in sleep mode (4 bits); 1 LSB = 2 + 512/ODR_XL time
    #[bits(4)]
    pub sleep_dur: u8,
    /// Reserved bit, read-only
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Wake-up duration event (2 bits); 1 LSB = 1/ODR_XL time
    #[bits(2)]
    pub wake_dur: u8,
    /// Free-fall duration event (1 bit); 1 LSB = 1/ODR_XL time
    #[bits(1)]
    pub ff_dur: u8,
}

/// FREE_FALL (0x5D)
///
/// Free-fall function duration setting register (R/W)
#[register(address = Reg::FreeFall, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FreeFall {
    /// Free-fall threshold setting (3 bits); see datasheet Table 182 for values
    #[bits(3)]
    pub ff_ths: u8,
    /// Free-fall duration event (5 bits); 1 LSB = 1/ODR_XL time
    #[bits(5)]
    pub ff_dur: u8,
}

/// MD1_CFG (0x5E)
///
/// Functions routing to INT1 pin register (R/W)
#[register(address = Reg::Md1Cfg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Md1Cfg {
    /// Routing activity/inactivity recognition event to INT1
    #[bits(1)]
    pub int1_sleep_change: u8,
    /// Routing single-tap recognition event to INT1
    #[bits(1)]
    pub int1_single_tap: u8,
    /// Routing wake-up event to INT1
    #[bits(1)]
    pub int1_wu: u8,
    /// Routing free-fall event to INT1
    #[bits(1)]
    pub int1_ff: u8,
    /// Routing double-tap event to INT1
    #[bits(1)]
    pub int1_double_tap: u8,
    /// Routing 6D event to INT1
    #[bits(1)]
    pub int1_6d: u8,
    /// Routing embedded functions event to INT1
    #[bits(1)]
    pub int1_emb_func: u8,
    /// Routing sensor hub communication concluded event to INT1
    #[bits(1)]
    pub int1_shub: u8,
}

/// MD2_CFG (0x5F)
///
/// Functions routing to INT2 pin register (R/W)
#[register(address = Reg::Md2Cfg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Md2Cfg {
    /// Routing activity/inactivity recognition event to INT2
    #[bits(1)]
    pub int2_sleep_change: u8,
    /// Routing single-tap recognition event to INT2
    #[bits(1)]
    pub int2_single_tap: u8,
    /// Routing wake-up event to INT2
    #[bits(1)]
    pub int2_wu: u8,
    /// Routing free-fall event to INT2
    #[bits(1)]
    pub int2_ff: u8,
    /// Routing double-tap event to INT2
    #[bits(1)]
    pub int2_double_tap: u8,
    /// Routing 6D event to INT2
    #[bits(1)]
    pub int2_6d: u8,
    /// Routing embedded functions event to INT2
    #[bits(1)]
    pub int2_emb_func: u8,
    /// Routing timestamp overflow alert to INT2
    #[bits(1)]
    pub int2_timestamp: u8,
}

/// HAODR_CFG (0x62)
///
/// HAODR data rate configuration register (R/W)
#[register(address = Reg::HaodrCfg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct HaodrCfg {
    /// Selects the ODR set supported when high-accuracy ODR (HAODR) mode is enabled (2 bits)
    #[bits(2)]
    pub haodr_sel: u8,
    /// Reserved bits, read-only
    #[bits(6, access = RO)]
    not_used0: u8,
}

/// EMB_FUNC_CFG (0x63)
///
/// Embedded functions configuration register (R/W)
#[register(address = Reg::EmbFuncCfg, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct EmbFuncCfg {
    /// Reserved bits, read-only
    #[bits(3, access = RO)]
    not_used0: u8,
    /// Disables execution of the embedded functions; 0: enabled, 1: disabled
    #[bits(1)]
    pub emb_func_disable: u8,
    /// Masks execution trigger of embedded functions when low-g accelerometer data are settling
    #[bits(1)]
    pub emb_func_irq_mask_xl_settl: u8,
    /// Masks execution trigger of embedded functions when gyroscope data are settling
    #[bits(1)]
    pub emb_func_irq_mask_g_settl: u8,
    /// Masks execution trigger of embedded functions when high-g accelerometer data are settling
    #[bits(1)]
    pub emb_func_irq_mask_xl_hg_settl: u8,
    /// Drives high-g accelerometer data with user offset correction to embedded functions
    #[bits(1)]
    pub hg_usr_off_on_emb_func: u8,
}

/// UI_HANDSHAKE_CTRL (0x64)
///
/// Control register (UI side) for UI / IF2 shared registers (R/W)
#[register(address = Reg::UiHandshakeCtrl, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiHandshakeCtrl {
    /// Primary interface side request bit for access to UI_IF2_SHARED registers
    #[bits(1)]
    pub ui_shared_req: u8,
    /// Primary interface side acknowledge bit for access to UI_IF2_SHARED registers
    #[bits(1)]
    pub ui_shared_ack: u8,
    /// Reserved bits, read-only
    #[bits(6, access = RO)]
    not_used0: u8,
}

/// UI_IF2_SHARED_0 (0x65)
///
/// UI / IF2 shared register 0 (R/W).
/// Volatile byte used as a contact point between primary and secondary interface hosts.
/// Access is mutually exclusive and managed via UI_SHARED_ACK/REQ and IF2_SHARED_ACK/REQ bits.
#[register(address = Reg::UiIf2Shared0, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiIf2Shared0 {
    /// Shared 8-bit data between UI and IF2 interfaces.
    #[bits(8)]
    pub ui_if2_shared: u8,
}

/// UI_IF2_SHARED_1 (0x66)
///
/// UI / IF2 shared register 1 (R/W).
/// Volatile byte used as a contact point between primary and secondary interface hosts.
/// Access is mutually exclusive and managed via UI_SHARED_ACK/REQ and IF2_SHARED_ACK/REQ bits.
#[register(address = Reg::UiIf2Shared1, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiIf2Shared1 {
    /// Shared 8-bit data between UI and IF2 interfaces.
    #[bits(8)]
    pub ui_if2_shared: u8,
}

/// UI_IF2_SHARED_2 (0x67)
///
/// UI / IF2 shared register 2 (R/W).
/// Volatile byte used as a contact point between primary and secondary interface hosts.
/// Access is mutually exclusive and managed via UI_SHARED_ACK/REQ and IF2_SHARED_ACK/REQ bits.
#[register(address = Reg::UiIf2Shared2, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiIf2Shared2 {
    /// Shared 8-bit data between UI and IF2 interfaces.
    #[bits(8)]
    pub ui_if2_shared: u8,
}

/// UI_IF2_SHARED_3 (0x68)
///
/// UI / IF2 shared register 3 (R/W).
/// Volatile byte used as a contact point between primary and secondary interface hosts.
/// Access is mutually exclusive and managed via UI_SHARED_ACK/REQ and IF2_SHARED_ACK/REQ bits.
#[register(address = Reg::UiIf2Shared3, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiIf2Shared3 {
    /// Shared 8-bit data between UI and IF2 interfaces.
    #[bits(8)]
    pub ui_if2_shared: u8,
}

/// UI_IF2_SHARED_4 (0x69)
///
/// UI / IF2 shared register 4 (R/W).
/// Volatile byte used as a contact point between primary and secondary interface hosts.
/// Access is mutually exclusive and managed via UI_SHARED_ACK/REQ and IF2_SHARED_ACK/REQ bits.
#[register(address = Reg::UiIf2Shared4, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiIf2Shared4 {
    /// Shared 8-bit data between UI and IF2 interfaces.
    #[bits(8)]
    pub ui_if2_shared: u8,
}

/// UI_IF2_SHARED_5 (0x6A)
///
/// UI / IF2 shared register 5 (R/W).
/// Volatile byte used as a contact point between primary and secondary interface hosts.
/// Access is mutually exclusive and managed via UI_SHARED_ACK/REQ and IF2_SHARED_ACK/REQ bits.
#[register(address = Reg::UiIf2Shared5, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiIf2Shared5 {
    /// Shared 8-bit data between UI and IF2 interfaces.
    #[bits(8)]
    pub ui_if2_shared: u8,
}

/// CTRL_EIS (0x6B)
///
/// Gyroscope EIS channel control register (R/W).
/// Controls full scale, LPF bandwidth, output routing, and ODR for EIS gyroscope channel.
#[register(address = Reg::CtrlEis, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct CtrlEis {
    /// Gyroscope full-scale selection for EIS channel (3 bits).
    /// Values 001 to 100 correspond to ±250 to ±2000 dps; 000 is reserved.
    #[bits(3)]
    pub fs_g_eis: u8,
    /// Enables routing gyroscope EIS output to OIS UI output registers (0=disabled, 1=enabled).
    #[bits(1)]
    pub g_eis_on_g_ois_out_reg: u8,
    /// Gyroscope digital LPF_EIS filter bandwidth selection (1 bit).
    #[bits(1)]
    pub lpf_g_eis_bw: u8,
    /// Reserved, read-only bit.
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Gyroscope EIS output data rate selection (2 bits).
    /// 00: off; 01: 1.92 kHz; 10: 960 Hz; 11: reserved.
    #[bits(2)]
    pub odr_g_eis: u8,
}

/// XL_HG_X_OFS_USR - XL_HG_Y_OFS_USR - XL_HG_Z_OFS_USR (0x6C - 0x6E)
///
/// High-g accelerometer (X, Y, Z)-axis user offset correction (R/W).
/// Offset added internally to measured acceleration on (X, Y, Z)-axis.
/// Expressed as 8-bit two's complement; weight 0.25 g/LSb for FS ≤ ±256g, 0.33 g/LSb for ±320g.
#[named_register(address = Reg::XlHgXOfsUsr, access_type = Lsm6dsv320x, generics = 2)]
#[derive(Default)]
pub struct XlHgXYZOfsUsr {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

/*
/// XL_HG_Y_OFS_USR (0x6D)
///
/// High-g accelerometer Y-axis user offset correction (R/W).
/// Offset added internally to measured acceleration on Y-axis.
/// Expressed as 8-bit two's complement; weight 0.25 g/LSb for FS ≤ ±256g, 0.33 g/LSb for ±320g.
#[register(address = Reg::XlHgYOfsUsr, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct XlHgYOfsUsr {
    /// 8-bit user offset for high-g accelerometer Y-axis.
    #[bits(8)]
    pub xl_hg_y_ofs_usr: u8,
}

/// XL_HG_Z_OFS_USR (0x6E)
///
/// High-g accelerometer Z-axis user offset correction (R/W).
/// Offset added internally to measured acceleration on Z-axis.
/// Expressed as 8-bit two's complement; weight 0.25 g/LSb for FS ≤ ±256g, 0.33 g/LSb for ±320g.
#[register(address = Reg::XlHgZOfsUsr, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct XlHgZOfsUsr {
    /// 8-bit user offset for high-g accelerometer Z-axis.
    #[bits(8)]
    pub xl_hg_z_ofs_usr: u8,
}*/

/// UI_INT_OIS (0x6F)
///
/// OIS interrupt configuration register (R/W).
/// Controls OIS DRDY masking, clamp disable during self-test, and INT2 DRDY routing.
#[register(address = Reg::UiIntOis, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiIntOis {
    /// Reserved, read-only (4 bits).
    #[bits(4, access = RO)]
    not_used0: u8,
    /// Disables OIS chain clamp during self-test (0=outputs=8000h, 1=OIS self-test outputs).
    #[bits(1)]
    pub st_ois_clampdis: u8,
    /// Reserved, read-only (1 bit).
    #[bits(1, access = RO)]
    not_used1: u8,
    /// Masks OIS data available signals until filter settling ends (0=enabled, 1=masked).
    #[bits(1)]
    pub drdy_mask_ois: u8,
    /// Enables OIS chain DRDY on INT2 pin from UI interface (priority over other INT2 settings).
    #[bits(1)]
    pub int2_drdy_ois: u8,
}

/// UI_CTRL1_OIS (0x70)
///
/// OIS configuration register 1 (R/W).
/// Controls auxiliary SPI mode, OIS accelerometer and gyroscope enable, and simulation mode.
#[register(address = Reg::UiCtrl1Ois, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiCtrl1Ois {
    /// Enables SPI 3-wire interface if set; 0 = 4-wire SPI.
    #[bits(1)]
    pub if2_spi_read_en: u8,
    /// Enables gyroscope OIS chain.
    #[bits(1)]
    pub ois_g_en: u8,
    /// Enables accelerometer OIS chain.
    #[bits(1)]
    pub ois_xl_en: u8,
    /// Reserved, read-only (2 bits).
    #[bits(2, access = RO)]
    not_used0: u8,
    /// Enables OIS simulation mode.
    #[bits(1)]
    pub sim_ois: u8,
    /// Reserved, read-only (2 bits).
    #[bits(2, access = RO)]
    not_used1: u8,
}

/// UI_CTRL2_OIS (0x71)
///
/// OIS configuration register 2 (R/W).
/// Controls gyroscope OIS full scale and LPF1 bandwidth selection.
#[register(address = Reg::UiCtrl2Ois, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiCtrl2Ois {
    /// Gyroscope OIS full-scale selection (3 bits).
    /// Values 001 to 100 correspond to ±250 to ±2000 dps; 000 reserved.
    #[bits(3)]
    pub fs_g_ois: u8,
    /// Gyroscope OIS digital LPF1 filter bandwidth selection (2 bits).
    #[bits(2)]
    pub lpf1_g_ois_bw: u8,
    /// Reserved, read-only (3 bits).
    #[bits(3, access = RO)]
    not_used0: u8,
}

/// UI_CTRL3_OIS (0x72)
///
/// OIS configuration register 3 (R/W).
/// Controls accelerometer OIS full scale and LPF bandwidth selection.
#[register(address = Reg::UiCtrl3Ois, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct UiCtrl3Ois {
    /// Accelerometer OIS full-scale selection (2 bits).
    /// 00=±2g (default), 01=±4g, 10=±8g, 11=±16g.
    #[bits(2)]
    pub fs_xl_ois: u8,
    /// Reserved, read-only (1 bit).
    #[bits(1, access = RO)]
    not_used0: u8,
    /// Accelerometer OIS channel bandwidth selection (3 bits).
    #[bits(3)]
    pub lpf_xl_ois_bw: u8,
    /// Reserved, read-only (2 bits).
    #[bits(2, access = RO)]
    not_used1: u8,
}

/// X_OFS_USR (0x73)
///
/// Accelerometer X-axis user offset correction (R/W).
/// Offset added internally to measured acceleration on X-axis.
/// Expressed as 8-bit two's complement; weight depends on USR_OFF_W in CTRL9.
#[register(address = Reg::XOfsUsr, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct XOfsUsr {
    /// 8-bit user offset for accelerometer X-axis.
    #[bits(8)]
    pub x_ofs_usr: u8,
}

/// Y_OFS_USR (0x74)
///
/// Accelerometer Y-axis user offset correction (R/W).
/// Offset added internally to measured acceleration on Y-axis.
/// Expressed as 8-bit two's complement; weight depends on USR_OFF_W in CTRL9.
#[register(address = Reg::YOfsUsr, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct YOfsUsr {
    /// 8-bit user offset for accelerometer Y-axis.
    #[bits(8)]
    pub y_ofs_usr: u8,
}

/// Z_OFS_USR (0x75)
///
/// Accelerometer Z-axis user offset correction (R/W).
/// Offset added internally to measured acceleration on Z-axis.
/// Expressed as 8-bit two's complement; weight depends on USR_OFF_W in CTRL9.
#[register(address = Reg::ZOfsUsr, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct ZOfsUsr {
    /// 8-bit user offset for accelerometer Z-axis.
    #[bits(8)]
    pub z_ofs_usr: u8,
}

/// FIFO_DATA_OUT_TAG (0x78)
///
/// FIFO tag register (R).
/// Identifies sensor source and tag count for FIFO data output.
#[register(address = Reg::FifoDataOutTag, access_type = Lsm6dsv320x, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoDataOutTag {
    /// Reserved, read-only (1 bit).
    #[bits(1, access = RO)]
    not_used0: u8,
    /// 2-bit counter identifying sensor time slot.
    #[bits(2)]
    pub tag_cnt: u8,
    /// 5-bit FIFO tag identifying sensor type (see datasheet Table 232).
    #[bits(5)]
    pub tag_sensor: u8,
}

/// FIFO_DATA_OUT_X_L - FIFO_DATA_OUT_Z_H (0x79 - 0x7E)
///
/// FIFO data output X, Y, Z (R)
#[register(address = Reg::FifoDataOutXL, access_type = Lsm6dsv320x, generics = 2)]
pub struct FifoDataOutXYZ(pub [u8; 6]);

pub struct XlOffsetMg {
    pub z_mg: f32,
    pub y_mg: f32,
    pub x_mg: f32,
}

#[derive(Default)]
/// Include descript of events for High-G
pub struct HgEvent {
    /// Interrupt event (hg_event)
    pub hg_event: u8,
    /// Wakeup event for Z-axis
    pub hg_wakeup_z: u8,
    /// Wakeup event for Y-axis
    pub hg_wakeup_y: u8,
    /// Wakeup event for X-axis
    pub hg_wakeup_x: u8,
    /// Wakeup event
    pub hg_wakeup: u8,
    /// Wakeup change event
    pub hg_wakeup_chg: u8,
    /// Shock function state
    pub hg_shock: u8,
    /// Shock change event detection state
    pub hg_shock_change: u8,
}

#[derive(Default)]
pub struct HgWakeUpCfg {
    pub hg_wakeup_ths: u8,
    pub hg_shock_dur: u8,
}

#[derive(Default)]
pub struct HgWuInterruptCfg {
    pub hg_interrupts_enable: u8,
    pub hg_wakeup_int_sel: u8,
}

#[derive(Default)]
pub struct InterruptMode {
    pub enable: u8,
    pub lir: u8,
}

#[derive(Default)]
pub struct PinIntRoute {
    pub drdy_xl: u8,
    pub drdy_g: u8,
    /// available only on pin2
    pub drdy_g_eis: u8,
    /// available only on pin2
    pub drdy_temp: u8,
    pub fifo_th: u8,
    pub fifo_ovr: u8,
    pub fifo_full: u8,
    pub cnt_bdr: u8,
    pub timestamp: u8,
    pub shub: u8,
    pub sixd: u8,
    pub single_tap: u8,
    pub double_tap: u8,
    pub wakeup: u8,
    pub freefall: u8,
    pub sleep_change: u8,
    pub drdy_hg_xl: u8,
    pub hg_wakeup: u8,
    pub hg_shock_change: u8,
    pub step_detector: u8,
    pub tilt: u8,
    pub sig_mot: u8,
    pub emb_func_endop: u8,
    pub fsm1: u8,
    pub fsm2: u8,
    pub fsm3: u8,
    pub fsm4: u8,
    pub fsm5: u8,
    pub fsm6: u8,
    pub fsm7: u8,
    pub fsm8: u8,
    pub mlc1: u8,
    pub mlc2: u8,
    pub mlc3: u8,
    pub mlc4: u8,
    pub mlc5: u8,
    pub mlc6: u8,
    pub mlc7: u8,
    pub mlc8: u8,
}

#[derive(Default)]
pub struct DataReady {
    pub drdy_hgxl: u8,
    pub drdy_xl: u8,
    pub drdy_gy: u8,
    pub drdy_temp: u8,
}

#[derive(Default)]
pub struct FifoStatus {
    // Number of element stored in the fifo
    pub fifo_level: u16,
    // Counter batch data rate (bdr) interrupt active flag
    pub fifo_bdr: u8,
    // FIFO full interrupt active flag
    pub fifo_full: u8,
    // FIFO overrun interrup active flag
    pub fifo_ovr: u8,
    // FIFO watermark interrupt active flag
    pub fifo_th: u8,
}

#[derive(Default)]
pub struct FiltSettlingMask {
    pub drdy: u8,
    pub ois_drdy: u8,
    pub irq_xl: u8,
    pub irq_xl_hg: u8,
    pub irq_g: u8,
}

#[derive(Default)]
pub struct OisHandshake {
    pub ack: u8,
    pub req: u8,
}

#[derive(Default)]
pub struct OisChain {
    pub gy: u8,
    pub xl: u8,
}

#[derive(Default)]
pub struct TapDetection {
    pub tap_x_en: u8,
    pub tap_y_en: u8,
    pub tap_z_en: u8,
}

#[derive(Default)]
pub struct TapThresholds {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Default)]
pub struct TapTimeWindows {
    pub shock: u8,
    pub quiet: u8,
    pub tap_gap: u8,
}

#[derive(Default)]
pub struct ActThresholds {
    pub inactivity_cfg: InactivityDur,
    pub inactivity_ths: u8,
    pub threshold: u8,
    pub duration: u8,
}

#[derive(Default)]
pub struct ActWkupTimeWindows {
    pub shock: u8,
    pub quiet: u8,
}

#[allow(dead_code)]
#[derive(Default)]
pub struct FifoOutRaw {
    pub tag: Tag,
    pub cnt: u8,
    pub data: [u8; 6],
}

pub struct I3cConfig {
    pub if2_ta0_pid: u8,
    pub rst_mode: RstMode,
    pub ibi_time: IbiTime,
}

/// Reset types for the device.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, TryFrom)]
#[try_from(repr)]
pub enum Reset {
    /// No reset, device ready state.
    Ready = 0x0,
    /// Global reset of the device (software power-on reset).
    GlobalRst = 0x1,
    /// Restore factory calibration parameters.
    RestoreCalParam = 0x2,
    /// Restore control registers to default values.
    RestoreCtrlRegs = 0x4,
}

/// Output Data Rate (ODR) settings for accelerometer and gyroscope.
///
/// Includes standard and high-accuracy ODR modes with various frequencies.
/// The high-accuracy modes are indicated by higher bits set.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum DataRate {
    /// Output data rate off (default).
    #[default]
    Off = 0x0,
    /// 1.875 Hz
    _1_875hz = 0x1,
    /// 7.5 Hz
    _7_5hz = 0x2,
    /// 15 Hz
    _15hz = 0x3,
    /// 30 Hz
    _30hz = 0x4,
    /// 60 Hz
    _60hz = 0x5,
    /// 120 Hz
    _120hz = 0x6,
    /// 240 Hz
    _240hz = 0x7,
    /// 480 Hz
    _480hz = 0x8,
    /// 960 Hz
    _960hz = 0x9,
    /// 1920 Hz
    _1920hz = 0xA,
    /// 3840 Hz
    _3840hz = 0xB,
    /// 7680 Hz
    _7680hz = 0xC,
    /// High-accuracy ODR 15.625 Hz
    Ha01At15_625hz = 0x13,
    /// High-accuracy ODR 31.25 Hz
    Ha01At31_25hz = 0x14,
    /// High-accuracy ODR 62.5 Hz
    Ha01At62_5hz = 0x15,
    /// High-accuracy ODR 125 Hz
    Ha01At125hz = 0x16,
    /// High-accuracy ODR 250 Hz
    Ha01At250hz = 0x17,
    /// High-accuracy ODR 500 Hz
    Ha01At500hz = 0x18,
    /// High-accuracy ODR 1000 Hz
    Ha01At1000hz = 0x19,
    /// High-accuracy ODR 2000 Hz
    Ha01At2000hz = 0x1A,
    /// High-accuracy ODR 4000 Hz
    Ha01At4000hz = 0x1B,
    /// High-accuracy ODR 8000 Hz
    Ha01At8000hz = 0x1C,
    /// High-accuracy ODR 12.5 Hz (second set)
    Ha02At12_5hz = 0x23,
    /// High-accuracy ODR 25 Hz (second set)
    Ha02At25hz = 0x24,
    /// High-accuracy ODR 50 Hz (second set)
    Ha02At50hz = 0x25,
    /// High-accuracy ODR 100 Hz (second set)
    Ha02At100hz = 0x26,
    /// High-accuracy ODR 200 Hz (second set)
    Ha02At200hz = 0x27,
    /// High-accuracy ODR 400 Hz (HA02)
    Ha02At400hz = 0x28,
    /// High-accuracy ODR 800 Hz (HA02)
    Ha02At800hz = 0x29,
    /// High-accuracy ODR 1600 Hz (HA02)
    Ha02At1600hz = 0x2A,
    /// High-accuracy ODR 3200 Hz (HA02)
    Ha02At3200hz = 0x2B,
    /// High-accuracy ODR 6400 Hz (HA02)
    Ha02At6400hz = 0x2C,
    /// High-accuracy ODR 13 Hz (HA03)
    Ha03At13hz = 0x33,
    /// High-accuracy ODR 26 Hz (HA03)
    Ha03At26hz = 0x34,
    /// High-accuracy ODR 52 Hz (HA03)
    Ha03At52hz = 0x35,
    /// High-accuracy ODR 104 Hz (HA03)
    Ha03At104hz = 0x36,
    /// High-accuracy ODR 208 Hz (HA03)
    Ha03At208hz = 0x37,
    /// High-accuracy ODR 417 Hz (HA03)
    Ha03At417hz = 0x38,
    /// High-accuracy ODR 833 Hz (HA03)
    Ha03At833hz = 0x39,
    /// High-accuracy ODR 1667 Hz (HA03)
    Ha03At1667hz = 0x3A,
    /// High-accuracy ODR 3333 Hz (HA03)
    Ha03At3333hz = 0x3B,
    /// High-accuracy ODR 6667 Hz (HA03)
    Ha03At6667hz = 0x3C,
}

/// High-G accelerometer output data rate (ODR) selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum HgXlDataRate {
    /// Output data rate off (default).
    #[default]
    Off = 0x0,
    /// 480 Hz
    _480hz = 0x3,
    /// 960 Hz
    _960hz = 0x4,
    /// 1920 Hz
    _1920hz = 0x5,
    /// 3840 Hz
    _3840hz = 0x6,
    /// 7680 Hz
    _7680hz = 0x7,
}

/// Accelerometer operating mode selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum XlMode {
    /// High-performance mode (default).
    #[default]
    HighPerformance = 0x0,
    /// High-accuracy ODR mode.
    HighAccuracyOdr = 0x1,
    /// ODR-triggered mode.
    OdrTriggered = 0x3,
    /// Low-power mode 1 (2 mean).
    LowPower2Avg = 0x4,
    /// Low-power mode 2 (4 mean).
    LowPower4Avg = 0x5,
    /// Low-power mode 3 (8 mean).
    LowPower8Avg = 0x6,
    /// Normal mode.
    Normal = 0x7,
}

/// Gyroscope operating mode selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum GyMode {
    /// High-performance mode (default).
    #[default]
    HighPerformance = 0x0,
    /// High-accuracy ODR mode.
    HighAccuracyOdr = 0x1,
    /// ODR-triggered mode.
    OdrTriggered = 0x3,
    /// Sleep mode.
    Sleep = 0x4,
    /// Low-power mode.
    LowPower = 0x5,
}

/// Data ready signal mode.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum DataReadyMode {
    /// Latched mode (default).
    #[default]
    Latched = 0x0,
    /// Pulsed mode (~75 µs pulse).
    Pulsed = 0x1,
}

/// Gyroscope full-scale selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum GyFullScale {
    /// ±250 dps (default).
    #[default]
    _250dps = 0x1,
    /// ±500 dps.
    _500dps = 0x2,
    /// ±1000 dps.
    _1000dps = 0x3,
    /// ±2000 dps.
    _2000dps = 0x4,
    /// ±4000 dps.
    _4000dps = 0x5,
}

/// Accelerometer full-scale selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum XlFullScale {
    /// ±2 g (default).
    #[default]
    _2g = 0x0,
    /// ±4 g.
    _4g = 0x1,
    /// ±8 g.
    _8g = 0x2,
    /// ±16 g.
    _16g = 0x3,
}

/// High-G accelerometer full-scale selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum HgXlFullScale {
    /// ±32 g (default).
    #[default]
    _32g = 0x0,
    /// ±64 g.
    _64g = 0x1,
    /// ±128 g.
    _128g = 0x2,
    /// ±256 g.
    _256g = 0x3,
    /// ±320 g.
    _320g = 0x4,
}

/// Accelerometer and gyroscope self-test selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum SelfTest {
    /// Self-test disabled (default).
    #[default]
    Disable = 0x0,
    /// Positive sign self-test.
    Positive = 0x1,
    /// Negative sign self-test.
    Negative = 0x2,
}

/// INT2 pin input trigger polarity.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum DenPolarity {
    /// Active low (default).
    #[default]
    ActiveLow = 0x0,
    /// Active high.
    ActiveHigh = 0x1,
}

/// Gyroscope full-scale selection for EIS channel.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum EisGyFullScale {
    /// ±250 dps (default).
    #[default]
    _250dps = 0x1,
    /// ±500 dps.
    _500dps = 0x2,
    /// ±1000 dps.
    _1000dps = 0x3,
    /// ±2000 dps.
    _2000dps = 0x4,
    /// ±4000 dps.
    _4000dps = 0x5,
}

/// Gyroscope EIS output data rate selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum EisGyDataRate {
    /// EIS channel off.
    OdrOff = 0x0,
    /// 1.92 kHz (default).
    #[default]
    _1920hz = 0x1,
    /// 960 Hz.
    _960hz = 0x2,
}

/// FIFO compression algorithm selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoCompressAlgo {
    /// Compression disabled (default).
    #[default]
    Disable = 0x0,
    /// Compression ratio 8:1.
    _8To1 = 0x1,
    /// Compression ratio 16:1.
    _16To1 = 0x2,
    /// Compression ratio 32:1.
    _32To1 = 0x3,
}

/// FIFO batch data rate selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoBatch {
    /// Not batched (default).
    #[default]
    NotBatched = 0x0,
    /// 1.875 Hz.
    _1_875hz = 0x1,
    /// 7.5 Hz.
    _7_5hz = 0x2,
    /// 15 Hz.
    _15hz = 0x3,
    /// 30 Hz.
    _30hz = 0x4,
    /// 60 Hz.
    _60hz = 0x5,
    /// 120 Hz.
    _120hz = 0x6,
    /// 240 Hz.
    _240hz = 0x7,
    /// 480 Hz.
    _480hz = 0x8,
    /// 960 Hz.
    _960hz = 0x9,
    /// 1920 Hz.
    _1920hz = 0xA,
    /// 3840 Hz.
    _3840hz = 0xB,
    /// 7680 Hz.
    _7680hz = 0xC,
}

/// FIFO mode selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoMode {
    /// Bypass mode: FIFO disabled (default).
    #[default]
    Bypass = 0x0,
    /// FIFO mode: stops collecting data when FIFO is full.
    Fifo = 0x1,
    /// Continuous WTM-to-full mode.
    StreamWtmToFull = 0x2,
    /// Continuous-to-FIFO mode.
    StreamToFifo = 0x3,
    /// Bypass-to-continuous mode.
    BypassToStream = 0x4,
    /// Continuous mode: new samples overwrite older ones.
    Stream = 0x6,
    /// Bypass-to-FIFO mode.
    BypassToFifo = 0x7,
}

/// Batch data rate (write frequency in FIFO) for temperature data.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoTempBatch {
    /// Temperature not batched (default).
    #[default]
    NotBatched = 0x0,
    /// 1.875 Hz.
    _1_875hz = 0x1,
    /// 15 Hz.
    _15hz = 0x2,
    /// 60 Hz.
    _60hz = 0x3,
}

/// Decimation for timestamp batching in FIFO.
///
/// Write rate is the maximum rate between accelerometer and gyroscope BDR divided by decimation factor.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoTimestampBatch {
    /// Timestamp not batched (default).
    #[default]
    NotBatched = 0x0,
    /// Decimation 1: write at max(BDR_XL, BDR_GY).
    Dec1 = 0x1,
    /// Decimation 8: write at max(BDR_XL, BDR_GY)/8.
    Dec8 = 0x2,
    /// Decimation 32: write at max(BDR_XL, BDR_GY)/32.
    Dec32 = 0x3,
}

/// Trigger for the internal counter of batch events.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FifoBatchCntEvent {
    /// Low-g accelerometer batch event (default).
    #[default]
    XlLgBatch = 0x0,
    /// Gyroscope batch event.
    GyBatch = 0x1,
    /// Enhanced EIS gyroscope batch event.
    GyEisBatch = 0x2,
    /// High-g accelerometer batch event.
    XlHgBatch = 0x3,
}

/// Protocol anti-spike filter configuration.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FiltAntiSpike {
    /// Antispike filters managed by protocol (default).
    #[default]
    Auto = 0x0,
    /// Antispike filters always enabled.
    AlwaysActive = 0x1,
}

/// Gyroscope low-pass filter (LPF1) bandwidth selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FiltLpBandwidth {
    /// Ultra light bandwidth (default).
    #[default]
    UltraLight = 0x0,
    /// Very light bandwidth.
    VeryLight = 0x1,
    /// Light bandwidth.
    Light = 0x2,
    /// Medium bandwidth.
    Medium = 0x3,
    /// Strong bandwidth.
    Strong = 0x4,
    /// Very strong bandwidth.
    VeryStrong = 0x5,
    /// Aggressive bandwidth.
    Aggressive = 0x6,
    /// Extreme bandwidth.
    Xtreme = 0x7,
}

/// Accelerometer high-pass filter mode.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FiltXlHpMode {
    /// Normal mode (default).
    #[default]
    Normal = 0x0,
    /// Reference mode.
    Reference = 0x1,
}

/// Filter selection for wake-up and activity/inactivity functions.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FiltWkupActFeed {
    /// Slope filter applied (default).
    #[default]
    Slope = 0x0,
    /// High-pass filter applied.
    HighPass = 0x1,
    /// Low-pass filter with offset.
    LpWithOffset = 0x2,
}

/// LPF2 filter on 6D (sixd) function selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FiltSixdFeed {
    /// ODR divided by 2 (default).
    #[default]
    OdrDiv2 = 0x0,
    /// Low-pass filter.
    LowPass = 0x1,
}

/// Gyroscope digital LPF_EIS filter bandwidth selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FiltGyEisLpBandwidth {
    /// Normal bandwidth (default).
    #[default]
    Normal = 0x0,
    /// Light bandwidth.
    Light = 0x1,
}

/// Gyroscope OIS digital LPF1 filter bandwidth selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FiltGyOisLpBandwidth {
    /// Normal bandwidth (default).
    #[default]
    Normal = 0x0,
    /// Strong bandwidth.
    Strong = 0x1,
    /// Aggressive bandwidth.
    Aggressive = 0x2,
    /// Light bandwidth.
    Light = 0x3,
}

/// Accelerometer OIS channel bandwidth selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FiltXlOisLpBandwidth {
    /// Ultra light bandwidth (default).
    #[default]
    UltraLight = 0x0,
    /// Very light bandwidth.
    VeryLight = 0x1,
    /// Light bandwidth.
    Light = 0x2,
    /// Normal bandwidth.
    Normal = 0x3,
    /// Strong bandwidth.
    Strong = 0x4,
    /// Very strong bandwidth.
    VeryStrong = 0x5,
    /// Aggressive bandwidth.
    Aggressive = 0x6,
    /// Extreme bandwidth.
    Xtreme = 0x7,
}

/// FSM permission to write control registers.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FsmPermission {
    /// Protect control registers from FSM write.
    #[default]
    ProtectCtrlRegs = 0x0,
    /// Allow FSM to write control registers.
    WriteCtrlReg = 0x1,
}

/// Free-fall threshold settings.
///
/// Defines the threshold for free-fall detection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum FfThreshold {
    /// 156 mg threshold (default).
    #[default]
    _156mg = 0x0,
    /// 219 mg threshold.
    _219mg = 0x1,
    /// 250 mg threshold.
    _250mg = 0x2,
    /// 312 mg threshold.
    _312mg = 0x3,
    /// 344 mg threshold.
    _344mg = 0x4,
    /// 406 mg threshold.
    _406mg = 0x5,
    /// 469 mg threshold.
    _469mg = 0x6,
    /// 500 mg threshold.
    _500mg = 0x7,
}

/// OIS control mode.
///
/// - `FromOis`: OIS chain full control from auxiliary interface (default).
/// - `FromUi`: OIS chain full control from primary interface.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum OisCtrlMode {
    /// Control from auxiliary interface (OIS).
    #[default]
    FromOis = 0x0,
    /// Control from primary interface (UI).
    FromUi = 0x1,
}

/// Gyroscope OIS full-scale selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum OisGyFullScale {
    /// ±250 dps (default).
    #[default]
    _250dps = 0x1,
    /// ±500 dps.
    _500dps = 0x2,
    /// ±1000 dps.
    _1000dps = 0x3,
    /// ±2000 dps.
    _2000dps = 0x4,
}

/// Accelerometer OIS full-scale selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum OisXlFullScale {
    /// ±2 g (default).
    #[default]
    _2g = 0x0,
    /// ±4 g.
    _4g = 0x1,
    /// ±8 g.
    _8g = 0x2,
    /// ±16 g.
    _16g = 0x3,
}

/// Threshold for 4D/6D function.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum SixdThreshold {
    /// 80 degrees threshold (default).
    #[default]
    _80deg = 0x0,
    /// 70 degrees threshold.
    _70deg = 0x1,
    /// 60 degrees threshold.
    _60deg = 0x2,
    /// 50 degrees threshold.
    _50deg = 0x3,
}

/// UI I2C and MIPI I3C interface mode.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum UiI2cI3cMode {
    /// SPI, I2C, and MIPI I3C interfaces enabled.
    #[default]
    Enable = 0x0,
    /// I2C and MIPI I3C interfaces disabled.
    Disable = 0x1,
}

/// Pad drive strength selection.
///
/// - `Low`: Lowest strength (recommended for VDDIO ≥ 3.0 V).
/// - `Middle`: Intermediate strength (recommended for 2.0 V ≤ VDDIO < 3.0 V).
/// - `High`: Highest strength (recommended for VDDIO < 2.0 V, default).
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum PadStrength {
    /// Lowest drive strength.
    Low = 0x1,
    /// Intermediate drive strength.
    Middle = 0x2,
    /// Highest drive strength.
    #[default]
    High = 0x4,
}

/// SPI serial interface mode selection.
///
/// - `Spi4Wire`: 4-wire SPI interface (default).
/// - `Spi3Wire`: 3-wire SPI interface.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum SpiMode {
    /// 4-wire SPI interface.
    #[default]
    Spi4Wire = 0x0,
    /// 3-wire SPI interface.
    Spi3Wire = 0x1,
}

/// Tap axis priority selection.
///
/// Defines the priority order of axes for tap detection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum TapAxisPriority {
    /// X > Y > Z priority.
    #[default]
    Xyz = 0x0,
    /// Y > X > Z priority.
    Yxz = 0x1,
    /// X > Z > Y priority.
    Xzy = 0x2,
    /// Z > Y > X priority.
    Zyx = 0x3,
    /// Y > Z > X priority.
    Yzx = 0x5,
    /// Z > X > Y priority.
    Zxy = 0x6,
}

/// Tap mode selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum TapMode {
    /// Only single-tap event enabled.
    #[default]
    OnlySingle = 0x0,
    /// Both single and double-tap events enabled.
    BothSingleDouble = 0x1,
}

/// Activity/Inactivity (sleep) mode selection.
///
/// - `XlAndGyNotAffected`: Stationary/motion-only interrupts generated; accelerometer and gyroscope configuration unchanged (default).
/// - `XlLowPowerGyNotAffected`: Accelerometer in low-power mode 1; gyroscope configuration unchanged.
/// - `XlLowPowerGySleep`: Accelerometer in low-power mode 1; gyroscope in sleep mode.
/// - `XlLowPowerGyPowerDown`: Accelerometer in low-power mode 1; gyroscope in power-down mode.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ActMode {
    /// Stationary/motion-only interrupts; no config change.
    #[default]
    XlAndGyNotAffected = 0x0,
    /// Accelerometer low-power mode 1; gyroscope unchanged.
    XlLowPowerGyNotAffected = 0x1,
    /// Accelerometer low-power mode 1; gyroscope sleep mode.
    XlLowPowerGySleep = 0x2,
    /// Accelerometer low-power mode 1; gyroscope power-down mode.
    XlLowPowerGyPowerDown = 0x3,
}

/// Duration in transition from inactivity to activity.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ActFromSleepToActDur {
    /// Immediate transition at first over-threshold event.
    #[default]
    _1stSample = 0x0,
    /// Transition after two consecutive over-threshold events.
    _2ndSample = 0x1,
    /// Transition after three consecutive over-threshold events.
    _3rdSample = 0x2,
    /// Transition after four consecutive over-threshold events.
    _4thSample = 0x3,
}

/// Accelerometer output data rate during inactivity.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ActSleepXlOdr {
    /// 1.875 Hz output data rate.
    #[default]
    _1_875hz = 0x0,
    /// 15 Hz output data rate.
    _15hz = 0x1,
    /// 30 Hz output data rate.
    _30hz = 0x2,
    /// 60 Hz output data rate.
    _60hz = 0x3,
}

/// In-band interrupt (IBI) bus available time selection.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum IbiTime {
    /// 50 microseconds bus available time.
    #[default]
    _50us = 0x0,
    /// 2 microseconds bus available time.
    _2us = 0x1,
    /// 1 millisecond bus available time.
    _1ms = 0x2,
    /// 50 milliseconds bus available time.
    _50ms = 0x3,
}

/// Reset mode after "reset whole chip" I3C pattern.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum RstMode {
    /// Configuration reset (software reset + dynamic address reset).
    #[default]
    SwRstDynAddress = 0x0,
    /// Global reset (power-on reset).
    I3cGlobal = 0x1,
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Tag {
    #[default]
    FifoEmpty = 0x0,
    GyNc = 0x1,
    XlNc = 0x2,
    Temperature = 0x3,
    Timestamp = 0x4,
    CfgChange = 0x5,
    XlNcT2 = 0x6,
    XlNcT1 = 0x7,
    Xl2Xc = 0x8,
    Xl3Xc = 0x9,
    GyNcT2 = 0xA,
    GyNcT1 = 0xB,
    Gy2Xc = 0xC,
    Gy3Xc = 0xD,
    SensorhubTarget0 = 0xE,
    SensorhubTarget1 = 0xF,
    SensorhubTarget2 = 0x10,
    SensorhubTarget3 = 0x11,
    StepCounter = 0x12,
    SflpGameRotationVector = 0x13,
    SflpGyroscopeBias = 0x16,
    SflpGravityVector = 0x17,
    HgXlPeak = 0x18,
    SensorhubNack = 0x19,
    MlcResult = 0x1A,
    MlcFilter = 0x1B,
    MlcFeature = 0x1C,
    XlHg = 0x1D,
    GyEnhancedEis = 0x1E,
    FsmResult = 0x1F,
}
