bounds_usize64:
        daddiu  sp,sp,-64
        sd      ra,56(sp)
        sd      s8,48(sp)
        sd      gp,40(sp)
        move    s8,sp
        lui     at,0x1
        daddu   at,at,t9
        daddiu  at,at,31600
        move    t9,a2
        move    v0,a1
        sltu    v1,a2,a1
        sd      at,32(s8)
        sd      t9,24(s8)
        sd      v0,16(s8)
        sd      a0,8(s8)
        beqz    v1,1051c        ; panic
