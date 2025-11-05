.globl func
                func: 
                pushq %rbp
                movq %rsp, %rbp
		%movl %edi, 16(%rbp)
		%movl %esi, 24(%rbp)
                mov -0(%rbp), %rax
                push %rax
                
                mov -0(%rbp), %rax
                cqto
                pop %rcx
                idivq %rcx
                mov %rbp, %rsp
                pop %rbp
                ret
.globl main
                main: 
                pushq %rbp
                movq %rsp, %rbp
                mov -0(%rbp), %rax
                push %rax
                
                mov -0(%rbp), %rax
                pop %rcx
                imul %rcx, %rax
                mov %rbp, %rsp
                pop %rbp
                ret
