.globl main
        main:
                movl $6, %eax
                push %rax
                movl $10, %eax
                cqto
                pop %rcx
                idivq %rcx
                mov %rdx, %rax
                ret
