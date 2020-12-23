#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_semihosting as _;

use stm32f1xx_hal::gpio::*;
use stm32f1xx_hal::i2c::{BlockingI2c, Mode};
use stm32f1xx_hal::prelude::*;

use stm32f1xx_hal::stm32;

use tof10120::TOF10120;

#[entry]
fn main() -> ! {
    let peripherals = stm32::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.constrain();
    let mut flash = peripherals.FLASH.constrain();
    let mut afio = peripherals.AFIO.constrain(&mut rcc.apb2);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = peripherals.GPIOB.split(&mut rcc.apb2);

    // ==== INIT I2C1 on PB8 and PB9
    let i2c1_scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let i2c1_sck = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);
    let i2c1_pins = (i2c1_scl, i2c1_sck);
    let i2c1_mode = Mode::standard(100.khz());
    let i2c1 = &mut BlockingI2c::i2c1(
        peripherals.I2C1,
        i2c1_pins,
        &mut afio.mapr,
        i2c1_mode,
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    // ==== Init driver
    let tof = TOF10120::init(i2c1);

    // Blockingly read samples and print them over semihosting.
    loop {
        let sample = nb::block!(tof.read_sample(i2c1)).unwrap();
        cortex_m_semihosting::hprintln!("Sample: {:?}mm", sample).unwrap();
    }
}
