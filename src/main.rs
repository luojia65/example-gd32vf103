#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;

// use gd32vf103_hal::prelude::*;
// use gd32vf103_hal::pac as pac;
// use gd32vf103_hal::gpio::UpTo50MHz;
// use embedded_hal::digital::v2::OutputPin;
use gd32vf103_pac as pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    // let mut parts = dp.GPIOA.split();
    // let mut pa1 = parts.pa1.into_open_drain_output_speed::<UpTo50MHz>(&mut parts.ctl0);
    // let mut pa2 = parts.pa2.into_open_drain_output_speed::<UpTo50MHz>(&mut parts.ctl0);
    // pa1.set_low().unwrap();
    // pa2.set_high().unwrap();
    dp.RCU.apb2en.write(|w| w.paen().set_bit());
    dp.GPIOA.ctl0.write(|w| unsafe {
        w.ctl1().bits(0b00).md1().bits(0b11)
        .ctl2().bits(0b00).md2().bits(0b11)
    });
    // 1
    dp.GPIOA.bop.write(|w| w.bop1().set_bit());
    // 2
    dp.GPIOA.bc.write(|w| w.cr2().set_bit());
    loop {
        unsafe { riscv::asm::wfi() };
    }
}