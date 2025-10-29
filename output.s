.globl main
main:
movl $1, %eax
                push %rax
                movl $3, %eax
                pop %rcx
                sub %rcx, %rax
                push %rax
                movl $1, %eax
                push %rax
                movl $2, %eax
                pop %rcx
                add %rcx, %rax
                pop %rcx
                shl %rcx, %rax
    ret
