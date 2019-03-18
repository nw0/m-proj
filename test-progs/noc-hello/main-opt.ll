; ModuleID = 'main.7rcbfp3g-cgu.0'
source_filename = "main.7rcbfp3g-cgu.0"
target datalayout = "E-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-n32:64-S128-A200-P200-G200"
target triple = "cheri-unknown-freebsd"

@0 = private unnamed_addr addrspace(200) constant <{ [13 x i8] }> <{ [13 x i8] c"Hello, world\0A" }>, align 1

; main::start
; Function Attrs: nounwind nonlazybind uwtable
define hidden i128 @_ZN4main5start17h49970259ac670c11E(i8 addrspace(200)* nocapture readnone %_main, i128 %_argc, i8 addrspace(200)* addrspace(200)* nocapture readnone %_argv) unnamed_addr addrspace(200) #0 {
start:
  %0 = tail call signext i32 (i8 addrspace(200)*, ...) @printf(i8 addrspace(200)* getelementptr inbounds (<{ [13 x i8] }>, <{ [13 x i8] }> addrspace(200)* @0, i64 0, i32 0, i64 0))
  ret i128 0
}

; Function Attrs: nounwind nonlazybind uwtable
declare signext i32 @printf(i8 addrspace(200)* nocapture readonly, ...) unnamed_addr addrspace(200) #0

; Function Attrs: nounwind nonlazybind
define i32 @main(i32, i8 addrspace(200)* addrspace(200)* nocapture readnone) unnamed_addr addrspace(200) #1 {
top:
; call main::start
  %2 = tail call i128 @_ZN4main5start17h49970259ac670c11E(i8 addrspace(200)* undef, i128 undef, i8 addrspace(200)* addrspace(200)* undef)
  ret i32 0
}

attributes #0 = { nounwind nonlazybind uwtable "no-frame-pointer-elim"="true" "target-cpu"="cheri128" "target-features"="+mips64r2" }
attributes #1 = { nounwind nonlazybind "no-frame-pointer-elim"="true" "target-cpu"="cheri128" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIE Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
