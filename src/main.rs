#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103_pac as pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    dp.RCU.apb2en.write(|w| w.pcen().set_bit());
    dp.GPIOC.ctl1.write(|w| unsafe {
        w.ctl13().bits(0b00).md13().bits(0b11)
    });
    loop {
        dp.GPIOC.bop.write(|w| w.bop13().set_bit());
        dp.GPIOC.bc.write(|w| w.cr13().set_bit());
    }
}