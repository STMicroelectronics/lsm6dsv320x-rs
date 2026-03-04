use defmt::info;
use maybe_async::maybe_async;
use crate::*;

const CNT_FOR_OUTPUT: u16 = 100;

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
    sensor.xl_setup(DataRate::_1920hz, XlMode::HighPerformance).await.unwrap();
    sensor.hg_xl_data_rate_set(HgXlDataRate::_960hz, 1).await.unwrap();
    sensor.gy_setup(DataRate::_120hz, GyMode::HighPerformance).await.unwrap();

    // Set full scale
    sensor.xl_full_scale_set(XlFullScale::_2g).await.unwrap();
    sensor.hg_xl_full_scale_set(HgXlFullScale::_320g).await.unwrap();
    sensor.gy_full_scale_set(GyFullScale::_2000dps).await.unwrap();

    // Configure filtering chain
    let mut filt_settling_mask = FiltSettlingMask::default();
    filt_settling_mask.drdy = 1;
    filt_settling_mask.irq_xl = 1;
    filt_settling_mask.irq_g = 1;

    sensor.filt_settling_mask_set(filt_settling_mask).await.unwrap();
    sensor.filt_gy_lp1_set(1).await.unwrap();
    sensor
        .filt_gy_lp1_bandwidth_set(FiltLpBandwidth::UltraLight)
        .await.unwrap();
    sensor.filt_xl_lp2_set(1).await.unwrap();
    sensor
        .filt_xl_lp2_bandwidth_set(FiltLpBandwidth::Strong)
        .await.unwrap();

    // Enable interrupt on High-G XL (sensor at highest frequency)
    let mut pin_int = PinIntRouteHg::default();
    pin_int.drdy_hg_xl = 1;
    sensor.pin_int1_route_hg_set(&pin_int).await.unwrap();
    //sensor.pint_int2_route_hg_set(&pin_int).await.unwrap();

    let mut lowg_xl_sum: [f32; 3] = [0.0; 3];
    let mut lowg_xl_cnt: u16 = 0;
    let mut hg_xl_sum: [f32; 3] = [0.0; 3];
    let mut hg_xl_cnt: u16 = 0;
    let mut gyro_sum: [f32; 3] = [0.0; 3];
    let mut gyro_cnt: u16 = 0;
    let mut temp_sum: f32 = 0.0;
    let mut temp_cnt: u16 = 0;

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let status = sensor.flag_data_ready_get().await.unwrap();

        // Read output only if new xl value is available
        if status.drdy_xl == 1 {
            // Read acceleration data
            let data_raw_acceleration = sensor.acceleration_raw_get().await.unwrap();

            for i in 0..3 {
                let acceleration_mg = from_fs2_to_mg(data_raw_acceleration[i]);
                lowg_xl_sum[i] += acceleration_mg;
            }
            lowg_xl_cnt += 1;
        }

        if status.drdy_hgxl == 1 {
            // Read acceleration field data
            let data_raw_motion = sensor.hg_acceleration_raw_get().await.unwrap();

            for i in 0..3 {
                let acceleration_mg = from_fs320_to_mg(data_raw_motion[i]);
                hg_xl_sum[i] += acceleration_mg;
            }
            hg_xl_cnt += 1;
        }

        // Read output only if new gyroscope value is available
        if status.drdy_gy == 1 {
            // Read angular rate data
            let data_raw_angular_rate = sensor.angular_rate_raw_get().await.unwrap();

            for i in 0..3 {
                let angular_rate_mdps =
                    from_fs2000_to_mdps(data_raw_angular_rate[i]);
                gyro_sum[i] += angular_rate_mdps;
            }
            gyro_cnt += 1;
        }

        if status.drdy_temp == 1 {
            // Read temperature data
            let data_raw_temperature = sensor.temperature_raw_get().await.unwrap();
            let temperature_deg_c = from_lsb_to_celsius(data_raw_temperature);

            temp_sum += temperature_deg_c;
            temp_cnt += 1;
        }

        if lowg_xl_cnt >= CNT_FOR_OUTPUT {
            // Print media low-g xl data
            let acceleration_mg = [
                lowg_xl_sum[0] / lowg_xl_cnt as f32,
                lowg_xl_sum[1] / lowg_xl_cnt as f32,
                lowg_xl_sum[2] / lowg_xl_cnt as f32,
            ];

            writeln!(
                tx,
                "lg xl (media of {} samples) [mg]: {:.2}\t{:.2}\t{:.2}",
                lowg_xl_cnt, acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();

            lowg_xl_sum = [0 as f32; 3];
            lowg_xl_cnt = 0;

            // Print media high-g xl data
            let acceleration_mg = [
                hg_xl_sum[0] / hg_xl_cnt as f32,
                hg_xl_sum[1] / hg_xl_cnt as f32,
                hg_xl_sum[2] / hg_xl_cnt as f32,
            ];

            writeln!(
                tx,
                "hg xl (media of {} samples) [mg]: {:.2}\t{:.2}\t{:.2}",
                hg_xl_cnt, acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();

            hg_xl_sum = [0 as f32; 3];
            hg_xl_cnt = 0;

            // Print media gyro data
            let angular_rate_mdps = [
                gyro_sum[0] / gyro_cnt as f32,
                gyro_sum[1] / gyro_cnt as f32,
                gyro_sum[2] / gyro_cnt as f32,
            ];

            writeln!(
                tx,
                "gyro (media of {} samples) [mdps]: {:.2}\t{:.2}\t{:.2}",
                gyro_cnt, angular_rate_mdps[0], angular_rate_mdps[1], angular_rate_mdps[2]
            )
            .unwrap();

            gyro_sum = [0.0; 3];
            gyro_cnt = 0;

            // Print media temperature data
            let temperature_deg_c = temp_sum as f32 / temp_cnt as f32;
            writeln!(
                tx,
                "Temperature (media of {} samples) [degC]:{:.2}\n",
                temp_cnt, temperature_deg_c
            )
            .unwrap();
            temp_sum = 0.0;
            temp_cnt = 0;
        }
    }
}
