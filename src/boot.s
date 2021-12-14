// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021 Andre Richter <andre.o.richter@gmail.com>

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

// Load the address of a symbol into a register, PC-relative.
//
// The symbol must lie within +/- 4 GiB of the Program Counter.
//
// # Resources
//
// - https://sourceware.org/binutils/docs-2.36/as/AArch64_002dRelocations.html
.macro ADR_REL register, symbol
	adrp	\register, \symbol
	add	\register, \register, #:lo12:\symbol
.endm

.equ _core_id_mask, 0b11

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	// Only proceed on the boot core. Park it otherwise.
	mrs	x1, MPIDR_EL1
	and	x1, x1, _core_id_mask
	ldr	x2, =0x0     // provided by bsp/__board_name__/cpu.rs
	cmp	x1, x2
	b.ne	.L_parking_loop

	// If execution reaches here, it is the boot core.
	mrs x0, CurrentEL
	lsr x0, x0, #2
	cmp x0, #2
	bne setup_c
	msr sctlr_el1, xzr
	mov x0, #(1<<31)
	msr hcr_el2, x0

	mov x0, #0b1111000101
	msr spsr_el2, x0
	adr x0, setup_c
	msr elr_el2, x0
	eret

setup_c:
	// Initialize DRAM.
	ADR_REL	x0, __bss_start
	ADR_REL x1, __bss_end_exclusive


.L_bss_init_loop:
	cmp	x0, x1
	b.eq	.L_prepare_rust
	stp	xzr, xzr, [x0], #16
	b	.L_bss_init_loop

	// Prepare the jump to Rust code.
.L_prepare_rust:
	// Set the stack pointer.
	ADR_REL	x0, __boot_core_stack_end_exclusive
	mov	sp, x0

	ldr x2, =vector_table
	msr vbar_el1, x2

	mrs x0, CurrentEL
	lsr x0, x0, #2
	// Jump to Rust code.
	b	_start_kernel

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
	wfe
	b	.L_parking_loop
	 
.size	_start, . - _start
.type	_start, function
.global	_start