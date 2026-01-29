pub mod advanced;
pub mod embedded;
pub mod if2;
pub mod main;
pub mod sensor_hub;

use super::{BusOperation, DelayNs, Error, Lsm6dsv320x, MemBankFunctions, only_async, only_sync};

use st_mem_bank_macro::mem_bank;

/// Memory banks available in the device.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[mem_bank(Lsm6dsv320x, generics = 2)]
pub enum MemBank {
    /// Main memory bank (default).
    #[main]
    MainMemBank = 0x0,
    /// Embedded functions memory bank.
    #[state(EmbedBank, fn_name = "operate_over_embed")]
    EmbedFuncMemBank = 0x1,
    /// Sensor hub memory bank.
    #[state(SensorHubBank, fn_name = "operate_over_sensor_hub")]
    SensorHubMemBank = 0x2,
}
