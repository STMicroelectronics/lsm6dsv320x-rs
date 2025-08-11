#![no_std]
#![no_main]
#![deny(unsafe_code)]

use core::fmt::Write;
use cortex_m_rt::entry;
use lsm6dsv320x_rs::prelude::*;
use lsm6dsv320x_rs::*;
use panic_halt as _;
use stm32f4xx_hal::{
    i2c::{DutyCycle, I2c, Mode},
    pac,
    prelude::*,
    serial::{config::Config, Serial},
};

const CNT_FOR_OUTPUT: u8 = 100;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(48.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);
    let tim1 = dp.TIM1.delay_us(&clocks);

    let gpiob = dp.GPIOB.split();
    let gpioa = dp.GPIOA.split();

    let scl = gpiob.pb8.into_alternate().set_open_drain();
    let sda = gpiob.pb9.into_alternate().set_open_drain();

    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        &clocks,
    );

    let tx_pin = gpioa.pa2.into_alternate();

    let mut tx = Serial::tx(
        dp.USART2,
        tx_pin,
        Config::default().baudrate(115_200.bps()),
        &clocks,
    )
    .unwrap();

    delay.delay_ms(5);
    let mut sensor = Lsm6dsv320x::new_i2c(i2c, I2CAddress::I2cAddL, tim1);

    // Check device ID
    let id = sensor.device_id_get().unwrap();
    if id != lsm6dsv320x_rs::ID {
        writeln!(tx, "Unexpected device ID: {}", id).unwrap();
        loop {}
    }

    // Restore default configuration
    sensor.reset_set(Reset::RestoreCtrlRegs).unwrap();
    let mut rst: Reset = Reset::RestoreCtrlRegs;
    while rst != Reset::Ready {
        rst = sensor.reset_get().unwrap();
    }

    // Enable Block Data Update
    sensor.block_data_update_set(1).unwrap();

    // Set Output Data Rate.
    sensor.xl_data_rate_set(DataRate::_60hz).unwrap();
    sensor.hg_xl_data_rate_set(HgXlDataRate::_960hz, 1).unwrap();
    sensor.gy_data_rate_set(DataRate::_120hz).unwrap();

    // Set full scale
    sensor.xl_full_scale_set(XlFullScale::_2g).unwrap();
    sensor.hg_xl_full_scale_set(HgXlFullScale::_320g).unwrap();
    sensor.gy_full_scale_set(GyFullScale::_2000dps).unwrap();

    // Configure filtering chain
    let mut filt_settling_mask = FiltSettlingMask::default();
    filt_settling_mask.drdy = 1;
    filt_settling_mask.irq_xl = 1;
    filt_settling_mask.irq_g = 1;
    sensor.filt_settling_mask_set(filt_settling_mask).unwrap();
    sensor.filt_gy_lp1_set(1).unwrap();
    sensor
        .filt_gy_lp1_bandwidth_set(FiltLpBandwidth::UltraLight)
        .unwrap();
    sensor.filt_xl_lp2_set(1).unwrap();
    sensor
        .filt_xl_lp2_bandwidth_set(FiltLpBandwidth::Strong)
        .unwrap();

    let mut lowg_xl_sum = [0f32; 3];
    let mut lowg_xl_cnt = 0;
    let mut hg_xl_sum = [0f32; 3];
    let mut hg_xl_cnt = 0;
    let mut gyro_sum = [0f32; 3];
    let mut gyro_cnt = 0;
    let mut temp_sum = 0.0f32;
    let mut temp_cnt = 0;

    // Read samples in polling mode (no int)
    loop {
        // Read output only if new xl value is available
        let drdy = sensor.flag_data_ready_get().unwrap();

        if drdy.drdy_xl == 1 {
            // Read acceleration data
            let data_raw_acceleration = sensor.acceleration_raw_get().unwrap();

            for i in 0..3 {
                let acceleration_mg = lsm6dsv320x_rs::from_fs2_to_mg(data_raw_acceleration[i]);
                lowg_xl_sum[i] += acceleration_mg;
            }
            lowg_xl_cnt += 1;
        }

        if drdy.drdy_hgxl == 1 {
            // Read acceleration field data
            let data_raw_motion = sensor.hg_acceleration_raw_get().unwrap();

            for i in 0..3 {
                let acceleration_mg = lsm6dsv320x_rs::from_fs320_to_mg(data_raw_motion[i]);
                hg_xl_sum[i] += acceleration_mg;
            }
            hg_xl_cnt += 1;
        }

        // Read output only if new gyroscope value is available
        if drdy.drdy_gy == 1 {
            // Read angular rate data
            let data_raw_angular_rate = sensor.angular_rate_raw_get().unwrap();

            for i in 0..3 {
                let angular_rate_mdps =
                    lsm6dsv320x_rs::from_fs2000_to_mdps(data_raw_angular_rate[i]);
                gyro_sum[i] += angular_rate_mdps;
            }
            gyro_cnt += 1;
        }

        if drdy.drdy_temp == 1 {
            // Read temperature data
            let data_raw_temperature = sensor.temperature_raw_get().unwrap();
            let temperature_deg_c = lsm6dsv320x_rs::from_lsb_to_celsius(data_raw_temperature);

            temp_sum += temperature_deg_c;
            temp_cnt += 1;
        }

        if lowg_xl_cnt >= CNT_FOR_OUTPUT {
            // Print media low-g xl data
            let acceleration_mg = [
                lowg_xl_sum[0] as f32 / lowg_xl_cnt as f32,
                lowg_xl_sum[1] as f32 / lowg_xl_cnt as f32,
                lowg_xl_sum[2] as f32 / lowg_xl_cnt as f32,
            ];

            writeln!(
                tx,
                "lg xl (media of {} samples) [mg]: {:.2}\t{:.2}\t{:.2}",
                lowg_xl_cnt, acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();

            lowg_xl_sum = [0.0; 3];
            lowg_xl_cnt = 0;

            // Print media high-g xl data
            let acceleration_mg = [
                hg_xl_sum[0] as f32 / hg_xl_cnt as f32,
                hg_xl_sum[1] as f32 / hg_xl_cnt as f32,
                hg_xl_sum[2] as f32 / hg_xl_cnt as f32,
            ];

            writeln!(
                tx,
                "hg xl (media of {} samples) [mg]: {:.2}\t{:.2}\t{:.2}",
                hg_xl_cnt, acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();

            hg_xl_sum = [0.0; 3];
            hg_xl_cnt = 0;

            // Print media gyro data
            let angular_rate_mdps = [
                gyro_sum[0] as f32 / gyro_cnt as f32,
                gyro_sum[1] as f32 / gyro_cnt as f32,
                gyro_sum[2] as f32 / gyro_cnt as f32,
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
            temp_cnt = 0;
            temp_sum = 0.0;
        }
    }
}
