	.file	"code.c"
	.text
	.globl	func
	.def	func;	.scl	2;	.type	32;	.endef
func:
INICIA A FUNCAO
	pushq	%rbp
	movq	%rsp, %rbp
PEGA OS ARGS
	movl	%ecx, 16(%rbp)
	movl	%edx, 24(%rbp)
	movl	16(%rbp), %eax
	cltd
DIVIDE (salvo em eax, mesmo que retorna)
	idivl	24(%rbp)
	popq	%rbp
	ret
	.globl	main
	.def	main;	.scl	2;	.type	32;	.endef
main:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$48, %rsp
	call	__main
	movl	$0, -4(%rbp)
	movl	-4(%rbp), %eax
	addl	$2, %eax
	movl	%eax, -8(%rbp)
	movl	-8(%rbp), %edx
	movl	-4(%rbp), %eax
	movl	%eax, %ecx
	call	func
	movl	%eax, -12(%rbp)
	movl	-4(%rbp), %eax
	imull	-8(%rbp), %eax
	leave
	ret
	.def	__main;	.scl	2;	.type	32;	.endef
