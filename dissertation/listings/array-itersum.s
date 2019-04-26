sum_iter:
    beqz    $5, .LBB0_4
    nop
    dsll    $3, $5, 3       ; i = arr.len() * 8
    daddiu  $2, $zero, 0    ; retval = 0
.LBB0_2:
    ld      $1, 0($4)       ; *arr
    daddu   $2, $1, $2      ; retval += *arr
    daddiu  $3, $3, -8      ; i -= 8
    bnez    $3, .LBB0_2     ; until i == 0
    daddiu  $4, $4, 8       ; arr += 4 (delay slot)
    jr      $ra
    nop
.LBB0_4:
    jr      $ra
    daddiu  $2, $zero, 0