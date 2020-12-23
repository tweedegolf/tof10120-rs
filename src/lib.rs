#![no_std]

use core::marker::PhantomData;
use embedded_hal::blocking::i2c::WriteRead;
const TOF10120_ADDR: u8 = 0x52; //( == 0xa4 & 0x7F)

pub struct TOF10120<TI2C> {
    _marker: PhantomData<TI2C>,
}

impl<TI2C, E> TOF10120<TI2C>
where
    TI2C: WriteRead<Error = E>,
{
    /// Initialize the driver
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
