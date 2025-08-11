#![no_std]
#![no_main]

use panic_halt as _;

use core::cell::RefCell;
use core::fmt::Write;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use lsm6dsv320x_rs::prelude::*;
use lsm6dsv320x_rs::*;
use stm32f4xx_hal::{
    gpio::{self, Edge, Input},
    i2c::{DutyCycle, I2c, Mode},
    pac::{self, interrupt},
    prelude::*,
    serial::{config::Config, Serial},
};
type IntPin = gpio::PC0<Input>;

const FIFO_WATERMARK: u8 = 128;

static INT_PIN: Mutex<RefCell<Option<IntPin>>> = Mutex::new(RefCell::new(None));
static MEMS_EVENT: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

#[entry]
fn main() -> ! {
    let mut dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(48.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);
    let tim1 = dp.TIM1.delay_us(&clocks);

    let gpiob = dp.GPIOB.split();
    let gpioa = dp.GPIOA.split();
    let gpioc = dp.GPIOC.split();

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

    let mut int_pin = gpioc.pc0.into_input();
    // Configure Pin for Interrupts
    // 1) Promote SYSCFG structure to HAL to be able to configure interrupts
    let mut syscfg = dp.SYSCFG.constrain();
    // 2) Make an interrupt source
    int_pin.make_interrupt_source(&mut syscfg);
    // 3) Make an interrupt source
    int_pin.trigger_on_edge(&mut dp.EXTI, Edge::Rising);
    // 4) Enable gpio interrupt
    int_pin.enable_interrupt(&mut dp.EXTI);

    // Enable the external interrupt in the NVIC by passing the interrupt number
    unsafe {
        cortex_m::peripheral::NVIC::unmask(int_pin.interrupt());
    }

    // Now that pin is configured, move pin into global context
    cortex_m::interrupt::free(|cs| {
        INT_PIN.borrow(cs).replace(Some(int_pin));
    });

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

    let mut lowg_xl_sum = [0f32; 3];
    let mut lowg_xl_cnt = 0;
    let mut hg_xl_sum = [0f32; 3];
    let mut hg_xl_cnt = 0;
    let mut gyro_sum = [0f32; 3];
    let mut gyro_cnt = 0;

    loop {
        // Wait for interrupt
        let mems_event = cortex_m::interrupt::free(|cs| {
            let flag = *MEMS_EVENT.borrow(cs).borrow();
            if flag {
                MEMS_EVENT.borrow(cs).replace(false);
            }
            flag
        });
        if !mems_event {
            continue;
        }
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
                        lowg_xl_sum[index] += acc_mg;
                    }
                    lowg_xl_cnt += 1;
                }
                Tag::XlHg => {
                    for (index, &data) in raw_array_data.iter().enumerate() {
                        let acc_mg = lsm6dsv320x_rs::from_fs320_to_mg(data);
                        hg_xl_sum[index] += acc_mg;
                    }
                    hg_xl_cnt += 1;
                }
                Tag::GyNc => {
                    for (index, &data) in raw_array_data.iter().enumerate() {
                        let angular_rate_mdps = lsm6dsv320x_rs::from_fs2000_to_mdps(data);
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

#[interrupt]
fn EXTI0() {
    // Start a Critical Section
    cortex_m::interrupt::free(|cs| {
        // Obtain access to Peripheral and Clear Interrupt Pending Flag
        let mut int_pin = INT_PIN.borrow(cs).borrow_mut();
        if int_pin.as_mut().unwrap().check_interrupt() {
            int_pin.as_mut().unwrap().clear_interrupt_pending_bit();
        }
        MEMS_EVENT.borrow(cs).replace(true);
    });
}
