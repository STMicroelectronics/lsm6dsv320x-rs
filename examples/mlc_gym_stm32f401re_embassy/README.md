# LSM6DSV320X Gym Right Arm Exercise Detection on STM32F401RE Nucleo-64

This example demonstrates how to detect specific gym exercises—*biceps curl*, *lateral raises*, and *squats*—using the **LSM6DSV320X** inertial measurement unit (IMU) sensor on an **STM32F401RE** Nucleo-64 board. The sensor's machine learning core (MLC) is configured via a JSON-generated register sequence to recognize these exercises, and detected events are output over UART.

The program interfaces with the sensor over I2C, configures the MLC using a JSON-generated configuration, and waits for MLC event interrupts to report recognized exercises.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** LSM6DSV320X IMU (accelerometer + gyroscope + MLC-based exercise detection)
- **Communication Interface:** I2C1 at 100 kHz (PB8 = SCL, PB9 = SDA)
- **UART:** USART2 (PA2 TX) at 115200 baud for serial output
- **Interrupt Pin:** PC0 configured as input with external interrupt for MLC event notification

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI0 (INT)  | PC0             | External interrupt from sensor MLC event |

The LSM6DSV320X sensor is connected via I2C1 on PB8/PB9. The MLC event interrupt line is connected to PC0, configured to trigger an external interrupt on rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals including clocks, GPIO pins, I2C, and UART.
- I2C1 is configured at 100 kHz Standard Mode on pins PB8 (SCL) and PB9 (SDA).
- UART is configured on USART2 (PA2) at 115200 baud, 8 data bits, no parity.
- PC0 is configured as an input pin with no pull resistor and set up as an external interrupt line (EXTI0).
- A delay abstraction is created for timing operations.

### Sensor Setup via JSON Configuration

- The LSM6DSV320X sensor is initialized over I2C with the low I2C address.
- The device ID is read and verified; if mismatched, an error message is sent over UART and the program halts.
- The sensor is reset to default configuration and the program waits until reset completes.
- The sensor is configured by applying a sequence of register writes and delays defined in the `GYM_RIGHT` array, which is generated from a JSON file. This programs the sensor's machine learning core (MLC) to detect gym exercises.

### Event Detection Loop

- The main loop waits for rising edge interrupts on PC0 signaling MLC events.
- When an interrupt occurs, the program reads the sensor's MLC event status.
- If MLC1 event is detected, it reads the MLC output source and matches known event codes:
  - `4` indicates a "biceps curl" event.
  - `8` indicates a "lateral raises" event.
  - `12` indicates a "squats" event.
- Detected events are printed over UART for monitoring.

---

## Usage

1. Connect the LSM6DSV320X sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's MLC interrupt output to PC0 on the STM32F401RE.
3. Build the project, which uses the **`ucf-tool`** to generate Rust configuration code from JSON files automatically at build time.
4. Flash the compiled Rust firmware onto the STM32F401RE.
5. Open a serial terminal at 115200 baud on the UART port.
6. Perform gym exercises corresponding to the configured gestures.
7. Observe event notifications printed over UART.

---

## Notes

- This example uses polling of interrupts and MLC event registers for exercise detection.
- The **`ucf-tool`** enables flexible sensor MLC configuration by converting JSON files into Rust code.
- The sensor driver and JSON-generated code handle low-level register access and MLC programming.
- UART output uses blocking writes without DMA.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic (`defmt` and `panic_probe`).

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [LSM6DSV320X Datasheet](https://www.st.com/resource/en/datasheet/lsm6dsv320x.pdf)
- [Embassy STM32 HAL](https://docs.rs/embassy-stm32)
---

*This README provides a detailed explanation of the embedded Rust program for gym exercise detection on STM32F401RE using the LSM6DSV320X sensor and JSON-generated MLC configuration.*
