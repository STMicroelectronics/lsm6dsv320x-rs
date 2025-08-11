# LSM6DSV320X Sensor Data Acquisition on STM32F401RE with IKS4A1 Shield (Interrupt-Driven)

This project demonstrates how to interface the **LSM6DSV320X** inertial measurement unit (IMU) sensor with an **STM32F401RE** microcontroller board using the **IKS4A1** sensor shield. The program reads accelerometer, gyroscope, and temperature data from the sensor via I2C, processes the data, and outputs averaged sensor readings over UART.

This example uses **interrupt-driven data acquisition** by configuring an external interrupt line connected to the sensor's data-ready pin, improving efficiency by avoiding constant polling.

The code is written in Rust using the `stm32f4xx-hal` hardware abstraction layer and the `lsm6dsv320x` sensor driver crate.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor Shield:** IKS4A1 (STMicroelectronics sensor expansion board)
- **Sensor:** LSM6DSV320X IMU (accelerometer + gyroscope + temperature sensor)
- **Communication Interface:** I2C (400 kHz Fast Mode, Duty Cycle 2:1)
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PC0 configured as input with rising edge interrupt from sensor's data-ready signal

### Default Pin Configuration

| Signal       | STM32F401RE Pin | IKS4A1 Shield Pin | Description                      |
|--------------|-----------------|-------------------|----------------------------------|
| I2C1_SCL     | PB8             | SCL               | I2C clock line (open-drain)      |
| I2C1_SDA     | PB9             | SDA               | I2C data line (open-drain)       |
| USART2_TX    | PA2             | UART TX           | UART transmit for debug output   |
| EXTI0 (INT)  | PC0             | INT1 (Data Ready) | External interrupt from sensor   |

The IKS4A1 shield is mounted on top of the STM32F401RE Nucleo board, connecting the LSM6DSV320X sensor to the microcontroller's I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). The sensor's data-ready interrupt line is connected to PC0, which is configured to trigger an external interrupt on rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals: clocks, GPIO pins, I2C, UART, and timers.
- I2C1 is configured for 400 kHz Fast Mode with a 2:1 duty cycle on open-drain pins PB8 and PB9.
- UART TX is configured on PA2 at 115200 baud for serial output.
- PC0 is configured as a pull-up input pin and set up to generate an interrupt on rising edge.
- The external interrupt line is enabled in the NVIC and linked to the EXTI0 interrupt handler.
- A global mutex-protected static holds the interrupt pin to clear interrupt flags safely.
- A global atomic flag (`MEMS_EVENT`) signals the main loop when new data is ready.

### Sensor Setup

- The LSM6DSV320X sensor is initialized over I2C.
- The device ID is read and verified to confirm sensor presence.
- The sensor is reset to default configuration and waits until reset completes.
- Block Data Update (BDU) is enabled to ensure consistent data reads.
- Output data rates are configured:
  - Low-g accelerometer: 60 Hz
  - High-g accelerometer: 960 Hz
  - Gyroscope: 120 Hz
- Full scale ranges are set:
  - Low-g accelerometer: ±2g
  - High-g accelerometer: ±320g
  - Gyroscope: ±2000 dps
- Filtering options are configured to optimize sensor data quality.
- Interrupt routing is configured to enable data-ready signals on the sensor's INT1 pin (connected to PC0).

### Interrupt Handler

- The `EXTI0` interrupt handler is triggered on rising edge of PC0.
- It sets the atomic flag `MEMS_EVENT` to notify the main loop that new sensor data is available.
- The interrupt pending bit is cleared to allow further interrupts.

### Data Acquisition Loop

- The main loop waits for the interrupt flag (`MEMS_EVENT`) to be set.
- When set, it clears the flag and reads the sensor's data-ready status.
- If new data is available, raw sensor values for acceleration, angular rate, and temperature are read.
- Raw values are converted to physical units (mg for acceleration, mdps for gyroscope, °C for temperature).
- Data samples are accumulated and averaged over 100 samples.
- Averaged sensor readings are printed over UART for monitoring.
- If no interrupt is pending, the CPU executes `wfi()` (wait for interrupt) to save power.

---

## Usage

1. Connect the IKS4A1 shield to the STM32F401RE Nucleo board.
2. Connect the sensor's data-ready interrupt line to PC0 on the STM32F401RE.
3. Flash the compiled Rust firmware onto the STM32F401RE.
4. Open a serial terminal at 115200 baud on the UART port.
5. Observe periodic output of averaged accelerometer, gyroscope, and temperature data triggered by sensor interrupts.

---

## Notes

- This example uses interrupt-driven data acquisition for efficient CPU usage.
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

---

*This README provides a detailed explanation of the embedded Rust program for interrupt-driven sensor data acquisition on STM32F401RE with the IKS4A1 shield.*