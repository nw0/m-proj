bounds_usize64:
        lui     $1, %hi(%neg(%gp_rel(bounds_usize64)))
        daddu   $3, $1, $25
        sltu    $1, $6, $5
        beqz    $1, .LBB1_2     ; panic