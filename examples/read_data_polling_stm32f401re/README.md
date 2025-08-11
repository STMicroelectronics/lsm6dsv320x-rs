# LSM6DSV320X Sensor Data Acquisition on STM32F401RE with IKS4A1 Shield

This project demonstrates how to interface the **LSM6DSV320X** inertial measurement unit (IMU) sensor with an **STM32F401RE** microcontroller board using the **IKS4A1** sensor shield. The program reads accelerometer, gyroscope, and temperature data from the sensor via I2C, processes the data, and outputs averaged sensor readings over UART.

The code is written in Rust using the `stm32f4xx-hal` hardware abstraction layer and the `lsm6dsv320x` sensor driver crate.

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

The IKS4A1 shield is mounted on top of the STM32F401RE Nucleo board, providing the LSM6DSV320X sensor connected to the microcontroller's I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). The UART output is routed through PA2 to allow serial communication with a host PC or terminal.

---

## Code Description

### Initialization

- The program initializes the microcontroller peripherals, including clocks, GPIO pins, I2C, and UART.
- The I2C bus is configured for 400 kHz operation with open-drain pins PB8 and PB9.
- UART is configured on PA2 at 115200 baud for serial output.
- A delay timer and TIM1 timer are initialized for sensor timing requirements.

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

### Data Acquisition Loop

- The program continuously polls the sensor for new data availability flags.
- When new data is ready, raw sensor values for acceleration, angular rate, and temperature are read.
- Raw values are converted to physical units (mg for acceleration, mdps for gyroscope, °C for temperature).
- Data samples are accumulated and averaged over 100 samples.
- Averaged sensor readings are printed over UART for monitoring.

---

## Usage

1. Connect the IKS4A1 shield to the STM32F401RE Nucleo board.
2. Flash the compiled Rust firmware onto the STM32F401RE.
3. Open a serial terminal at 115200 baud on the UART port.
4. Observe periodic output of averaged accelerometer, gyroscope, and temperature data.

---

## Notes

- The program uses polling mode for simplicity; no interrupts are configured.
- The sensor driver handles low-level register access and conversions.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic (`panic_halt`).

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IKS4A1 Sensor Shield](https://www.st.com/en/ecosystems/x-nucleo-iks4a1.html)
- [LSM6DSV320X Datasheet](https://www.st.com/resource/en/datasheet/lsm6dsv320x.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README provides a detailed explanation of the embedded Rust program for sensor data acquisition on STM32F401RE with the IKS4A1 shield.*