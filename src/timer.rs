/*use core::fmt;*/

use cortex_a::asm;
pub use asm::nop;
use cortex_a::{registers::*};
pub const TIMER_START:usize = 0x4000_0000;

use tock_registers::{
    interfaces::{Readable, Writeable},
    register_structs,
    registers::{ReadOnly, ReadWrite},
};

register_structs! {
    #[allow(non_snake_case)]
    pub Registers {
        (0x00 => _reserved1),
        (0x40 => CNTP_EL0: ReadWrite<u32>),
        (0x44 => _reserved2),
        (0x60 => CNTP_STATUS_EL0: ReadOnly<u32>),
        (0x64 => @END),
    }   
}

pub struct BcmTmr {
    registers: &'static mut Registers,   
}

impl BcmTmr {

    pub fn new(start_addr: usize)-> Self{
        Self {
            registers: unsafe { &mut *(start_addr as *mut Registers) },
        }
    }

    pub fn init(&self) {
          
        CNTP_TVAL_EL0.set(CNTFRQ_EL0.get()/100);
        CNTP_CTL_EL0.set(0x1);
        self.registers.CNTP_EL0.set(0x2);
        DAIF.write(DAIF::I::Unmasked)     
    }
    pub fn read_tmr_status(&self)->bool {
        CNTP_CTL_EL0.matches_all(CNTP_CTL_EL0::ISTATUS::SET)
    }

    pub fn read_tmr_irq_status(&self)->u32 {
        self.registers.CNTP_STATUS_EL0.get()
    }
}
