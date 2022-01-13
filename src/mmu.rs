use cortex_a::asm::barrier;
pub use barrier::isb;
use cortex_a::registers::*;
use cortex_a::asm;
pub use asm::nop;

use tock_registers:: {
    interfaces::{Writeable, Readable},
    register_bitfields,
    registers::InMemoryRegister,    
};
// A level 1 block descriptor, 39-bit descriptor and table start from level1
register_bitfields! {u64,
    STAGE1_BLOCK_DESCRIPTOR [
        /// Unprivileged execute-never.
        UXN      OFFSET(54) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Privileged execute-never.
        PXN      OFFSET(53) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Physical address block descriptor (lvl3).
        OUTPUT_ADDR_1GB OFFSET(30) NUMBITS(9) [], // [38:30]

        /// Physical address block descriptor (lvl2).
        OUTPUT_ADDR_2MB OFFSET(21) NUMBITS(9) [], // [29:21]

        /// Access flag.
        AF       OFFSET(10) NUMBITS(1) [
            False = 0,
            True = 1
        ],

        /// Shareability field.
        SH       OFFSET(8) NUMBITS(2) [
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],

        /// Access Permissions.
        AP       OFFSET(6) NUMBITS(2) [
            RW_EL2 = 0b00,
            RW_EL2_EL0 = 0b01,
            RO_EL2 = 0b10,
            RO_EL2_EL0 = 0b11
        ],

        /// Memory attributes index into the MAIR_EL1 register.
        AttrIndx OFFSET(2) NUMBITS(3) [
            DEVICE = 0,
            NORMAL = 1
        ],

        TYPE     OFFSET(1) NUMBITS(1) [
            Block = 0,
            Table = 1
        ],

        VALID    OFFSET(0) NUMBITS(1) [
            False = 0,
            True = 1
        ]
    ]
}

#[repr(align(4096))]
pub struct MMU {
    lvl1_table : [u64; 512 ],
    lvl2_table : [u64; 513 ],
}

impl MMU {    
    pub fn new (val:u64) -> Self {
        Self {
            lvl1_table: [val; 512],
            lvl2_table: [val; 513]
        }
    }

    pub fn init(&self) {
        MAIR_EL2.write(MAIR_EL2::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc + 
            MAIR_EL2::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc + 
            MAIR_EL2::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck);

            TCR_EL2.write(TCR_EL2::T0SZ.val(64 - 39) + TCR_EL2::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable + 
            TCR_EL2::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable + TCR_EL2::SH0::Inner + TCR_EL2::PS::Bits_32 + 
            TCR_EL2::TG0::KiB_4 + TCR_EL2::TBI::Ignored);
            HCR_EL2.write(HCR_EL2::VM::Disable + HCR_EL2::DC.val(0));

            SPSel.write(SPSel::SP::ELx);
    }
    pub fn create_page_table(&mut self) {
        let val = InMemoryRegister::<u64, STAGE1_BLOCK_DESCRIPTOR::Register>::new(0);
        let lvl2_entry1 = &self.lvl2_table[0] as *const u64;
        val.write(STAGE1_BLOCK_DESCRIPTOR::VALID::True + STAGE1_BLOCK_DESCRIPTOR::TYPE::Table);
        self.lvl1_table[0] = val.get() | lvl2_entry1 as u64;
        for i in 0..504 {
            val.set(0);
            val.write(STAGE1_BLOCK_DESCRIPTOR::VALID::True + STAGE1_BLOCK_DESCRIPTOR::TYPE::Block + 
                STAGE1_BLOCK_DESCRIPTOR::OUTPUT_ADDR_2MB.val(i) + STAGE1_BLOCK_DESCRIPTOR::AttrIndx::NORMAL + 
            STAGE1_BLOCK_DESCRIPTOR::SH::InnerShareable + STAGE1_BLOCK_DESCRIPTOR::AF::True);
            self.lvl2_table[i as usize] = val.get();
        }

        for i in 504..513 {
            val.set(0);
            val.write(STAGE1_BLOCK_DESCRIPTOR::VALID::True + STAGE1_BLOCK_DESCRIPTOR::TYPE::Block + 
            STAGE1_BLOCK_DESCRIPTOR::OUTPUT_ADDR_2MB.val(i) + STAGE1_BLOCK_DESCRIPTOR::AttrIndx::DEVICE +
            STAGE1_BLOCK_DESCRIPTOR::AF::True);
            self.lvl2_table[i as usize] = val.get();
        }
    }
    pub unsafe fn enable_mmu(&mut self) {
        TTBR0_EL2.set(self.get_base_addr() as u64);
        barrier::isb(barrier::SY);
        SCTLR_EL2.write(SCTLR_EL2::I::Cacheable + SCTLR_EL2::WXN::Disable + SCTLR_EL2::A::Disable
            + SCTLR_EL2::SA::Disable + SCTLR_EL2::M::Enable + SCTLR_EL2::C::Cacheable);
        asm::nop();
        asm::nop();
        barrier::isb(barrier::SY);
    }

    pub fn get_base_addr(&mut self)->*const u64 {
        &self.lvl1_table[0]
    }

    /*pub fn get_tmp_val(&mut self) -> u64{
        let val = InMemoryRegister::<u64, STAGE1_BLOCK_DESCRIPTOR::Register>::new(0);
        let lvl2_entry1 = &self.lvl2_table[0] as *const u64;
        val.write(STAGE1_BLOCK_DESCRIPTOR::VALID::True + STAGE1_BLOCK_DESCRIPTOR::TYPE::Table);
        self.lvl1_table[0] = val.get() | lvl2_entry1 as u64;
        self.lvl1_table[0]    
    }*/
}
