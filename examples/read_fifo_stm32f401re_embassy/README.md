# LSM6DSV320X FIFO Data Streaming on STM32F401RE Nucleo-64

This example demonstrates how to stream sensor data from the **LSM6DSV320X** inertial measurement unit (IMU) using its FIFO buffer on an **STM32F401RE** Nucleo-64 board. The program configures the sensor to batch accelerometer (low-g and high-g) and gyroscope data into the FIFO, reads data when the FIFO watermark is reached, and outputs averaged sensor values and timestamps over UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** LSM6DSV320X IMU (accelerometer + gyroscope + FIFO support)
- **Communication Interface:** I2C1 at 400 kHz Fast Mode (PB8 = SCL, PB9 = SDA)
- **UART:** USART2 (PA2 TX) at 115200 baud for serial output
- **Interrupt Pin:** PC0 configured as input with external interrupt for FIFO watermark signaling

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI0 (INT)  | PC0             | External interrupt from FIFO watermark |

The LSM6DSV320X sensor is connected via I2C1 on PB8/PB9. The FIFO watermark interrupt line is connected to PC0, configured to trigger an external interrupt on rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The microcontroller peripherals are initialized, including clocks, GPIO pins, I2C, and UART.
- I2C1 is configured at 400 kHz Fast Mode on pins PB8 (SCL) and PB9 (SDA).
- UART is configured on USART2 (PA2) at 115200 baud, 8 data bits, no parity.
- PC0 is configured as an input pin with no pull resistor and set up as an external interrupt line (EXTI0).
- A delay abstraction is created for timing operations.

### Sensor Configuration

- The LSM6DSV320X sensor is initialized over I2C with the low I2C address.
- The device ID is read and verified; if mismatched, an error message is sent over UART and the program halts.
- The sensor is reset to default configuration and the program waits until reset completes.
- Block Data Update is enabled to ensure consistent sensor data reads.
- FIFO watermark is set to 128 samples.
- FIFO batching is enabled for:
  - Low-g accelerometer at 60 Hz
  - High-g accelerometer at 480 Hz
  - Gyroscope at 120 Hz
- FIFO mode is set to Stream (continuous) mode.
- Timestamp batching and timestamp generation are enabled.
- Output data rates are set to match or exceed MLC data rates:
  - Low-g accelerometer: 60 Hz
  - High-g accelerometer: 480 Hz
  - Gyroscope: 120 Hz
- Full scale ranges are configured:
  - Low-g accelerometer: ±2g
  - High-g accelerometer: ±320g
  - Gyroscope: ±2000 dps
- Filtering chain is configured with low-pass filters and settling masks for data ready and interrupts.
- FIFO watermark interrupt is enabled on INT1 (PC0).

### Data Acquisition Loop

- The program waits for a rising edge interrupt on PC0 indicating FIFO watermark reached.
- It reads the FIFO status to determine the number of samples available.
- For each sample, raw FIFO data is read and parsed by tag:
  - `XlNc` (low-g accelerometer)
  - `XlHg` (high-g accelerometer)
  - `GyNc` (gyroscope)
  - `Timestamp`
- Sensor data is converted to physical units (mg for acceleration, mdps for angular rate).
- Data samples are accumulated and averaged.
- When a timestamp tag is encountered, the program prints the timestamp and the averaged sensor data over UART.
- Accumulators and counters are reset after each output.
- Unhandled FIFO tags are reported over UART.

---

## Usage

1. Connect the LSM6DSV320X sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's FIFO watermark interrupt output to PC0 on the STM32F401RE.
3. Build and flash the firmware onto the STM32F401RE board.
4. Open a serial terminal at 115200 baud on the USART2 TX line.
5. Observe timestamped averaged sensor data printed periodically over UART.

---

## Notes

- The example uses interrupt-driven FIFO watermark signaling combined with polling of FIFO data.
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

*This README explains the embedded Rust program for FIFO-based sensor data streaming from the LSM6DSV320X on STM32F401RE using Embassy STM32 HAL.*
