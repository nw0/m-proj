sum_checked:
        daddiu  $sp, $sp, -16
        sd      $ra, 8($sp)
        sd      $gp, 0($sp)
        lui     $1, %hi(%neg(%gp_rel(sum_checked)))
        daddu   $1, $1, $25
        daddiu  $gp, $1, %lo(%neg(%gp_rel(sum_checked)))
        move    $6, $5
        daddiu  $3, $5, -1
        daddiu  $2, $zero, 0
        daddiu  $5, $zero, 0
.LBB1_1:
        sltu    $1, $5, $6
        beqz    $1, .LBB1_4
        nop
        ld      $1, 0($4)
        daddu   $2, $1, $2
        daddiu  $5, $5, 1
        sltu    $1, $3, $5
        beqz    $1, .LBB1_1
        daddiu  $4, $4, 8
        ld      $gp, 0($sp)
        ld      $ra, 8($sp)
        jr      $ra
        daddiu  $sp, $sp, 16
.LBB1_4:
        ld      $1, %got_page(.L__unnamed_1)($gp)
        ld      $25, %call16(core::panicking::panic_bounds_check)($gp)
        jalr    $25
        daddiu  $4, $1, %got_ofst(.L__unnamed_1)
        break