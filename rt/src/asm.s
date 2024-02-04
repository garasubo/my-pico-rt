.cpu cortex-m0plus
.thumb

.section .text
.syntax unified
.global asm_execute_process

asm_execute_process:
    push {r4, r5, r6, r7, lr}
    mov r4, r8
    mov r5, r9
    mov r6, r10
    mov r7, r11
    push {r4, r5, r6, r7}
    msr psp, r0
    svc 0
    mrs r0, psp
    pop {r4, r5, r6, r7}
    mov r8, r4
    mov r9, r5
    mov r10, r6
    mov r11, r7
    pop {r4, r5, r6, r7, pc}
