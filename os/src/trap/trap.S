.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm

.macro LOAD_GP n
    ld x\n, \n*8(sp)
.endm
.section .text
.global __restore
.global __trap_entrance
.align 2
__trap_entrance:
    # 交换SP寄存器与sscratch寄存器
    csrrw sp, sscratch, sp
    # 分配34byte空间保存
    addi sp, sp, -34*8
    # x2 跳过
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    # 保存x2(SP)寄存器, 因为sp与sscratch交换了
    csrr t2, sscratch
    sd t2, 2*8(sp)
    mv a0, sp
    call trap_handler
__restore:
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr
    addi sp, sp, 34*8
    csrrw sp, sscratch, sp
    sret
