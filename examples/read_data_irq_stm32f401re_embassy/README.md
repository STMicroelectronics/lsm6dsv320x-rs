# LSM6DSV320X Sensor Data Streaming on STM32F401RE Nucleo-64

This example demonstrates continuous data acquisition from the **LSM6DSV320X** inertial measurement unit (IMU) sensor on an **STM32F401RE** Nucleo-64 board. It reads accelerometer (low-g and high-g), gyroscope, and temperature data via I2C, averages samples, and outputs the results over UART.

The sensor is configured with different output data rates and full scales for each sensor type, and filtering is applied to improve data quality. Interrupts signal when new data is available, enabling efficient data reading.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** LSM6DSV320X IMU (accelerometer + gyroscope + temperature)
- **Communication Interface:** I2C1 at 400 kHz Fast Mode (PB8 = SCL, PB9 = SDA)
- **UART:** USART2 (PA2 TX) at 115200 baud for serial output
- **Interrupt Pin:** PC0 configured as input with external interrupt for data ready signaling

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI0 (INT)  | PC0             | External interrupt from sensor data ready signal |

The LSM6DSV320X sensor is connected via I2C1 on PB8/PB9. The sensor's data ready interrupt line is connected to PC0, configured to trigger an external interrupt on rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals including clocks, GPIO pins, I2C, and UART.
- I2C1 is configured at 400 kHz Fast Mode on pins PB8 (SCL) and PB9 (SDA).
- UART is configured on USART2 (PA2) at 115200 baud, 8 data bits, no parity.
- PC0 is configured as an input pin with no pull resistor and set up as an external interrupt line (EXTI0).
- A delay abstraction is created for timing operations.

### Sensor Configuration

- The LSM6DSV320X sensor is initialized over I2C with the low I2C address.
- The device ID is read and verified; if mismatched, an error message is sent over UART and the program halts.
- The sensor is reset to default configuration and the program waits until reset completes.
- Block Data Update is enabled to ensure consistent sensor data reads.
- Output data rates are set individually:
  - Accelerometer (low-g): 60 Hz
  - Accelerometer (high-g): 960 Hz
  - Gyroscope: 120 Hz
- Full scale ranges are configured:
  - Accelerometer (low-g): ±2g
  - Accelerometer (high-g): ±320g
  - Gyroscope: ±2000 dps
- Filtering chain is configured with low-pass filters and settling masks for data ready and interrupts.
- Interrupt routing is configured to output high-g accelerometer data ready signals on INT1 pin (PC0).

### Data Acquisition Loop

- An asynchronous task waits for rising edge interrupts on PC0 signaling new data availability.
- When signaled, the main loop reads sensor status flags to determine which data is ready.
- For each ready sensor (low-g accel, high-g accel, gyro, temperature), raw data is read and converted to physical units.
- Data samples are accumulated and averaged over 100 samples.
- After accumulating enough samples, the program prints averaged values for each sensor type over UART.
- Accumulators and counters are reset after each output.

### Interrupt Handling

- The external interrupt on PC0 is handled asynchronously.
- On each rising edge, a signal is sent to the main loop to process new data.

---

## Usage

1. Connect the LSM6DSV320X sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's data ready interrupt output to PC0 on the STM32F401RE.
3. Build and flash the firmware onto the STM32F401RE board.
4. Open a serial terminal at 115200 baud on the USART2 TX line.
5. Observe averaged sensor data printed periodically over UART.

---

## Notes

- The example uses interrupt-driven data ready signaling combined with polling of sensor status flags.
- UART output uses blocking writes without DMA.
- The sensor is configured with different data rates and full scales for low-g and high-g accelerometers.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `defmt` and `panic_probe`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [LSM6DSV320X Datasheet](https://www.st.com/resource/en/datasheet/lsm6dsv320x.pdf)
- [Embassy STM32 HAL](https://docs.rs/embassy-stm32)

---

*This README explains the embedded Rust program for continuous sensor data streaming from the LSM6DSV320X on STM32F401RE using Embassy STM32 HAL.*
