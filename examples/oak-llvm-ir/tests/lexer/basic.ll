; Comprehensive LLVM IR Lexer Test

; --- Target Datalayout & Triple ---
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; --- Named Types ---
%struct.Node = type { i32, %struct.Node* }
%class.MyClass = type { [10 x i8], i32 }
@MyGlobal = global i32 42, align 4

; --- Constants ---
@.str = private unnamed_addr constant [13 x i8] c"hello world\0A\00", align 1

; --- Function Definition ---
define i32 @main(i32 %argc, i8** %argv) #0 {
entry:
    %retval = alloca i32, align 4
    %argc.addr = alloca i32, align 4
    store i32 0, i32* %retval, align 4
    store i32 %argc, i32* %argc.addr, align 4
  
    ; --- Arithmetic Instructions ---
    %0 = load i32, i32* %argc.addr, align 4
    %add = add nsw i32 %0, 1
    %sub = sub nsw i32 %add, 1
    %mul = mul nsw i32 %sub, 2
    %div = sdiv i32 %mul, 2
    %rem = srem i32 %div, 5
  
    ; --- Bitwise Operations ---
    %shl = shl i32 %rem, 1
    %lshr = lshr i32 %shl, 1
    %ashr = ashr i32 %lshr, 1
    %and = and i32 %ashr, 15
    %or = or i32 %and, 1
    %xor = xor i32 %or, 0
  
    ; --- Floating Point ---
    %conv = sitofp i32 %xor to double
    %fadd = fadd double %conv, 1.000000e+00
    %fsub = fsub double %fadd, 0.5
  
    ; --- Control Flow ---
    %cmp = icmp eq i32 %xor, 0
    br i1 %cmp, label %if.then, label %if.else

if.then:                                          ; preds = %entry
    call void @foo()
    br label %if.end

if.else:                                          ; preds = %entry
    %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @.str, i64 0, i64 0))
    br label %if.end

if.end:                                           ; preds = %if.else, %if.then
    ret i32 0
}

; --- External Declaration ---
declare i32 @printf(i8*, ...) #1

; --- Attributes ---
attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" }
attributes #1 = { "no-trapping-math"="true" }

; --- Metadata ---
!llvm.module.flags = !{!0, !1, !2}
!llvm.ident = !{!3}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"uwtable", i32 1}
!2 = !{i32 7, !"frame-pointer", i32 2}
!3 = !{!"clang version 15.0.0"}

; --- Aliases ---
@alias_name = alias i32, i32* @MyGlobal

; --- Inline Assembly ---
define void @asm_test() {
    call void asm sideeffect "nop", "~{dirflag},~{fpsr},~{flags}"()
    ret void
}

; --- Vector Types ---
define <4 x float> @vector_ops(<4 x float> %a, <4 x float> %b) {
    %1 = fadd <4 x float> %a, %b
    ret <4 x float> %1
}

; --- GetElementPtr ---
define i32 @gep_test(%struct.Node* %node) {
    %next_ptr = getelementptr inbounds %struct.Node, %struct.Node* %node, i32 0, i32 1
    %next = load %struct.Node*, %struct.Node** %next_ptr
    ret i32 0
}

; --- Switch ---
define void @switch_test(i32 %val) {
entry:
    switch i32 %val, label %default [
        i32 0, label %case0
        i32 1, label %case1
    ]

case0:
    br label %end
case1:
    br label %end
default:
    br label %end
end:
    ret void
}

; --- Select ---
define i32 @select_test(i1 %cond) {
    %val = select i1 %cond, i32 10, i32 20
    ret i32 %val
}

; --- Phi Node ---
define i32 @phi_test(i1 %cond) {
entry:
    br i1 %cond, label %true_block, label %false_block

true_block:
    br label %merge

false_block:
    br label %merge

merge:
    %result = phi i32 [ 1, %true_block ], [ 2, %false_block ]
    ret i32 %result
}
