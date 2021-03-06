#![no_std]
//! Simple driver for the TOF10120 time-of-flight distance sensor.
//! For now, only implements communication over I2C.
//! This time-of-flight sensor yields 16-bit values in millimeters.

use core::marker::PhantomData;
use embedded_hal::blocking::i2c::WriteRead;
const TOF10120_ADDR: u8 = 0x52; //( == 0xa4 & 0x7F)

/// Wrapper for a TOF10120, specific for an I2C peripheral,
/// so that it cannot be written to from multiple peripherals,
/// which would not make sense.
pub struct TOF10120<TI2C> {
    _marker: PhantomData<TI2C>,
}

impl<TI2C, E> TOF10120<TI2C>
where
    TI2C: WriteRead<Error = E>,
{
    /// Initialize the driver, binding it to an I2C peripheral
    pub fn init(_i2c: &mut TI2C) -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Read a sample and fetch the result in millimeters
    pub fn read_sample(&self, i2c: &mut TI2C) -> Result<u16, E> {
        let mut buffer = [0u8; 2];
        i2c.write_read(TOF10120_ADDR, &[0x00], &mut buffer)?;
        Ok(u16::from_be_bytes(buffer))
    }
}
