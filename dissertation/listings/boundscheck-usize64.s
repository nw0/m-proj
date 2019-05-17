bounds_usize64:
        sltu    v1,a2,a1
        beqz    v1,1051c        ; panic
