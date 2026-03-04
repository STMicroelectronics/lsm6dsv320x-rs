use defmt::info;
use maybe_async::maybe_async;
use crate::*;

#[maybe_async]
pub async fn run<B, D, L, I>(bus: B, mut tx: L, mut delay: D, mut int_pin: I) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write,
    I: InterruptPin
{
    use lsm6dsv320x::*;
    use lsm6dsv320x::prelude::*;

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
    // Enable Block Data Update
    sensor.block_data_update_set(1).await.unwrap();

    // Set Otuput Data Rate
    sensor.hg_xl_data_rate_set(HgXlDataRate::_960hz, 1).await.unwrap();

    // Set full scale
    sensor.hg_xl_full_scale_set(HgXlFullScale::_256g).await.unwrap();

    // Configure filtering chain
    let mut filt_settling_mask = FiltSettlingMask::default();
    filt_settling_mask.drdy = 1;
    filt_settling_mask.irq_xl = 1;
    filt_settling_mask.irq_g = 1;

    sensor.filt_settling_mask_set(filt_settling_mask).await.unwrap();
    sensor.filt_xl_lp2_set(1).await.unwrap();
    sensor
        .filt_xl_lp2_bandwidth_set(FiltLpBandwidth::Strong)
        .await.unwrap();

    let mut wakeup_cfg = HgWakeUpCfg::default();
    wakeup_cfg.hg_shock_dur = 1;
    wakeup_cfg.hg_wakeup_ths = 4;
    sensor.hg_wake_up_cfg_set(wakeup_cfg).await.unwrap();

    // Enable interrupt on High-G XL (sensor at highest frequency)
    let mut pin_int = PinIntRouteHg::default();
    pin_int.hg_wakeup = 1;
    sensor.pin_int1_route_hg_set(&pin_int).await.unwrap();
    //sensor.pint_int2_route_hg_set(&pin_int).await.unwrap();

    let mut int_cfg = HgWuInterruptCfg::default();
    int_cfg.hg_interrupts_enable = 1;
    sensor.hg_wu_interrupt_cfg_set(int_cfg).await.unwrap();

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let status = sensor.hg_event_get().await.unwrap();

        if status.hg_event == 1 {
            let axis_x = status.hg_wakeup_x;
            let axis_y = status.hg_wakeup_y;
            let axis_z = status.hg_wakeup_z;

            writeln!(
                tx,
                "WAKEUP event on X: {}, Y: {}, Z: {}",
                axis_x, axis_y, axis_z
            )
            .unwrap();
        }
    }
}
