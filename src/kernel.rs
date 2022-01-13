#![feature(global_asm)]
//#![feature(asm)]
#![no_main]
#![no_std]

mod uart;
use cortex_a::registers::*;
use tock_registers::interfaces::{Readable};
//use tock_registers::interfaces::Readable;
use uart::BcmUart;
mod timer;
use timer::BcmTmr;
mod mmu;
use mmu::MMU;
/*mod irq;
use irq::BcmIrq;
*/
//use core::fmt;
mod panic;
use core::fmt::{Write};

pub fn timer_irq_handler() {
    let mut uart = BcmUart::new (uart::UART0_START);
    let tmr = BcmTmr::new(timer::TIMER_START);
    let status =  tmr.read_tmr_status();
    if status {
        write!(&mut uart, "Timer IRQ fired2...").unwrap();    
    }
}    

global_asm!(include_str!("vectors.S"));
#[no_mangle]
pub fn _handler(exp_typ: u64, esr: u64, elr: u64) {
    let mut uart = BcmUart::new (uart::UART0_START);
    let tmr = BcmTmr::new(timer::TIMER_START);
    //write!(&mut uart, "Hello World {:X},{:X}", esr,elr).unwrap();
    tmr.init();
    match exp_typ {
        0=>write!(&mut uart, "unhandled exp").unwrap(),
        1=>write!(&mut uart, "sync exp 0x{:X}:0x{:X}", esr, elr).unwrap(),
        2=>{
            //write!(&mut uart, "Inside tmr handle").unwrap();
            let irq_num = tmr.read_tmr_irq_status();
            if irq_num == 0x2 {
                timer_irq_handler();
            }
        }
        _=>write!(&mut uart, "unknown exp").unwrap(),
    }  
}

global_asm!(include_str!("boot.S"));
#[no_mangle]
pub unsafe fn _start_kernel(_vector_table: u64) -> ! {
    let mut uart = BcmUart::new (uart::UART0_START);
    let mut mmu = MMU::new(0); 
    //let tmr = BcmTmr::new(timer::TIMER_START);
    /*let irq = BcmIrq::new(irq::IRQ_START);*/
    uart.init();
    
    
    let el = CurrentEL.read_as_enum(CurrentEL::EL);
    match el {
        Some(CurrentEL::EL::Value::EL2)=>{
            write!(&mut uart, "Booted in EL2").unwrap();
            mmu.init();
            mmu.create_page_table();
            mmu.enable_mmu();
            write!(&mut uart, "MMU Online Now").unwrap();
        },
        Some(CurrentEL::EL::Value::EL1)=>{
            write!(&mut uart, "Booted in EL1").unwrap();
        },
        Some(CurrentEL::EL::Value::EL0)=>{
            write!(&mut uart, "Booted in EL0").unwrap();
        },
        _ =>  write!(&mut uart, "Unknown exception level").unwrap(),
    }
    //tmr.init();
    /*irq.init();*/
   
    write!(&mut uart, "Entring into loop").unwrap();
   
    /*let fake_exp = {&mut *(0xFFFF_0000_0000_0000 as *mut u64)};
    *(fake_exp) = 0xFFFF_0000_0000_0000;*/
    loop{
        uart.write_char(uart.read_char() as char);
    }
}
