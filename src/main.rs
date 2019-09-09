#![no_std]
#![no_main]

extern crate panic_halt;

use riscv_rt::entry;
// use gd32vf103_pac as pac;

#[entry]
fn main() -> ! {
    let gpioa_ctl0 = 0x4001_0800_usize as *mut u32;
    let gpioc_ctl1 = 0x4001_1004_usize as *mut u32;
    let gpioa_bop = 0x4001_0810_usize as *mut u32;
    let gpioa_bc = 0x4001_0814_usize as *mut u32;
    let gpioc_bop = 0x4001_1010_usize as *mut u32;
    let gpioc_bc = 0x4001_1014_usize as *mut u32;
    let rcu_apb2en = 0x4002_1018_usize as *mut u32;
    unsafe {
        *rcu_apb2en = 0x00000014_u32;
        *gpioa_ctl0 = 0x44444334_u32;
        *gpioc_ctl1 = 0x44344444_u32;
        *gpioa_bop = 0b00000000_00000000_00000000_00000110_u32;
        *gpioa_bc  = 0b00000000_00000000_00000000_00000010_u32;
        *gpioc_bop = 0b00000000_00000000_00100000_00000000_u32;
        *gpioc_bc  = 0b00000000_00000000_00100000_00000000_u32;
    }
    loop {}
}
