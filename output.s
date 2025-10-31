.globl func
        func:
                movl $2, %eax
                ret
.globl main
        main:
                z
                push %rax
                x
                push %rax
                y
                pop %rcx
                imul %rcx, %rax
                pop %rcx
                sub %rcx, %rax
                ret
