	.file	"code.c"
	.text
	.globl	func
	.def	func;	.scl	2;	.type	32;	.endef
func:
	pushq	%rbp
	movq	%rsp, %rbp
	subq	$16, %rsp
	movl	$1, -4(%rbp)
	movl	$10, -8(%rbp)
	movl	-4(%rbp), %edx
	movl	-8(%rbp), %eax
	addl	%edx, %eax
	leave
	ret
	.ident	"GCC: (Rev2, Built by MSYS2 project) 14.2.0"
