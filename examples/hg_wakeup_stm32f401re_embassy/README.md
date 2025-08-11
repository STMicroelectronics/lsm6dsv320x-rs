# LSM6DSV320X High-G Wakeup Event Detection on STM32F401RE Nucleo-64

This example demonstrates how to detect *High-G wakeup* events using the **LSM6DSV320X** inertial measurement unit (IMU) sensor on an **STM32F401RE** Nucleo-64 board. The sensor is configured to generate interrupts on high-g accelerations, and detected events are reported over UART.

The program initializes the sensor with a high output data rate and full scale, configures filtering and wakeup thresholds, and routes the interrupt to a GPIO pin. The main loop waits for interrupt signals and prints wakeup event details via UART.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** LSM6DSV320X IMU (accelerometer + gyroscope + high-g wakeup detection)
- **Communication Interface:** I2C1 at 400 kHz (PB8 = SCL, PB9 = SDA)
- **UART:** USART2 (PA2 TX) at 115200 baud for serial output
- **Interrupt Pin:** PC0 configured as input with external interrupt for wakeup event notification

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI0 (INT)  | PC0             | External interrupt from sensor wakeup event |

The LSM6DSV320X sensor is connected via I2C1 on PB8/PB9. The wakeup event interrupt line is connected to PC0, configured to trigger an external interrupt on rising edge. UART output is routed through PA2 for serial communication.

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
- The device ID is read and verified; if mismatched, an error message is sent over UART.
- The sensor is reset to default configuration and the program waits until reset completes.
- Block Data Update is enabled to ensure consistent sensor data reads.
- The sensor output data rate is set to 960 Hz for both accelerometer and gyroscope.
- The full scale is set to ±256g to detect high-g events.
- The filtering chain is configured with low-pass filtering and settling masks for data ready and interrupts.
- High-G wakeup configuration is set with shock duration and wakeup threshold.
- Interrupt routing is configured to output High-G wakeup events on INT1 pin (PC0).
- High-G wakeup interrupts are enabled.

### Interrupt Handling and Event Reporting

- An asynchronous task waits for rising edge interrupts on PC0.
- When an interrupt occurs, a signal is sent to the main task.
- The main loop waits for this signal, then reads the High-G event status from the sensor.
- If a wakeup event is detected, the program prints the wakeup event status on X, Y, and Z axes over UART.

---

## Usage

1. Connect the LSM6DSV320X sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's High-G wakeup interrupt output to PC0 on the STM32F401RE.
3. Build and flash the firmware onto the STM32F401RE board.
4. Open a serial terminal at 115200 baud on the USART2 TX line.
5. Generate high-g acceleration events by moving or shaking the sensor.
6. Observe wakeup event notifications printed on the serial terminal.

---

## Notes

- The example uses polling of an interrupt signal combined with asynchronous waiting for GPIO interrupts.
- UART output uses blocking writes without DMA.
- The sensor is configured for high-frequency data output and high-g detection.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic using `defmt` and `panic_probe`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [LSM6DSV320X Datasheet](https://www.st.com/resource/en/datasheet/lsm6dsv320x.pdf)
- [Embassy STM32 HAL](https://docs.rs/embassy-stm32)
---

*This README explains the embedded Rust example for High-G wakeup event detection on the LSM6DSV320X sensor using STM32F401RE and Embassy STM32 HAL.*
