.globl next
next:
	pushq %rbp
	movq %rsp, %rbp
	movq $0, %r9
	movq $100, %rdx
	movq %r9, %rax
	cmpq %rdx, %rax
	jae L6
L7:
	movq %r9, %rax
	addq $2, %rax
	movq %rax, %r8
	movq %r8, %r9
	movq %rdx, %rax
	addq $1, %rax
	movq %rax, %r8
	movq %r8, %rdx
	movq %r9, %rax
	cmpq %rdx, %rax
	jae L6
	jmp L7
L6:
	movq %r9, %rax
	jmp next_end
next_end:
	movq %rbp, %rsp
	popq %rbp
	ret
.globl func
func:
	pushq %rbp
	movq %rsp, %rbp
	movq $1, %r11
	movq $10, %r10
	movq $20, %r9
	movq $30, %r8
	movq %r11, %rax
	addq %r10, %rax
	movq %rax, %r8
	movq %r8, %rax
	addq %r9, %rax
	movq %rax, %r8
	movq %r8, %r8
	movq %r10, %rax
	cmpq %r11, %rax
	jle L0
	movq $100, %r8
	jmp L2
L0:
	movq %r8, %rax
	cmpq %r9, %rax
	jae L2
	movq $0, %r8
L2:
	movq %r8, %rax
	cmpq %r8, %rax
	jne L3
	movq $200, %r11
L3:
	movq $0, %r8
	movq $0, %r8
	movq %r8, %rax
	cmpq %r8, %rax
	jae L4
L5:
	movq %r8, %r8
	movq %r8, %rax
	addq $1, %rax
	movq %rax, %r8
	movq %r8, %r8
	movq %r8, %rax
	cmpq %r8, %rax
	jae L4
	jmp L5
L4:
	movq %r8, %rax
	jmp func_end
func_end:
	movq %rbp, %rsp
	popq %rbp
	ret
