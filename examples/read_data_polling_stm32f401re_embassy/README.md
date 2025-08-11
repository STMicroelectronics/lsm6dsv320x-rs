# LSM6DSV320X Sensor Data Acquisition on STM32F401RE with IKS4A1 Shield using Embassy

## Overview

This example demonstrates interfacing the **LSM6DSV320X** inertial measurement unit (IMU) sensor with an **STM32F401RE** microcontroller board using the **IKS4A1** sensor shield, leveraging the **Embassy** async embedded framework in Rust. The program asynchronously reads accelerometer, gyroscope, and temperature data from the sensor over I2C, processes the data, and outputs averaged sensor readings via UART.

Embassy provides a modern async runtime for embedded systems, enabling efficient, non-blocking peripheral access and concurrency without an operating system.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor Shield:** IKS4A1 (STMicroelectronics sensor expansion board)
- **Sensor:** LSM6DSV320X IMU (accelerometer + gyroscope + temperature sensor)
- **Communication Interface:** I2C (400 kHz Standard Mode)
- **UART:** USART2 for serial output at 115200 baud

### Default Pin Configuration

| Signal       | STM32F401RE Pin | IKS4A1 Shield Pin | Description                  |
|--------------|-----------------|-------------------|------------------------------|
| I2C1_SCL     | PB8             | SCL               | I2C clock line (open-drain)  |
| I2C1_SDA     | PB9             | SDA               | I2C data line (open-drain)   |
| USART2_TX    | PA2             | UART TX           | UART transmit for debug output|

The IKS4A1 shield is mounted on the STM32F401RE Nucleo board, connecting the LSM6DSV320X sensor to the microcontroller's I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). UART output is routed through PA2 for serial communication.

---

## Code Description

### Embassy Async Runtime

- The example uses Embassy's async runtime to manage concurrency and peripheral access without blocking.
- Peripherals such as I2C and UART are wrapped in Embassy drivers that provide async interfaces.
- The `#[embassy::main]` macro initializes the async executor and sets up the environment.

### Initialization

- The microcontroller clocks and GPIO pins are configured similarly to the synchronous example.
- I2C1 is initialized with Embassy's I2C driver at 400 kHz using open-drain pins PB8 and PB9.
- UART TX is configured on PA2 with Embassy's serial driver at 115200 baud.
- Embassy's timer or delay abstractions are used for timing and sensor requirements.

### Sensor Setup

- The LSM6DSV320X sensor is initialized over I2C.
- The device ID is checked to ensure the sensor is connected and responding.
- The sensor is reset to default configuration.
- Block Data Update (BDU) is enabled to ensure data consistency during reads.
- Output data rates are configured:
  - Low-g accelerometer: 60 Hz
  - High-g accelerometer: 960 Hz
  - Gyroscope: 120 Hz
- Full scale ranges are set:
  - Low-g accelerometer: ±2g
  - High-g accelerometer: ±320g
  - Gyroscope: ±2000 dps
- Filtering options are configured to optimize sensor data quality.

### Async Data Acquisition Loop

- The main async task continuously polls the sensor for new data availability flags.
- When new data is ready, raw sensor values for acceleration, angular rate, and temperature are read asynchronously.
- Raw values are converted to physical units (mg for acceleration, mdps for gyroscope, °C for temperature).
- Data samples are accumulated and averaged over a defined number of samples.
- Averaged sensor readings are sent asynchronously over UART for monitoring.

---

## Usage

1. Connect the IKS4A1 shield to the STM32F401RE Nucleo board.
2. Flash the Embassy-based Rust firmware onto the STM32F401RE.
3. Open a serial terminal at 115200 baud on the UART port.
4. Observe periodic output of averaged accelerometer, gyroscope, and temperature data.

---

## Notes

- All the write are made blocking for simplicity: providing a DMA they could become async.

---

## References

- [Embassy Embedded Framework](https://embassy.dev/)
- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IKS4A1 Sensor Shield](https://www.st.com/en/ecosystems/x-nucleo-iks4a1.html)
- [LSM6DSV320X Datasheet](https://www.st.com/resource/en/datasheet/lsm6dsv320x.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---