#![no_std]
#![no_main]

use core::fmt::Write;
use embassy_executor::Spawner;
use embassy_stm32::bind_interrupts;
use embassy_stm32::dma::NoDma;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::i2c::{self, Config as I2cConfig, I2c};
use embassy_stm32::peripherals::{self, USART2};
use embassy_stm32::time::khz;
use embassy_stm32::usart::{
    BufferedInterruptHandler, Config as UsartConfig, DataBits, Parity, UartTx,
};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::Delay;
use embedded_hal::delay::DelayNs;
use heapless::String;
use {defmt_rtt as _, panic_probe as _};
static INTERRUPT_SIGNAL: Signal<ThreadModeRawMutex, ()> = Signal::new();

use lsm6dsv320x_rs::prelude::*;
use lsm6dsv320x_rs::*;

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
    let p = embassy_stm32::init(Default::default());

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
    let interrupt = ExtiInput::new(interrupt, p.EXTI0);

    delay.delay_ms(5_u32);

    let mut msg: String<64> = String::new();

    let mut sensor = Lsm6dsv320x::new_i2c(i2c, I2CAddress::I2cAddL, delay.clone());

    // Check device ID
    let whoami = sensor.device_id_get().unwrap();
    if whoami != ID {
        writeln!(&mut msg, "Device ID mismatch: {:#02x}", whoami).unwrap();
        let _ = tx.blocking_write(msg.as_bytes());
        msg.clear();
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
    sensor.hg_xl_data_rate_set(HgXlDataRate::_960hz, 1).unwrap();

    // Set full scale
    sensor.hg_xl_full_scale_set(HgXlFullScale::_256g).unwrap();

    // Configure filtering chain
    let mut filt_settling_mask = FiltSettlingMask::default();
    filt_settling_mask.drdy = 1;
    filt_settling_mask.irq_xl = 1;
    filt_settling_mask.irq_g = 1;

    sensor.filt_settling_mask_set(filt_settling_mask).unwrap();
    sensor.filt_xl_lp2_set(1).unwrap();
    sensor
        .filt_xl_lp2_bandwidth_set(FiltLpBandwidth::Strong)
        .unwrap();

    let mut wakeup_cfg = HgWakeUpCfg::default();
    wakeup_cfg.hg_shock_dur = 1;
    wakeup_cfg.hg_wakeup_ths = 4;
    sensor.hg_wake_up_cfg_set(wakeup_cfg).unwrap();

    // Enable interrupt on High-G XL (sensor at highest frequency)
    let mut pin_int = PinIntRoute::default();
    pin_int.hg_wakeup = 1;
    sensor.pin_int1_route_hg_set(&pin_int).unwrap();
    //sensor.pint_int2_route_hg_set(&pin_int).unwrap();

    let mut int_cfg = HgWuInterruptCfg::default();
    int_cfg.hg_interrupts_enable = 1;
    sensor.hg_wu_interrupt_cfg_set(int_cfg).unwrap();

    _spawner.spawn(handle_interrupt(interrupt)).unwrap();

    loop {
        INTERRUPT_SIGNAL.wait().await;

        let status = sensor.hg_event_get().unwrap();

        if status.hg_event == 1 {
            let axis_x = status.hg_wakeup_x;
            let axis_y = status.hg_wakeup_y;
            let axis_z = status.hg_wakeup_z;

            writeln!(
                &mut msg,
                "WAKEUP event on X: {}, Y: {}, Z: {}",
                axis_x, axis_y, axis_z
            )
            .unwrap();
            let _ = tx.blocking_write(msg.as_bytes());
            msg.clear();
        }
    }
}

#[embassy_executor::task]
async fn handle_interrupt(mut int_pin: ExtiInput<'static, embassy_stm32::peripherals::PC0>) {
    loop {
        int_pin.wait_for_rising_edge().await;
        INTERRUPT_SIGNAL.signal(());
    }
}
