.section .text._start

_start:
    /*Prepare Rust enviorment */
    ldr x0, =__bss_start
    ldr x1, =__bss_end_exclusive
    /*Zeroing bss section */
bss_zero:
    cmp x0, x1
    beq initiate_rust
    stp xzr, xzr, [x0], #16
    b bss_zero

initiate_rust:
    /*Setup stack pointer */
    ldr x0, =__boot_core_stack_end_exclusive
    mov sp, x0

    ldr x0, =__vector_table
    msr vbar_el2, x0
    /*Jump to Rust code*/
    b _start_kernel

wait_forever:
    wfe
    b wait_forever     