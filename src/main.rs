#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
use gd32vf103_hal as hal;
use hal::prelude::*;
use hal::pac as pac;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcu = dp.RCU.constrain();
    let mut gpioa = dp.GPIOA.split(&mut rcu.apb2);
    let mut pa1 = gpioa.pa1.into_push_pull_output(&mut gpioa.ctl0);
    unsafe {
        let lock = 0x40010818 as *mut u32;
        *lock = 0b00000000_00000001_00000000_00000110;
        *lock = 0b00000000_00000000_00000000_00000110;
        *lock = 0b00000000_00000001_00000000_00000110;
        let res1 = *lock;
        let res2 = *lock;
        if res1 != 0 || res2 & 0x00010000 == 0 {
            panic!("wtf")
        }
        *lock = 0b00000000_00000001_00000000_00000100;
        *lock = 0b00000000_00000000_00000000_00000100;
        *lock = 0b00000000_00000001_00000000_00000100;
        let res1 = *lock;
        let res2 = *lock;
        if res1 != 0 || res2 & 0x00010000 == 0 {
            panic!("wtf")
        }
    }
    pa1.set_low().unwrap();
    loop {}
}
