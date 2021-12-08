#![feature(global_asm)]
//#![feature(asm)]
#![no_main]
#![no_std]

mod uart;
use uart::BcmUart;

//use core::ptr;
mod panic;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub fn _start_kernel() -> ! {
    let uart = BcmUart::new (uart::UART0_START);
    uart.init();
    uart.write_string("Hello World");
    loop{
        uart.write_char(uart.read_char() as char);
    }
}
