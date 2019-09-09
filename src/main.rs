#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103_hal as hal;
use hal::prelude::*;
use gd32vf103_hal::pac as pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    dp.RCU.apb2en.write(|w| w.paen().set_bit());
    let mut gpioa = dp.GPIOA.split();
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    pa1.set_low().unwrap();
    // dp.GPIOA.bop.write(|w| unsafe { w.bits(0x0000FFFF) });
    // dp.GPIOA.ctl0.write(|w| unsafe { w.md1().bits(0b11).ctl1().bits(0b00) });
    // dp.GPIOA.bc.write(|w| w.cr1().set_bit());
    loop {}
}
