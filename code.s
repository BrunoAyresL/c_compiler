.globl factorial
factorial:
	pushq %rbp
	movq %rsp, %rbp
	movq %rdi, %rax
	cmpq $0, %rax
	jae L0
	movq $0, %r10
	movq %r10, %rax
	jmp factorial_end
L0:
	movq %rdi, %rax
	cmpq $0, %rax
	sete %al
	movzbl %al, %eax
	movq %rax, %r10
	movq %rdi, %rax
	cmpq $1, %rax
	sete %al
	movzbl %al, %eax
	movq %rax, %r12
	movq %r10, %rax
	or %r12, %rax
	je L1
	movq $1, %r10
	movq %r10, %rax
	jmp factorial_end
	jmp L2
L1:
	pushq %rdi
	movq %rdi, %rax
	subq $1, %rax
	movq %rax, %r10
	movq %r10, %rdi
	call factorial
	movq %rax, %r10
	popq %rdi
	movq %rdi, %rax
	imulq %r10, %rax
	movq %rax, %r10
	movq %r10, %rax
	jmp factorial_end
	jmp L2
L2:
factorial_end:
	movq %rbp, %rsp
	popq %rbp
	ret
.globl func
func:
	pushq %rbp
	movq %rsp, %rbp
	movq $3, %r11
	movq %r11, %rdi
	call factorial
	movq %rax, %r10
	movq %r10, %r10
	movq %r10, %rax
	cqo
	movq $4, %rbx
	idivq %rbx
	movq %rdx, %r10
	movq %r10, %rax
	jmp func_end
func_end:
	movq %rbp, %rsp
	popq %rbp
	ret
