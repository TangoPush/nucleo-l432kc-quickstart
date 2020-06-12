#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;

use hal::prelude::*;
use hal::delay::Delay;
use rt::ExceptionFrame;
use rt::entry;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = hal::stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain(); // .constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .sysclk(8.mhz())
        // .pclk1(32.mhz())
        .freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb2);
    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut timer = Delay::new(cp.SYST, clocks);
    loop {
        timer.delay_ms(2000 as u32);
        led.set_high().unwrap();
        timer.delay_ms(50 as u32);
        led.set_low().unwrap();
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}