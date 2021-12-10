.section .text
.global vector_table

.balign 0x800
vector_table:
cur_el_sp0_sync:
    b unhandled_exeption
.balign 0x80
cur_el_sp0_irq:
    b unhandled_exeption
.balign 0x80
cur_el_sp0_fiq:
    b unhandled_exeption
.balign 0x80
cur_el_sp0_serr:
    b unhandled_exeption
.balign 0x80
cur_el_spn_sync:
    b sync_handler
.balign 0x80
cur_el_spn_irq:
    b unhandled_exeption
.balign 0x80
cur_el_spn_fiq:
    b unhandled_exeption
.balign 0x80
cur_el_spn_serr:
    b unhandled_exeption        
.balign 0x80
lower_el_aarch64_sync:
    b unhandled_exeption
.balign 0x80
lower_el_aarch64_irq:
    b unhandled_exeption
.balign 0x80
lower_el_aarch64_fiq:
    b unhandled_exeption
.balign 0x80
lower_el_aarch64_serr:
    b unhandled_exeption
.balign 0x80
lower_el_aarch32_sync:
    b unhandled_exeption
.balign 0x80
lower_el_aarch32_irq:
    b unhandled_exeption
.balign 0x80
lower_el_aarch32_fiq:
    b unhandled_exeption
.balign 0x80
lower_el_aarch32_serr:
    b unhandled_exeption

sync_handler:
    mov x0, #1
    mrs x1, esr_el2
    mrs x2, elr_el2
    bl _handler
    eret

unhandled_exeption:
    mov x0, #0
    bl _handler
    eret