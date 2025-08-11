#![no_std]
#![no_main]

use core::fmt::Write;
use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, Config};
use embassy_stm32::dma::NoDma;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::i2c::{self, Config as I2cConfig, I2c};
use embassy_stm32::peripherals::{self, USART2};
use embassy_stm32::time::{khz, mhz};
use embassy_stm32::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllQDiv, PllSource,
    Sysclk,
};
use embassy_stm32::usart::{
    BufferedInterruptHandler, Config as UsartConfig, DataBits, Parity, UartTx,
};
use embassy_time::Delay;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

use lsm6dsv320x_rs::prelude::*;
use lsm6dsv320x_rs::*;

const FIFO_WATERMARK: u8 = 128;

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

    let i2c: I2c<_> = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        NoDma,
        NoDma,
        khz(100),
        I2cConfig::default(),
    );

    let mut delay = Delay;

    // Configure the interrupt pin (if needed) and obtain handler.
    // On the Nucleo FR401 the interrupt pin is connected to pin PC0.
    let interrupt = Input::new(p.PC0, Pull::None);
    let mut interrupt = ExtiInput::new(interrupt, p.EXTI0);

    delay.delay_ms(5_u32);

    let mut msg: String<64> = String::new();

    let mut sensor = Lsm6dsv320x::new_i2c(i2c, I2CAddress::I2cAddL, delay.clone());

    // Check device ID
    let id = sensor.device_id_get().unwrap();
    if id != lsm6dsv320x_rs::ID {
        writeln!(&mut msg, "Unexpected device ID: {}", id).unwrap();
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

    // Set FIFO watermark (numer of unread sensor data TAG + 6 bytes
    // stored in FIFO) to FIFO_WATERMARK samples
    sensor.fifo_watermark_set(FIFO_WATERMARK).unwrap();

    // Set FIFO batch XL/Gyro ODR.
    sensor.fifo_xl_batch_set(FifoBatch::_60hz).unwrap();
    sensor.fifo_hg_xl_batch_set(1).unwrap();
    sensor.fifo_gy_batch_set(FifoBatch::_120hz).unwrap();
    // Set FIFO mode to Stream mode (aka Continuous Mode)
    sensor.fifo_mode_set(FifoMode::Stream).unwrap();
    sensor
        .fifo_timestamp_batch_set(FifoTimestampBatch::Dec32)
        .unwrap();
    sensor.timestamp_set(1).unwrap();

    // Set Output Data Rate
    // Selected data rate have to be equal or greater with respect with MLC
    // data rate.
    sensor.xl_data_rate_set(DataRate::_60hz).unwrap();
    sensor.hg_xl_data_rate_set(HgXlDataRate::_480hz, 0).unwrap();
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

    // enable fifo_th on High-G XL (sensor at highest frequency)
    let mut pin_int = PinIntRoute::default();
    pin_int.fifo_th = 1;
    sensor.pin_int1_route_set(&pin_int).unwrap();
    //sensor.pin_int2_route_set(&pin_int);

    let mut lowg_xl_sum = [0f64; 3];
    let mut lowg_xl_cnt = 0;
    let mut hg_xl_sum = [0f64; 3];
    let mut hg_xl_cnt = 0;
    let mut gyro_sum = [0f64; 3];
    let mut gyro_cnt = 0;

    loop {
        interrupt.wait_for_high().await;

        let fifo_status = sensor.fifo_status_get().unwrap();
        let num = fifo_status.fifo_level;

        for _ in 0..num {
            // Read FIFO sensor value
            let f_data = sensor.fifo_out_raw_get().unwrap();

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
                        let acc_mg = lsm6dsv320x_rs::from_fs2_to_mg(data);
                        lowg_xl_sum[index] += acc_mg as f64;
                    }
                    lowg_xl_cnt += 1;
                }
                Tag::XlHg => {
                    for (index, &data) in raw_array_data.iter().enumerate() {
                        let acc_mg = lsm6dsv320x_rs::from_fs320_to_mg(data);
                        hg_xl_sum[index] += acc_mg as f64;
                    }
                    hg_xl_cnt += 1;
                }
                Tag::GyNc => {
                    for (index, &data) in raw_array_data.iter().enumerate() {
                        let angular_rate_mdps = lsm6dsv320x_rs::from_fs2000_to_mdps(data);
                        gyro_sum[index] += angular_rate_mdps as f64;
                    }
                    gyro_cnt += 1;
                }
                Tag::Timestamp => {
                    writeln!(&mut msg, "TIMESTAMP [ms] {}", ts).unwrap();
                    let _ = tx.blocking_write(msg.as_bytes());
                    msg.clear();

                    // Print media low-g xl data
                    if lowg_xl_cnt > 0 {
                        let acc_mg = [
                            lowg_xl_sum[0] / lowg_xl_cnt as f64,
                            lowg_xl_sum[1] / lowg_xl_cnt as f64,
                            lowg_xl_sum[2] / lowg_xl_cnt as f64,
                        ];

                        writeln!(
                            &mut msg,
                            "lg xl (media of {} samples) [mg]:{:.2}\t{:.2}\t{:.2}",
                            lowg_xl_cnt, acc_mg[0], acc_mg[1], acc_mg[2]
                        )
                        .unwrap();
                        let _ = tx.blocking_write(msg.as_bytes());
                        msg.clear();

                        lowg_xl_sum = [0.0; 3];
                        lowg_xl_cnt = 0;
                    }

                    // print media high-g xl data
                    if hg_xl_cnt > 0 {
                        let acc_mg = [
                            hg_xl_sum[0] / hg_xl_cnt as f64,
                            hg_xl_sum[1] / hg_xl_cnt as f64,
                            hg_xl_sum[2] / hg_xl_cnt as f64,
                        ];

                        writeln!(
                            &mut msg,
                            "hg xl (media of {} samples) [mg]:{:.2}\t{:.2}\t{:.2}",
                            hg_xl_cnt, acc_mg[0], acc_mg[1], acc_mg[2]
                        )
                        .unwrap();
                        let _ = tx.blocking_write(msg.as_bytes());
                        msg.clear();

                        hg_xl_sum = [0.0; 3];
                        hg_xl_cnt = 0;
                    }

                    // print media gyro data
                    if gyro_cnt > 0 {
                        let angular_rate_mdps = [
                            gyro_sum[0] / gyro_cnt as f64,
                            gyro_sum[1] / gyro_cnt as f64,
                            gyro_sum[2] / gyro_cnt as f64,
                        ];

                        writeln!(
                            &mut msg,
                            "gyro (media of {} samples) [mdps]:{:.2}\t{:.2}\t{:.2}",
                            gyro_cnt,
                            angular_rate_mdps[0],
                            angular_rate_mdps[1],
                            angular_rate_mdps[2]
                        )
                        .unwrap();
                        let _ = tx.blocking_write(msg.as_bytes());
                        msg.clear();

                        gyro_sum = [0.0; 3];
                        gyro_cnt = 0;
                    }
                }
                _ => {
                    writeln!(&mut msg, "[0x{:x}] UNHANDLED TAG", f_data.tag as u8).unwrap();
                    let _ = tx.blocking_write(msg.as_bytes());
                    msg.clear();
                }
            }
        }
    }
}
