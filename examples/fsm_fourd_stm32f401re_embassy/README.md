# LSM6DSV320X FSM Event Detection on STM32F401RE Nucleo-64 (4-Directional Events)

This example demonstrates how to configure and use the **LSM6DSV320X** inertial measurement unit (IMU) sensor on an **STM32F401RE Nucleo-64** board to detect 4-directional FSM (Finite State Machine) events: X/Y up and down. The sensor is configured via a UCF-generated register sequence, and detected events are output over UART.

The program uses the Embassy STM32 HAL for peripheral initialization and interrupt handling, and the `lsm6dsv320x` Rust driver crate for sensor communication. The sensor FSM events trigger an external interrupt, which the program handles asynchronously to read and report the event.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** LSM6DSV320X IMU (accelerometer + gyroscope + FSM event detection)
- **Communication Interface:** I2C1 at 100 kHz (PB8 = SCL, PB9 = SDA)
- **UART:** USART2 (PA2 TX) at 115200 baud for serial output
- **Interrupt Pin:** PC0 configured as input with external interrupt for FSM event notification

### Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |
| EXTI0 (INT)  | PC0             | External interrupt from sensor FSM event |

The LSM6DSV320X sensor is connected via I2C1 on PB8/PB9. The FSM event interrupt line is connected to PC0, configured to trigger an interrupt on rising edge. UART output is routed through PA2.

---

## Code Description

The configuration runs at 30 Hz.

### Initialization

- The microcontroller peripherals are initialized using Embassy STM32 HAL.
- UART is configured on USART2 (PA2) at 115200 baud, 8 data bits, no parity.
- I2C1 is configured at 100 kHz standard mode on PB8 (SCL) and PB9 (SDA).
- PC0 is configured as an input pin with no pull resistor and set up as an external interrupt line (EXTI0).
- A delay abstraction is created for timing operations.

### Sensor Setup

- The LSM6DSV320X sensor is initialized over I2C with the low I2C address.
- The device ID is read and verified; if mismatched, an error message is sent over UART and the program halts.
- The sensor is reset to default configuration and the program waits until reset completes.
- The sensor is configured by applying a sequence of register writes and delays defined in the `FOUR_D` array, which is generated from a UCF file. This programs the sensor's FSM to detect 4-directional events.

### Main Loop

- The program waits asynchronously for a rising edge interrupt on PC0, indicating an FSM event.
- Upon interrupt, it reads the sensor's event status registers.
- If FSM1 event is detected, it reads the FSM output and matches the event code:
  - `0x10` → "Y down event"
  - `0x20` → "Y up event"
  - `0x40` → "X down event"
  - `0x80` → "X up event"
- The detected event is printed over UART.

---

## Usage

1. Connect the LSM6DSV320X sensor to the STM32F401RE Nucleo board via I2C1 (PB8/PB9).
2. Connect the sensor's FSM interrupt output to PC0 on the STM32F401RE.
3. Build and flash the firmware onto the STM32F401RE board.
4. Open a serial terminal at 115200 baud on the USART2 TX line.
5. Trigger FSM events by moving the sensor in the configured directions.
6. Observe event messages printed on the serial terminal.

---

## Notes

- The sensor FSM configuration is applied via a UCF-generated register sequence (`FOUR_D`), enabling flexible FSM programming.
- UART output uses blocking writes without DMA.
- The example runs entirely within the Embassy async main task.
- Panic behavior is set to halt on panic using `defmt` and `panic_probe`.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust.
- ENU orientation is required.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [LSM6DSV320X Datasheet](https://www.st.com/resource/en/datasheet/lsm6dsv320x.pdf)
- [Embassy STM32 HAL](https://docs.rs/embassy-stm32)
---

*This README explains the embedded Rust example for detecting 4-directional FSM events on the LSM6DSV320X sensor using STM32F401RE and Embassy STM32 HAL.*
