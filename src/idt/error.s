.section .text

.global error_handler

.macro ERROR_NOCODE	n
.global error\n

error\n:
	push %ebp
	mov %esp, %ebp

	sub $40, %esp
	call get_regs

	push %esp
	push $0
	push $\n
	call error_handler
	add $12, %esp

	call restore_regs
	add $40, %esp

	mov %ebp, %esp
	pop %ebp
	iret
.endm

.macro ERROR_CODE	n
.global error\n

error\n:
	push %eax
	mov 4(%esp), %eax
	mov %eax, -44(%esp)
	pop %eax
	add $4, %esp

	push %ebp
	mov %esp, %ebp

	sub $40, %esp
	call get_regs

	push %esp
	sub $4, %esp
	push $\n
	call error_handler
	add $12, %esp

	call restore_regs
	add $40, %esp

	mov %ebp, %esp
	pop %ebp
	iret
.endm

ERROR_NOCODE 0
ERROR_NOCODE 1
ERROR_NOCODE 2
ERROR_NOCODE 3
ERROR_NOCODE 4
ERROR_NOCODE 5
ERROR_NOCODE 6
ERROR_NOCODE 7
ERROR_CODE 8
ERROR_NOCODE 9
ERROR_CODE 10
ERROR_CODE 11
ERROR_CODE 12
ERROR_CODE 13
ERROR_CODE 14
ERROR_NOCODE 15
ERROR_NOCODE 16
ERROR_CODE 17
ERROR_NOCODE 18
ERROR_NOCODE 19
ERROR_NOCODE 20
ERROR_NOCODE 21
ERROR_NOCODE 22
ERROR_NOCODE 23
ERROR_NOCODE 24
ERROR_NOCODE 25
ERROR_NOCODE 26
ERROR_NOCODE 27
ERROR_NOCODE 28
ERROR_NOCODE 29
ERROR_CODE 30
ERROR_NOCODE 31
