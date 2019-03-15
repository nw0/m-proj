; ModuleID = 'main.7rcbfp3g-cgu.0'
source_filename = "main.7rcbfp3g-cgu.0"
target datalayout = "E-m:e-pf200:128:128:128:64-i8:8:32-i16:16:32-i64:64-n32:64-S128-A200-G200-P200"
target triple = "cheri-unknown-freebsd"

; main::start
; Function Attrs: nonlazybind uwtable
define hidden i128 @_ZN4main5start17h49970259ac670c11E(i8 addrspace(200)* %_main, i128 %_argc, i8 addrspace(200)* addrspace(200)* %_argv) unnamed_addr addrspace(200) #0 {
start:
  ret i128 1
}

; main::main
; Function Attrs: nonlazybind uwtable
define internal void @_ZN4main4main17h63c14aaf96a48a14E() unnamed_addr addrspace(200) #0 {
start:
  ret void
}

; Function Attrs: nonlazybind
define i32 @main(i32, i8 addrspace(200)* addrspace(200)*) unnamed_addr addrspace(200) #1 {
top:
  %2 = sext i32 %0 to i128
; call main::start
  %3 = call i128 @_ZN4main5start17h49970259ac670c11E(i8 addrspace(200)* bitcast (void () addrspace(200)* @_ZN4main4main17h63c14aaf96a48a14E to i8 addrspace(200)*), i128 %2, i8 addrspace(200)* addrspace(200)* %1)
  %4 = trunc i128 %3 to i32
  ret i32 %4
}

attributes #0 = { nonlazybind uwtable "no-frame-pointer-elim"="true" "target-cpu"="cheri128" "target-features"="+mips64r2" }
attributes #1 = { nonlazybind "no-frame-pointer-elim"="true" "target-cpu"="cheri128" }

!llvm.module.flags = !{!0, !1}

!0 = !{i32 7, !"PIE Level", i32 2}
!1 = !{i32 2, !"RtLibUseGOT", i32 1}
