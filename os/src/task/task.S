.altmacro
.macro SAVE_SN n
    sd s\n, (\n+1)*8(sp)
.endm

.macro LOAD_SN n
    ld s\n, (\n+1)*8(sp)
.endm
.section .text
.global __switch_task 
__switch_task:
# save 
    beqz a0, __load
    addi sp, sp, -13 * 8
    sd sp, 0(a0)
    sd ra, 0(sp)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
__load:
#load
    ld sp, 0(a1)
    ld ra, 0(sp)
    .set n, 0
    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    addi sp, sp, 13 * 8
    ret
