; ModuleID = 'main.7rcbfp3g-cgu.0'
source_filename = "main.7rcbfp3g-cgu.0"
target datalayout = "E-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-n32:64-S128-A200-G200-P200"
target triple = "cheri-unknown-freebsd"

; main::start
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define hidden i128 @_ZN4main5start17h49970259ac670c11E(i8 addrspace(200)* nocapture readnone %_main, i128 %_argc, i8 addrspace(200)* addrspace(200)* nocapture readnone %_argv) unnamed_addr addrspace(200) #0 {
start:
  ret i128 1
}

; Function Attrs: norecurse nounwind nonlazybind readnone
define i32 @main(i32, i8 addrspace(200)* addrspace(200)* nocapture readnone) unnamed_addr addrspace(200) #1 {
top:
  ret i32 1
}

attributes #0 = { norecurse nounwind nonlazybind readnone uwtable "no-frame-pointer-elim"="true" "target-cpu"="cheri128" "target-features"="+mips64r2" }
attributes #1 = { norecurse nounwind nonlazybind readnone "no-frame-pointer-elim"="true" "target-cpu"="cheri128" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIE Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
