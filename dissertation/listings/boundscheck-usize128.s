bounds_usize128:
        xor     a5,a2,a0        ; xor upper bits of idx, len
        sltu    a6,a2,a0
        xori    a6,a6,0x1       ; cmp upper bits
        sltu    a7,a3,a1
        xori    a7,a7,0x1       ; cmp lower bits
        movz    a6,a7,a5        ; use lower bits if upper bits eq
        bnez    a6,10510        ; panic
