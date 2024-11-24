segment code

global _asm_start
_asm_start:
        mov ax, 4C00h
        int 21h
        ;extern main 
        ;call main 

segment stack class=stack
        resb 100h
