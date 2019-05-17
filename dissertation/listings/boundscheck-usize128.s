bounds_usize128:
        xor     a5,a2,a0
        sltu    a6,a2,a0
        xori    a6,a6,0x1
        sltu    a7,a3,a1
        xori    a7,a7,0x1
        movz    a6,a7,a5
        bnez    a6,10510        ; panic
