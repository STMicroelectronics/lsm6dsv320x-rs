use defmt::info;
use maybe_async::maybe_async;
use crate::*;

const FIFO_WATERMARK: u8 = 128;

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

    // Set FIFO watermark (numer of unread sensor data TAG + 6 bytes
    // stored in FIFO) to FIFO_WATERMARK samples
    sensor.fifo_watermark_set(FIFO_WATERMARK).await.unwrap();

    // Set FIFO batch XL/Gyro ODR.
    sensor.fifo_xl_batch_set(FifoBatch::_60hz).await.unwrap();
    sensor.fifo_hg_xl_batch_set(1).await.unwrap();
    sensor.fifo_gy_batch_set(FifoBatch::_120hz).await.unwrap();
    // Set FIFO mode to Stream mode (aka Continuous Mode)
    sensor.fifo_mode_set(FifoMode::Stream).await.unwrap();
    sensor
        .fifo_timestamp_batch_set(FifoTimestampBatch::Dec32)
        .await
        .unwrap();
    sensor.timestamp_set(1).await.unwrap();

    // Set Otuput Data Rate
    sensor.xl_setup(DataRate::_60hz, XlMode::HighPerformance).await.unwrap();
    sensor.hg_xl_data_rate_set(HgXlDataRate::_480hz, 1).await.unwrap();
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
        .await
        .unwrap();
    sensor.filt_xl_lp2_set(1).await.unwrap();
    sensor
        .filt_xl_lp2_bandwidth_set(FiltLpBandwidth::Strong)
        .await
        .unwrap();

    // enable fifo_th on High-G XL (sensor at highest frequency)
    let mut pin_int = PinInt1Route::default();
    pin_int.fifo_th = 1;
    sensor.pin_int1_route_set(&pin_int).await.unwrap();
    //sensor.pin_int2_route_set(&pin_int);

    let mut lowg_xl_sum = [0f32; 3];
    let mut lowg_xl_cnt = 0;
    let mut hg_xl_sum = [0f32; 3];
    let mut hg_xl_cnt = 0;
    let mut gyro_sum = [0f32; 3];
    let mut gyro_cnt = 0;

    loop {
        // Wait for interrupt
        int_pin.wait_for_event().await;

        let fifo_status = sensor.fifo_status_get().await.unwrap();
        let num = fifo_status.fifo_level;

        for _ in 0..num {
            // Read FIFO sensor value
            let f_data = sensor.fifo_out_raw_get().await.unwrap();

            let data_x: i16 = (((f_data.data[1] as u16) << 8) | f_data.data[0] as u16) as i16;
            let data_y: i16 = (((f_data.data[3] as u16) << 8) | f_data.data[2] as u16) as i16;
            let data_z: i16 = (((f_data.data[5] as u16) << 8) | f_data.data[4] as u16) as i16;
            let raw_array_data = [data_x, data_y, data_z];
            let ts = u32::from_le_bytes([
                f_data.data[0],
                f_data.data[1],
                f_data.data[2],
                f_data.data[3],
            ]);

            match f_data.tag {
                Tag::XlNc => {
                    for (index, &data) in raw_array_data.iter().enumerate() {
                        let acc_mg = from_fs2_to_mg(data);
                        lowg_xl_sum[index] += acc_mg;
                    }
                    lowg_xl_cnt += 1;
                }
                Tag::XlHg => {
                    for (index, &data) in raw_array_data.iter().enumerate() {
                        let acc_mg = from_fs320_to_mg(data);
                        hg_xl_sum[index] += acc_mg;
                    }
                    hg_xl_cnt += 1;
                }
                Tag::GyNc => {
                    for (index, &data) in raw_array_data.iter().enumerate() {
                        let angular_rate_mdps = from_fs2000_to_mdps(data);
                        gyro_sum[index] += angular_rate_mdps;
                    }
                    gyro_cnt += 1;
                }
                Tag::Timestamp => {
                    writeln!(tx, "TIMESTAMP [ms] {}", ts).unwrap();

                    // Print media low-g xl data
                    if lowg_xl_cnt > 0 {
                        let acc_mg = [
                            lowg_xl_sum[0] / lowg_xl_cnt as f32,
                            lowg_xl_sum[1] / lowg_xl_cnt as f32,
                            lowg_xl_sum[2] / lowg_xl_cnt as f32,
                        ];

                        writeln!(
                            tx,
                            "lg xl (media of {} samples) [mg]:{:.2}\t{:.2}\t{:.2}",
                            lowg_xl_cnt, acc_mg[0], acc_mg[1], acc_mg[2]
                        )
                        .unwrap();

                        lowg_xl_sum = [0.0; 3];
                        lowg_xl_cnt = 0;
                    }

                    // print media high-g xl data
                    if hg_xl_cnt > 0 {
                        let acc_mg = [
                            hg_xl_sum[0] / hg_xl_cnt as f32,
                            hg_xl_sum[1] / hg_xl_cnt as f32,
                            hg_xl_sum[2] / hg_xl_cnt as f32,
                        ];

                        writeln!(
                            tx,
                            "hg xl (media of {} samples) [mg]:{:.2}\t{:.2}\t{:.2}",
                            hg_xl_cnt, acc_mg[0], acc_mg[1], acc_mg[2]
                        )
                        .unwrap();

                        hg_xl_sum = [0.0; 3];
                        hg_xl_cnt = 0;
                    }

                    // print media gyro data
                    if gyro_cnt > 0 {
                        let angular_rate_mdps = [
                            gyro_sum[0] / gyro_cnt as f32,
                            gyro_sum[1] / gyro_cnt as f32,
                            gyro_sum[2] / gyro_cnt as f32,
                        ];

                        writeln!(
                            tx,
                            "gyro (media of {} samples) [mdps]:{:.2}\t{:.2}\t{:.2}",
                            gyro_cnt,
                            angular_rate_mdps[0],
                            angular_rate_mdps[1],
                            angular_rate_mdps[2]
                        )
                        .unwrap();

                        gyro_sum = [0.0; 3];
                        gyro_cnt = 0;
                    }
                }
                _ => {
                    writeln!(tx, "[0x{:x}] UNHANDLED TAG", f_data.tag as u8).unwrap();
                }
            }
        }
    }
}
