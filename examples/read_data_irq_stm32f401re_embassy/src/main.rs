#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::i2c::{self, Config as I2cConfig, I2c};
use embassy_stm32::peripherals::{self, USART2};
use embassy_stm32::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllQDiv, PllSource,
    Sysclk,
};
use embassy_stm32::time::{khz, mhz};
use embassy_stm32::usart::{
    BufferedInterruptHandler, Config as UsartConfig, DataBits, Parity, UartTx,
};
use embassy_stm32::{bind_interrupts, Config};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};
static INTERRUPT_SIGNAL: Signal<ThreadModeRawMutex, ()> = Signal::new();

use lsm6dsv320x_rs::prelude::*;
use lsm6dsv320x_rs::*;

const CNT_FOR_OUTPUT: u8 = 100;

#[defmt::panic_handler]
fn panic() -> ! {
    core::panic!("panic via `defmt::panic!`")
}

bind_interrupts!(struct Irqs {
    USART2 => BufferedInterruptHandler<USART2>;
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    config.rcc.hse = Some(Hse {
        freq: mhz(8),
        mode: HseMode::Bypass,
    });
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.pll_src = PllSource::HSI;
    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV16,
        mul: PllMul::MUL336,
        divp: Some(PllPDiv::DIV4),
        divq: Some(PllQDiv::DIV7),
        divr: None,
    });
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV2;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.hsi = true;

    let p = embassy_stm32::init(config);

    let mut usart_config: UsartConfig = UsartConfig::default();
    usart_config.baudrate = 115200;
    usart_config.data_bits = DataBits::DataBits8;
    usart_config.parity = Parity::ParityNone;

    let mut tx: UartTx<_> = UartTx::new(p.USART2, p.PA2, NoDma, usart_config).unwrap();
    let mut msg: String<1000> = String::new();

    let mut i2c_config = I2cConfig::default();
    i2c_config.sda_pullup = false;
    i2c_config.scl_pullup = false;

    let i2c: I2c<_> = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        khz(100),
        i2c_config,
    );

    let mut delay = Delay;

    delay.delay_ms(10_u32);

    // Configure the interrupt pin (if needed) and obtain handler.
    // On the Nucleo FR401 the interrupt pin is connected to pin PB0.
    let interrupt = Input::new(p.PC0, Pull::None);
    let interrupt = ExtiInput::new(interrupt, p.EXTI0);

    _spawner.spawn(handle_interrupt(interrupt)).unwrap();

    let mut sensor = Lsm6dsv320x::new_i2c(i2c, I2CAddress::I2cAddL, delay.clone());

    // Check device ID
    let whoami = sensor.device_id_get().unwrap();
    if whoami != ID {
        writeln!(&mut msg, "Device ID mismatch: {:#02x}", whoami).unwrap();
        let _ = tx.blocking_write(msg.as_bytes());
        msg.clear();
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

    // Set Otuput Data Rate
    sensor.xl_data_rate_set(DataRate::_1920hz).unwrap();
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

    // Enable interrupt on High-G XL (sensor at highest frequency)
    let mut pin_int = PinIntRoute::default();
    pin_int.drdy_hg_xl = 1;
    sensor.pin_int1_route_hg_set(&pin_int).unwrap();
    //sensor.pint_int2_route_hg_set(&pin_int).unwrap();

    let mut lowg_xl_sum = [0f64; 3];
    let mut lowg_xl_cnt = 0;
    let mut hg_xl_sum = [0f64; 3];
    let mut hg_xl_cnt = 0;
    let mut gyro_sum = [0f64; 3];
    let mut gyro_cnt = 0;
    let mut temp_sum = 0.0f64;
    let mut temp_cnt = 0;

    loop {
        INTERRUPT_SIGNAL.wait().await;

        let status = sensor.flag_data_ready_get().unwrap();

        // Read output only if new xl value is available
        if status.drdy_xl == 1 {
            // Read acceleration data
            let data_raw_acceleration = sensor.acceleration_raw_get().unwrap();

            for i in 0..3 {
                let acceleration_mg = lsm6dsv320x_rs::from_fs2_to_mg(data_raw_acceleration[i]);
                lowg_xl_sum[i] += acceleration_mg as f64;
            }
            lowg_xl_cnt += 1;
        }

        if status.drdy_hgxl == 1 {
            // Read acceleration field data
            let data_raw_motion = sensor.hg_acceleration_raw_get().unwrap();

            for i in 0..3 {
                let acceleration_mg = lsm6dsv320x_rs::from_fs320_to_mg(data_raw_motion[i]);
                hg_xl_sum[i] += acceleration_mg as f64;
            }
            hg_xl_cnt += 1;
        }

        // Read output only if new gyroscope value is available
        if status.drdy_gy == 1 {
            // Read angular rate data
            let data_raw_angular_rate = sensor.angular_rate_raw_get().unwrap();
            for i in 0..3 {
                let angular_rate_mdps =
                    lsm6dsv320x_rs::from_fs2000_to_mdps(data_raw_angular_rate[i]);
                gyro_sum[i] += angular_rate_mdps as f64;
            }
            gyro_cnt += 1;
        }

        if status.drdy_temp == 1 {
            // Read temperature data
            let data_raw_temperature = sensor.temperature_raw_get().unwrap();
            let temperature_deg_c = lsm6dsv320x_rs::from_lsb_to_celsius(data_raw_temperature);

            temp_sum += temperature_deg_c as f64;
            temp_cnt += 1;
        }

        if lowg_xl_cnt >= CNT_FOR_OUTPUT {
            // Print media low-g xl data
            let acceleration_mg = [
                lowg_xl_sum[0] / lowg_xl_cnt as f64,
                lowg_xl_sum[1] / lowg_xl_cnt as f64,
                lowg_xl_sum[2] / lowg_xl_cnt as f64,
            ];

            writeln!(
                &mut msg,
                "lg xl (media of {} samples) [mg]: {:.2}\t{:.2}\t{:.2}",
                lowg_xl_cnt, acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();

            lowg_xl_sum = [0.0; 3];
            lowg_xl_cnt = 0;

            // Print media high-g xl data
            let acceleration_mg = [
                hg_xl_sum[0] / hg_xl_cnt as f64,
                hg_xl_sum[1] / hg_xl_cnt as f64,
                hg_xl_sum[2] / hg_xl_cnt as f64,
            ];

            writeln!(
                &mut msg,
                "hg xl (media of {} samples) [mg]: {:.2}\t{:.2}\t{:.2}",
                hg_xl_cnt, acceleration_mg[0], acceleration_mg[1], acceleration_mg[2]
            )
            .unwrap();

            hg_xl_sum = [0.0; 3];
            hg_xl_cnt = 0;

            // Print media gyro data
            let angular_rate_mdps = [
                gyro_sum[0] / gyro_cnt as f64,
                gyro_sum[1] / gyro_cnt as f64,
                gyro_sum[2] / gyro_cnt as f64,
            ];

            writeln!(
                &mut msg,
                "gyro (media of {} samples) [mdps]: {:.2}\t{:.2}\t{:.2}",
                gyro_cnt, angular_rate_mdps[0], angular_rate_mdps[1], angular_rate_mdps[2]
            )
            .unwrap();

            gyro_sum = [0.0; 3];
            gyro_cnt = 0;

            // Print media temperature data
            let temperature_deg_c = temp_sum / temp_cnt as f64;
            writeln!(
                &mut msg,
                "Temperature (media of {} samples) [degC]:{:.2}",
                temp_cnt, temperature_deg_c
            )
            .unwrap();
            let _ = tx.blocking_write(msg.as_bytes());
            msg.clear();
            temp_cnt = 0;
            temp_sum = 0.0;
        }
    }
}

#[embassy_executor::task]
async fn handle_interrupt(mut int_pin: ExtiInput<'static, embassy_stm32::peripherals::PC0>) {
    loop {
        INTERRUPT_SIGNAL.signal(());
        int_pin.wait_for_rising_edge().await;
    }
}
