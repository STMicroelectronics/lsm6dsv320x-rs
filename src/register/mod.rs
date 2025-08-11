pub mod advanced;
pub mod embedded;
pub mod if2;
pub mod main;
pub mod sensor_hub;

use crate::Error;
use crate::Lsm6dsv320x;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::mem_bank;
use st_mems_bus::{BusOperation, MemBankFunctions};

/// Memory banks available in the device.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[mem_bank(Lsm6dsv320x, generics = 2)]
pub enum MemBank {
    /// Main memory bank (default).
    #[main]
    MainMemBank = 0x0,
    /// Embedded functions memory bank.
    #[state(EmbedFuncState, fn_name = "operate_over_embed")]
    EmbedFuncMemBank = 0x1,
    /// Sensor hub memory bank.
    #[state(SensorHubState, fn_name = "operate_over_sensor_hub")]
    SensorHubMemBank = 0x2,
}
