# TOF10120-rs
Simple driver for the TOF10120 time-of-flight distance sensor, written in Rust.
For now, only implements communication over I2C.
This time-of-flight sensor yields 16-bit values in millimeters.

[Documentation on docs.rs](https://docs.rs/tof10120)

See the [STM32F103 example](./examples/stm32f103.rs) for an example on how to use this driver.