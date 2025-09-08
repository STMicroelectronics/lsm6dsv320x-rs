# LSM6DSV320X Sensor Data Acquisition on STM32F401RE with IKS4A1 Shield (High-G Wakeup Interrupt)

This project demonstrates how to interface the **LSM6DSV320X** inertial measurement unit (IMU) sensor with an **STM32F401RE** microcontroller board using the **IKS4A1** sensor shield. The program configures the sensor to detect High-G wakeup events, reads these events via interrupts, and outputs the wakeup axis status over UART.

The code is written in Rust using the `stm32f4xx-hal` hardware abstraction layer and the `lsm6dsv320x` sensor driver crate.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor Shield:** IKS4A1 (STMicroelectronics sensor expansion board)
- **Sensor:** LSM6DSV320X IMU (High-G accelerometer with wakeup detection)
- **Communication Interface:** I2C (400 kHz Fast Mode, Duty Cycle 2:1)
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PC0 configured as input with rising edge interrupt from sensor High-G wakeup signal

### Default Pin Configuration

| Signal       | STM32F401RE Pin | IKS4A1 Shield Pin | Description                      |
|--------------|-----------------|-------------------|----------------------------------|
| I2C1_SCL     | PB8             | SCL               | I2C clock line (open-drain)      |
| I2C1_SDA     | PB9             | SDA               | I2C data line (open-drain)       |
| USART2_TX    | PA2             | UART TX           | UART transmit for debug output   |
| EXTI0 (INT)  | PC0             | INT1 (High-G Wakeup) | External interrupt from sensor High-G wakeup |

The IKS4A1 shield is mounted on top of the STM32F401RE Nucleo board, connecting the LSM6DSV320X sensor to the microcontroller's I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). The sensor's High-G wakeup interrupt line is connected to PC0, which is configured to trigger an external interrupt on rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals: clocks, GPIO pins, I2C, UART, and timers.
- I2C1 is configured for 400 kHz Fast Mode with a 2:1 duty cycle on open-drain pins PB8 and PB9.
- UART TX is configured on PA2 at 115200 baud for serial output.
- PC0 is configured as a pull-up input pin and set up to generate an interrupt on rising edge.
- The external interrupt line is enabled in the NVIC and linked to the EXTI0 interrupt handler.
- A global mutex-protected static holds the interrupt pin to clear interrupt flags safely.
- An atomic flag (`MEMS_EVENT`) signals the main loop when a wakeup event interrupt occurs.

### Sensor Setup

- The LSM6DSV320X sensor is initialized over I2C.
- The device ID is read and verified to confirm sensor presence.
- The sensor is reset to default configuration and waits until reset completes.
- Block Data Update (BDU) is enabled to ensure consistent data reads.
- High-G accelerometer output data rate is set to 960 Hz.
- High-G full scale is set to ±256g to detect strong acceleration events.
- Filtering chain is configured with strong low-pass filtering on accelerometer data.
- High-G wakeup configuration is set:
  - Shock duration enabled (minimum duration for wakeup event)
  - Wakeup threshold set to 4 (sensor-specific units)
- Interrupt routing is configured to enable High-G wakeup interrupt on the sensor's INT1 pin (connected to PC0).
- High-G wakeup interrupt is enabled in the sensor.

### Interrupt Handler

- The `EXTI0` interrupt handler is triggered on rising edge of PC0.
- It sets the atomic flag `MEMS_EVENT` to notify the main loop that a wakeup event occurred.
- The interrupt pending bit is cleared to allow further interrupts.

### Data Acquisition Loop

- The main loop waits for the interrupt flag (`MEMS_EVENT`) to be set.
- When set, it clears the flag and reads the sensor's High-G event status.
- If a wakeup event is detected, it reads the wakeup status on X, Y, and Z axes.
- The wakeup event axes are printed over UART for monitoring.

---

## Usage

1. Connect the IKS4A1 shield to the STM32F401RE Nucleo board.
2. Connect the sensor's High-G wakeup interrupt line to PC0 on the STM32F401RE.
3. Flash the compiled Rust firmware onto the STM32F401RE.
4. Open a serial terminal at 115200 baud on the UART port.
5. Observe printed wakeup event axis status when High-G events occur.

---

## Notes

- This example uses interrupt-driven detection of High-G wakeup events.
- The sensor driver handles low-level register access and conversions.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic (`panic_halt`).
- The external interrupt is configured on PC0 (EXTI0); ensure hardware wiring matches.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [IKS4A1 Sensor Shield](https://www.st.com/en/ecosystems/x-nucleo-iks4a1.html)
- [LSM6DSV320X Datasheet](https://www.st.com/resource/en/datasheet/lsm6dsv320x.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)
- [lsm6dsv320x Rust driver crate](https://crates.io/crates/lsm6dsv320x)

---

*This README provides a detailed explanation of the embedded Rust program for High-G wakeup event detection on STM32F401RE with the IKS4A1 shield.*