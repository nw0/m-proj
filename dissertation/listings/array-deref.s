idx:
    lui     $1, %hi(%neg(%gp_rel(idx)))
    daddu   $3, $1, $25
    sltu    $1, $4, $6      ; idx < arr.len()
    beqz    $1, .LBB3_2
    move    $2, $4
    dsll    $1, $2, 3       ; offset = idx * 8
    daddu   $1, $5, $1      ; ptr = &arr + offset
    ld      $2, 0($1)       ; retval = *ptr
    jr      $ra
    nop
.LBB3_2:                    ; prepare information for panic handler
    daddiu  $sp, $sp, -16
    sd      $ra, 8($sp)
    sd      $gp, 0($sp)
    daddiu  $gp, $3, %lo(%neg(%gp_rel(idx)))
    ld      $1, %got_page(.L__unnamed_2)($gp)
    daddiu  $4, $1, %got_ofst(.L__unnamed_2)
    ld      $25, %call16(_ZN4core9panicking18panic_bounds_check17h34b80e64d41db052E)($gp)
    jalr    $25
    move    $5, $2
    break