#![feature(global_asm)]
//#![feature(asm)]
#![no_main]
#![no_std]

mod uart;
use uart::BcmUart;
//use core::fmt;
mod panic;
use core::fmt::Write;
global_asm!(include_str!("vectors.s"));
#[no_mangle]

pub fn _handler(exp_typ: u64, esr: u64, elr: u64) {
    let mut uart = BcmUart::new (uart::UART0_START);
    write!(&mut uart, "Hello World {:X},{:X}", esr,elr).unwrap();
    match exp_typ {
        0=>uart.write_string("unhandled exp"),
        1=>uart.write_string("sync exp"),
        _=>uart.write_string("unknown exp"),
    }
}

global_asm!(include_str!("boot.s"));
#[no_mangle]
pub fn _start_kernel() -> ! {
    let uart = BcmUart::new (uart::UART0_START);
    uart.init();
    uart.write_string("Hello World");
    //fmt::Write::write_str(&mut uart, "Hello world").unwrap();
    //write!(&mut uart, "Hello World....2{}", 3).unwrap();
    //uart.write_string("Hello World");
    let fake_exp = unsafe{&mut *(0xFFFF_0000_0000_0000 as *mut u64)};
    *(fake_exp) = 0xFFFF_0000_0000_0000;
    loop{
        uart.write_char(uart.read_char() as char);
    }
}
