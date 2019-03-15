; ModuleID = 'c.c'
source_filename = "c.c"
target datalayout = "E-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-n32:64-S128-A200-P200-G200"
target triple = "cheri-unknown-freebsd"

; Function Attrs: noinline nounwind optnone
define signext i32 @main() addrspace(0) #0 {
  %1 = alloca i32, align 4, addrspace(200)
  store i32 0, i32 addrspace(200)* %1, align 4
  ret i32 1
}

attributes #0 = { noinline nounwind optnone "correctly-rounded-divide-sqrt-fp-math"="false" "disable-tail-calls"="false" "less-precise-fpmad"="false" "no-frame-pointer-elim"="true" "no-frame-pointer-elim-non-leaf" "no-infs-fp-math"="false" "no-jump-tables"="false" "no-nans-fp-math"="false" "no-signed-zeros-fp-math"="false" "no-trapping-math"="false" "stack-protector-buffer-size"="8" "target-cpu"="cheri128" "target-features"="+cheri128,+chericap,-noabicalls" "unsafe-fp-math"="false" "use-soft-float"="false" }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 1}
!2 = !{!"clang version 8.0.0 (git@github.com:CTSRD-CHERI/clang.git f4516218a824abb38ced9d5ca466e24ef508f46c) (git@github.com:CTSRD-CHERI/llvm.git 209eff0f6c2949ca0be287f0b77899e41e1a55d1)"}
