# lsm6dsv320x-rs
[![Crates.io][crates-badge]][crates-url]
[![BSD 3-Clause licensed][bsd-badge]][bsd-url]

[crates-badge]: https://img.shields.io/crates/v/lsm6dsv320x-rs
[crates-url]: https://crates.io/crates/lsm6dsv320x-rs
[bsd-badge]: https://img.shields.io/crates/l/lsm6dsv320x-rs
[bsd-url]: https://opensource.org/licenses/BSD-3-Clause

Provides a platform-agnostic, no_std-compatible driver for the ST LSM6DSV320X IMU, supporting both I2C and SPI communication interfaces.

## Sensor Overview

The LSM6DSV320X is a high-end, low-noise, low-power, small IMU, featuring a 3-axis digital low-g accelerometer at 16 g, a 3-axis digital high-g accelerometer at 320 g, and a 3-axis digital gyroscope, which offers the best IMU sensor with a quad-channel architecture for processing acceleration and angular rate data on four separate channels (user interface, OIS, EIS, and high-g accelerometer data) with dedicated configuration, processing, and filtering along with a dedicated high-g sensor for high-g shock and car crash detection.

The device enables edge computing, leveraging embedded advanced dedicated features such as a finite state machine (FSM) for configurable motion tracking and a machine learning core (MLC) for context awareness with exportable AI features for IoT applications.

The LSM6DSV320X supports the adaptive self-configuration (ASC) feature, which allows automatically reconfiguring the device in real time based on the detection of a specific motion pattern or based on the output of a specific decision tree configured in the MLC, without any intervention from the host processor.

The device embeds a dedicated accelerometer sensor with an independent channel and filtering for high-g acceleration detection that perfectly matches shock requirements for sports, concussion detection, shock detection, and a full range of car crash detection applications.

For more info, please visit the device page at [https://www.st.com/en/mems-and-sensors/lsm6dsv320x.html](https://www.st.com/en/mems-and-sensors/lsm6dsv320x.html)

## Installation

Add the driver to your `Cargo.toml` dependencies:

```toml
[dependencies]
lsm6dsv320x-rs = "0.1.0"
```

Or, add it directly from the terminal:

```sh
cargo add lsm6dsv320x-rs
```

## Usage

Include the crate and its prelude
```rust
use lsm6dsv320x_rs as lsm6dsv320x;
use lsm6dsv320x::*;
use lsm6dsv320x::prelude::*;
```

### Create an instance

Create an instance of the driver with the `new_<bus>` associated function, by passing an I2C (`embedded_hal::i2c::I2c`) instance and I2C address, or an SPI (`embedded_hal::spi::SpiDevice`) instance, along with a timing peripheral.

An example with I2C:

```rust
let mut sensor = Lsm6dsv320x::new_i2c(i2c, I2CAddress::I2cAddL, delay);
```

### Check "Who Am I" Register

This step ensures correct communication with the sensor. It returns a unique ID to verify the sensor's identity.

```rust
let whoami = sensor.device_id_get().unwrap();
if whoami != ID {
    panic!("Invalid sensor ID");
}
```

### Configure

See details in specific examples; the following are common api calls:

```rust
// Restore default configuration
sensor.reset_set(Reset::RestoreCtrlRegs).unwrap();
let mut rst: Reset = Reset::RestoreCtrlRegs;
while rst != Reset::Ready {
    rst = sensor.reset_get().unwrap();
}

// Enable Block Data Update
sensor.block_data_update_set(1).unwrap();

// Set Output Data Rate
sensor.xl_data_rate_set(DataRate::_1920hz).unwrap();
sensor.hg_xl_data_rate_set(HgXlDataRate::_960hz, 1).unwrap();
sensor.gy_data_rate_set(DataRate::_120hz).unwrap();

// Set full scale
sensor.xl_full_scale_set(XlFullScale::_2g).unwrap();
sensor.hg_xl_full_scale_set(HgXlFullScale::_320g).unwrap();
sensor.gy_full_scale_set(GyFullScale::_2000dps).unwrap();
```

## License

Distributed under the BSD-3 Clause license.

More Information: [http://www.st.com](http://st.com/MEMS).

**Copyright (C) 2025 STMicroelectronics**