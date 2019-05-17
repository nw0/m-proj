bounds_usize128:
    c2  0x6b5f90
    csc c24,zero,96(c11)
    csc c17,zero,80(c11)
    0x48185811
    lui at,0x2
    daddiu  at,at,-952
    0x481a6051
    0x480cd2bf
    move    at,a3
    move    v0,a2
    move    v1,a1
    move    a4,a0
    xor a5,a2,a0
    sltu    a6,a2,a0
    xori    a6,a6,0x1
    sltu    a7,a3,a1
    xori    a7,a7,0x1
    movz    a6,a7,a5
    csd at,zero,72(c24)     ; restore registers
    csc c12,zero,48(c24)
    csd v0,zero,40(c24)
    csd v1,zero,32(c24)
    csd a4,zero,24(c24)
    csc c3,zero,0(c24)
    bnez    a6,10510        ; panic
