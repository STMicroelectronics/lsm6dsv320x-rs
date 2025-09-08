# LSM6DSV320X Sensor Data Acquisition on STM32F401RE with IKS4A1 Shield Using FIFO and Interrupts

This project demonstrates how to interface the **LSM6DSV320X** inertial measurement unit (IMU) sensor with an **STM32F401RE** microcontroller board using the **IKS4A1** sensor shield. The program reads accelerometer, gyroscope, and timestamp data from the sensor's FIFO buffer via I2C, processes the data, and outputs averaged sensor readings over UART.

This example uses the sensor's FIFO with a watermark interrupt to efficiently batch sensor data reads, reducing CPU overhead and improving power efficiency.

The code is written in Rust using the `stm32f4xx-hal` hardware abstraction layer and the `lsm6dsv320x` sensor driver crate.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor Shield:** IKS4A1 (STMicroelectronics sensor expansion board)
- **Sensor:** LSM6DSV320X IMU (accelerometer + gyroscope + timestamp)
- **Communication Interface:** I2C (400 kHz Standard Mode)
- **UART:** USART2 for serial output at 115200 baud
- **Interrupt Pin:** PC0 configured as input with rising edge interrupt from sensor FIFO watermark signal

### Default Pin Configuration

| Signal       | STM32F401RE Pin | IKS4A1 Shield Pin | Description                      |
|--------------|-----------------|-------------------|----------------------------------|
| I2C1_SCL     | PB8             | SCL               | I2C clock line (open-drain)      |
| I2C1_SDA     | PB9             | SDA               | I2C data line (open-drain)       |
| USART2_TX    | PA2             | UART TX           | UART transmit for debug output   |
| EXTI0 (INT)  | PC0             | INT1 (FIFO watermark) | External interrupt from sensor FIFO watermark |

The IKS4A1 shield is mounted on top of the STM32F401RE Nucleo board, connecting the LSM6DSV320X sensor to the microcontroller's I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). The sensor's FIFO watermark interrupt line is connected to PC0, which is configured to trigger an external interrupt on rising edge. UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals: clocks, GPIO pins, I2C, UART, and timers.
- I2C1 is configured for 400 kHz Standard Mode on open-drain pins PB8 and PB9.
- UART TX is configured on PA2 at 115200 baud for serial output.
- PC0 is configured as an input pin and set up to generate an interrupt on rising edge.
- The external interrupt line is enabled in the NVIC and linked to the EXTI0 interrupt handler.
- A global mutex-protected static holds the interrupt pin to clear interrupt flags safely.

### Sensor Setup

- The LSM6DSV320X sensor is initialized over I2C.
- The device ID is read and verified to confirm sensor presence.
- The sensor is reset to default configuration and waits until reset completes.
- Block Data Update (BDU) is enabled to ensure consistent data reads.
- FIFO watermark is set to 128 samples to trigger interrupts when FIFO reaches this level.
- FIFO batching is enabled for accelerometer (low-g and high-g) and gyroscope at configured ODRs.
- FIFO mode is set to Stream (continuous) mode.
- Timestamp batching and timestamp generation are enabled.
- Output data rates are configured:
  - Low-g accelerometer: 60 Hz
  - High-g accelerometer: 480 Hz
  - Gyroscope: 120 Hz
- Full scale ranges are set:
  - Low-g accelerometer: ±2g
  - High-g accelerometer: ±320g
  - Gyroscope: ±2000 dps
- Filtering options are configured to optimize sensor data quality.
- Interrupt routing is configured to enable FIFO watermark interrupt on the sensor's INT1 pin (connected to PC0).

### Interrupt Handler

- The `EXTI0` interrupt handler is triggered on rising edge of PC0.
- It clears the interrupt pending bit to allow further interrupts.

### Data Acquisition Loop

- The main loop executes `wfi()` (wait for interrupt) to sleep until the FIFO watermark interrupt occurs.
- When woken, it reads the FIFO status to determine how many samples are available.
- It iterates over each FIFO sample, reading raw sensor data and the associated tag.
- Depending on the tag, it converts raw data to physical units and accumulates sums for averaging:
  - Low-g accelerometer samples
  - High-g accelerometer samples
  - Gyroscope samples
- When a timestamp tag is encountered, it prints the timestamp and the averaged sensor data over UART, then resets accumulators.
- Unhandled tags are reported over UART.

---

## Usage

1. Connect the IKS4A1 shield to the STM32F401RE Nucleo board.
2. Connect the sensor's FIFO watermark interrupt line to PC0 on the STM32F401RE.
3. Flash the compiled Rust firmware onto the STM32F401RE.
4. Open a serial terminal at 115200 baud on the UART port.
5. Observe periodic output of averaged accelerometer, gyroscope, and timestamp data triggered by FIFO watermark interrupts.

---

## Notes

- This example uses FIFO buffering and watermark interrupts to efficiently batch sensor data reads.
- The sensor driver handles low-level register access, FIFO parsing, and conversions.
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

*This README provides a detailed explanation of the embedded Rust program for FIFO-based sensor data acquisition on STM32F401RE with the IKS4A1 shield.*