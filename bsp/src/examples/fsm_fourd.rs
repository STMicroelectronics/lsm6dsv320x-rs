use defmt::info;
use maybe_async::maybe_async;
use crate::*;

use crate::config::fsm_config::FOUR_D;
use st_mems_reg_config_conv::ucf_entry::MemsUcfOp;

#[maybe_async]
pub async fn run<B, D, L, I>(bus: B, mut tx: L, mut delay: D, mut int_pin: I) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
    I: InterruptPin
{
    use lsm6dsv320x::*;

    info!("Configuring the sensor");
    let mut sensor = Lsm6dsv320x::from_bus(bus, delay.clone());

    // boot time
    delay.delay_ms(10).await;

    // Check device ID
    let whoami = sensor.device_id_get().await.unwrap();
    if whoami != ID {
        writeln!(tx, "Device ID mismatch: {:#02x}", whoami).unwrap();
        loop {}
    }

    // Restore default configuration
    sensor.sw_reset().await.unwrap();

    for ucf_entry in FOUR_D {
        match ucf_entry.op {
            MemsUcfOp::Delay => {
                delay.delay_ms(ucf_entry.data.into()).await;
            }
            MemsUcfOp::Write => {
                sensor
                    .bus
                    .write_to_register(ucf_entry.address as u8, &[ucf_entry.data])
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let status = sensor.all_sources_get().await.unwrap();
        if status.fsm1 == 1 {
            let fsm_out = sensor.fsm_out_get().await.unwrap();
            match fsm_out.fsm_outs1 {
                0x10 => {
                    writeln!(tx, "Y down event").unwrap();
                }
                0x20 => {
                    writeln!(tx, "Y up event").unwrap();
                }
                0x40 => {
                    writeln!(tx, "X down event").unwrap();
                }
                0x80 => {
                    writeln!(tx, "X up event").unwrap();
                }
                _ => {}
            }
        }
    }
}
