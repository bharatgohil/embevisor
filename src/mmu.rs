use cortex_a::registers::*;
use tock_registers::interfaces::{Writeable, ReadWriteable};
pub struct MMU {
    
}

impl MMU {    
    pub fn new () -> Self {
        Self {}
    }

    pub fn init(&self) {
        MAIR_EL2.write(MAIR_EL2::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc + 
            MAIR_EL2::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc + 
            MAIR_EL2::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck);

            TCR_EL2.modify(TCR_EL2::T0SZ.val(64 - 48) + TCR_EL2::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable + 
            TCR_EL2::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable + TCR_EL2::SH0::Inner + TCR_EL2::PS::Bits_40 + 
            TCR_EL2::TG0::KiB_64);

            SCTLR_EL2.modify(SCTLR_EL2::I::Cacheable + SCTLR_EL2::WXN::Disable);
            SPSel.write(SPSel::SP::ELx);
    }
    pub fn create_page_table(&self) {

    }
}
