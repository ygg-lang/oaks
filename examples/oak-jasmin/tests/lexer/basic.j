; Jasmin Assembly Test File - Comprehensive Syntax Coverage
; This file tests various Jasmin assembly syntax elements for lexer testing

; Class declaration
.class public TestClass
.super java/lang/Object

; Source file directive
.source TestClass.java

; Field declarations
.field public static counter I
.field private name Ljava/lang/String;
.field protected value D
.field public final MAX_SIZE I = 100
.field private static instance LTestClass;

; Constant pool entries
.field public static final PI D = 3.14159265359
.field public static final MESSAGE Ljava/lang/String; = "Hello, World!"

; Interface declarations
.implements java/lang/Runnable
.implements java/io/Serializable

; Inner class declarations
.inner class InnerClass inner TestClass$InnerClass outer TestClass

; Method declarations and implementations

; Default constructor
.method public <init>()V
    .limit stack 2
    .limit locals 1
    
    ; Load this reference
    aload_0
    
    ; Call superclass constructor
    invokespecial java/lang/Object/<init>()V
    
    ; Initialize instance fields
    aload_0
    ldc "Default Name"
    putfield TestClass/name Ljava/lang/String;
    
    aload_0
    dconst_0
    putfield TestClass/value D
    
    return
.end method

; Constructor with parameters
.method public <init>(Ljava/lang/String;D)V
    .limit stack 3
    .limit locals 4
    
    aload_0
    invokespecial java/lang/Object/<init>()V
    
    aload_0
    aload_1
    putfield TestClass/name Ljava/lang/String;
    
    aload_0
    dload_2
    putfield TestClass/value D
    
    return
.end method

; Static initializer
.method static <clinit>()V
    .limit stack 1
    .limit locals 0
    
    iconst_0
    putstatic TestClass/counter I
    
    return
.end method

; Simple getter method
.method public getName()Ljava/lang/String;
    .limit stack 1
    .limit locals 1
    
    aload_0
    getfield TestClass/name Ljava/lang/String;
    areturn
.end method

; Simple setter method
.method public setName(Ljava/lang/String;)V
    .limit stack 2
    .limit locals 2
    
    aload_0
    aload_1
    putfield TestClass/name Ljava/lang/String;
    return
.end method

; Method with arithmetic operations
.method public add(II)I
    .limit stack 2
    .limit locals 3
    
    iload_1
    iload_2
    iadd
    ireturn
.end method

; Method with floating-point operations
.method public multiply(DD)D
    .limit stack 4
    .limit locals 5
    
    dload_1
    dload_3
    dmul
    dreturn
.end method

; Method with array operations
.method public createArray(I)[I
    .limit stack 2
    .limit locals 4
    
    ; Create new array
    iload_1
    newarray int
    astore_2
    
    ; Initialize array elements
    iconst_0
    istore_3
    
loop_start:
    iload_3
    iload_1
    if_icmpge loop_end
    
    aload_2
    iload_3
    iload_3
    iload_3
    imul
    iastore
    
    iinc 3 1
    goto loop_start
    
loop_end:
    aload_2
    areturn
.end method

; Method with string operations
.method public concatenateStrings(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    .limit stack 3
    .limit locals 3
    
    new java/lang/StringBuilder
    dup
    invokespecial java/lang/StringBuilder/<init>()V
    
    aload_1
    invokevirtual java/lang/StringBuilder/append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    
    aload_2
    invokevirtual java/lang/StringBuilder/append(Ljava/lang/String;)Ljava/lang/StringBuilder;
    
    invokevirtual java/lang/StringBuilder/toString()Ljava/lang/String;
    areturn
.end method

; Method with exception handling
.method public divide(II)I
    .limit stack 3
    .limit locals 3
    .throws java/lang/ArithmeticException
    
    .catch java/lang/ArithmeticException from try_start to try_end using catch_handler
    
try_start:
    iload_2
    ifne division_ok
    
    new java/lang/ArithmeticException
    dup
    ldc "Division by zero"
    invokespecial java/lang/ArithmeticException/<init>(Ljava/lang/String;)V
    athrow
    
division_ok:
    iload_1
    iload_2
    idiv
    ireturn
    
try_end:

catch_handler:
    ; Re-throw the exception
    athrow
.end method

; Method with switch statement
.method public processCode(I)Ljava/lang/String;
    .limit stack 2
    .limit locals 2
    
    iload_1
    lookupswitch
        1: case_one
        2: case_two
        3: case_three
        default: case_default
    
case_one:
    ldc "One"
    areturn
    
case_two:
    ldc "Two"
    areturn
    
case_three:
    ldc "Three"
    areturn
    
case_default:
    ldc "Unknown"
    areturn
.end method

; Method with table switch
.method public getMonthName(I)Ljava/lang/String;
    .limit stack 2
    .limit locals 2
    
    iload_1
    iconst_1
    isub
    tableswitch 0 11
        month_0
        month_1
        month_2
        month_3
        month_4
        month_5
        month_6
        month_7
        month_8
        month_9
        month_10
        month_11
        default: invalid_month
    
month_0:
    ldc "January"
    areturn
month_1:
    ldc "February"
    areturn
month_2:
    ldc "March"
    areturn
month_3:
    ldc "April"
    areturn
month_4:
    ldc "May"
    areturn
month_5:
    ldc "June"
    areturn
month_6:
    ldc "July"
    areturn
month_7:
    ldc "August"
    areturn
month_8:
    ldc "September"
    areturn
month_9:
    ldc "October"
    areturn
month_10:
    ldc "November"
    areturn
month_11:
    ldc "December"
    areturn
    
invalid_month:
    ldc "Invalid month"
    areturn
.end method

; Method with object creation and method calls
.method public createList()Ljava/util/List;
    .limit stack 3
    .limit locals 2
    
    ; Create ArrayList
    new java/util/ArrayList
    dup
    invokespecial java/util/ArrayList/<init>()V
    astore_1
    
    ; Add elements
    aload_1
    ldc "First"
    invokeinterface java/util/List/add(Ljava/lang/Object;)Z 1
    pop
    
    aload_1
    ldc "Second"
    invokeinterface java/util/List/add(Ljava/lang/Object;)Z 1
    pop
    
    aload_1
    ldc "Third"
    invokeinterface java/util/List/add(Ljava/lang/Object;)Z 1
    pop
    
    aload_1
    areturn
.end method

; Method with type casting and instanceof
.method public processObject(Ljava/lang/Object;)Ljava/lang/String;
    .limit stack 2
    .limit locals 2
    
    aload_1
    instanceof java/lang/String
    ifeq not_string
    
    aload_1
    checkcast java/lang/String
    areturn
    
not_string:
    aload_1
    instanceof java/lang/Number
    ifeq not_number
    
    aload_1
    invokevirtual java/lang/Object/toString()Ljava/lang/String;
    areturn
    
not_number:
    ldc "Unknown type"
    areturn
.end method

; Method with synchronized block
.method public synchronized incrementCounter()V
    .limit stack 2
    .limit locals 1
    
    getstatic TestClass/counter I
    iconst_1
    iadd
    putstatic TestClass/counter I
    
    return
.end method

; Method with monitor operations
.method public safeIncrement()V
    .limit stack 2
    .limit locals 2
    
    aload_0
    dup
    astore_1
    monitorenter
    
    ; Critical section
    getstatic TestClass/counter I
    iconst_1
    iadd
    putstatic TestClass/counter I
    
    aload_1
    monitorexit
    return
.end method

; Method with wide instructions
.method public wideOperations()V
    .limit stack 2
    .limit locals 300
    
    ; Use wide instruction for large local variable index
    wide
    iload 256
    
    wide
    istore 257
    
    wide
    iinc 258 1
    
    return
.end method

; Method with all constant loading instructions
.method public loadConstants()V
    .limit stack 10
    .limit locals 1
    
    ; Integer constants
    iconst_m1
    iconst_0
    iconst_1
    iconst_2
    iconst_3
    iconst_4
    iconst_5
    bipush 100
    sipush 1000
    ldc 100000
    
    ; Long constants
    lconst_0
    lconst_1
    ldc2_w 9223372036854775807L
    
    ; Float constants
    fconst_0
    fconst_1
    fconst_2
    ldc 3.14159f
    
    ; Double constants
    dconst_0
    dconst_1
    ldc2_w 2.718281828459045d
    
    ; String constant
    ldc "Hello, Jasmin!"
    
    ; Class constant
    ldc java/lang/String
    
    ; Null reference
    aconst_null
    
    ; Pop all values from stack
    pop2
    pop2
    pop2
    pop2
    pop2
    pop2
    pop2
    pop2
    pop2
    pop2
    pop
    
    return
.end method

; Method with all load/store instructions
.method public loadStoreOperations(IJFD[I)V
    .limit stack 5
    .limit locals 10
    
    ; Integer operations
    iload_0
    iload_1
    iload 5
    istore_0
    istore_1
    istore 5
    
    ; Long operations
    lload_2
    lload 6
    lstore_2
    lstore 6
    
    ; Float operations
    fload_3
    fload 7
    fstore_3
    fstore 7
    
    ; Double operations
    dload 4
    dload 8
    dstore 4
    dstore 8
    
    ; Reference operations
    aload 5
    aload 9
    astore 5
    astore 9
    
    return
.end method

; Method with array operations
.method public arrayOperations()V
    .limit stack 5
    .limit locals 10
    
    ; Create arrays
    bipush 10
    newarray int
    astore_1
    
    bipush 10
    newarray long
    astore_2
    
    bipush 10
    newarray float
    astore_3
    
    bipush 10
    newarray double
    astore 4
    
    bipush 10
    newarray byte
    astore 5
    
    bipush 10
    newarray char
    astore 6
    
    bipush 10
    newarray short
    astore 7
    
    bipush 10
    newarray boolean
    astore 8
    
    bipush 10
    anewarray java/lang/String
    astore 9
    
    ; Array access operations
    aload_1
    iconst_0
    bipush 42
    iastore
    
    aload_1
    iconst_0
    iaload
    pop
    
    aload_2
    iconst_0
    lconst_1
    lastore
    
    aload_2
    iconst_0
    laload
    pop2
    
    aload_3
    iconst_0
    fconst_1
    fastore
    
    aload_3
    iconst_0
    faload
    pop
    
    aload 4
    iconst_0
    dconst_1
    dastore
    
    aload 4
    iconst_0
    daload
    pop2
    
    aload 5
    iconst_0
    bipush 65
    bastore
    
    aload 5
    iconst_0
    baload
    pop
    
    aload 6
    iconst_0
    bipush 65
    castore
    
    aload 6
    iconst_0
    caload
    pop
    
    aload 7
    iconst_0
    bipush 42
    sastore
    
    aload 7
    iconst_0
    saload
    pop
    
    aload 9
    iconst_0
    ldc "Hello"
    aastore
    
    aload 9
    iconst_0
    aaload
    pop
    
    ; Array length
    aload_1
    arraylength
    pop
    
    return
.end method

; Method with stack manipulation
.method public stackOperations()V
    .limit stack 10
    .limit locals 1
    
    ; Push some values
    iconst_1
    iconst_2
    iconst_3
    lconst_1
    fconst_1
    dconst_1
    
    ; Stack manipulation
    pop2        ; Remove double
    pop         ; Remove float
    pop2        ; Remove long
    dup         ; Duplicate top int
    dup_x1      ; Duplicate and insert below second value
    dup_x2      ; Duplicate and insert below third value
    dup2        ; Duplicate top two values
    dup2_x1     ; Duplicate top two and insert below third
    dup2_x2     ; Duplicate top two and insert below fourth
    swap        ; Swap top two values
    
    ; Clean up stack
    pop
    pop
    pop
    pop
    pop
    pop
    
    return
.end method

; Method with arithmetic and logical operations
.method public arithmeticOperations()V
    .limit stack 10
    .limit locals 1
    
    ; Integer arithmetic
    bipush 10
    bipush 5
    iadd
    bipush 3
    isub
    bipush 2
    imul
    bipush 4
    idiv
    bipush 3
    irem
    ineg
    pop
    
    ; Long arithmetic
    ldc2_w 1000L
    ldc2_w 500L
    ladd
    ldc2_w 100L
    lsub
    ldc2_w 2L
    lmul
    ldc2_w 3L
    ldiv
    ldc2_w 7L
    lrem
    lneg
    pop2
    
    ; Float arithmetic
    ldc 10.5f
    ldc 5.2f
    fadd
    ldc 2.1f
    fsub
    ldc 3.0f
    fmul
    ldc 2.0f
    fdiv
    ldc 3.0f
    frem
    fneg
    pop
    
    ; Double arithmetic
    ldc2_w 10.5d
    ldc2_w 5.2d
    dadd
    ldc2_w 2.1d
    dsub
    ldc2_w 3.0d
    dmul
    ldc2_w 2.0d
    ddiv
    ldc2_w 3.0d
    drem
    dneg
    pop2
    
    ; Bitwise operations
    bipush 15
    bipush 7
    iand
    bipush 8
    ior
    bipush 3
    ixor
    bipush 2
    ishl
    bipush 1
    ishr
    bipush 1
    iushr
    pop
    
    ; Long bitwise operations
    ldc2_w 255L
    ldc2_w 15L
    land
    ldc2_w 240L
    lor
    ldc2_w 85L
    lxor
    bipush 2
    lshl
    bipush 1
    lshr
    bipush 1
    lushr
    pop2
    
    return
.end method

; Method with type conversions
.method public typeConversions()V
    .limit stack 5
    .limit locals 1
    
    ; Integer conversions
    bipush 100
    i2l
    pop2
    
    bipush 100
    i2f
    pop
    
    bipush 100
    i2d
    pop2
    
    bipush 65
    i2b
    pop
    
    bipush 65
    i2c
    pop
    
    bipush 100
    i2s
    pop
    
    ; Long conversions
    ldc2_w 1000L
    l2i
    pop
    
    ldc2_w 1000L
    l2f
    pop
    
    ldc2_w 1000L
    l2d
    pop2
    
    ; Float conversions
    ldc 10.5f
    f2i
    pop
    
    ldc 10.5f
    f2l
    pop2
    
    ldc 10.5f
    f2d
    pop2
    
    ; Double conversions
    ldc2_w 10.5d
    d2i
    pop
    
    ldc2_w 10.5d
    d2l
    pop2
    
    ldc2_w 10.5d
    d2f
    pop
    
    return
.end method

; Method with comparison operations
.method public comparisonOperations()V
    .limit stack 10
    .limit locals 1
    
    ; Integer comparisons
    bipush 10
    bipush 5
    if_icmpeq equal1
    goto not_equal1
equal1:
not_equal1:
    
    bipush 10
    bipush 5
    if_icmpne not_equal2
    goto equal2
not_equal2:
equal2:
    
    bipush 10
    bipush 5
    if_icmplt less1
    goto not_less1
less1:
not_less1:
    
    bipush 10
    bipush 5
    if_icmpge greater_equal1
    goto not_greater_equal1
greater_equal1:
not_greater_equal1:
    
    bipush 10
    bipush 5
    if_icmpgt greater1
    goto not_greater1
greater1:
not_greater1:
    
    bipush 10
    bipush 5
    if_icmple less_equal1
    goto not_less_equal1
less_equal1:
not_less_equal1:
    
    ; Reference comparisons
    aconst_null
    aconst_null
    if_acmpeq ref_equal1
    goto ref_not_equal1
ref_equal1:
ref_not_equal1:
    
    aconst_null
    aconst_null
    if_acmpne ref_not_equal2
    goto ref_equal2
ref_not_equal2:
ref_equal2:
    
    ; Zero comparisons
    iconst_0
    ifeq zero1
    goto not_zero1
zero1:
not_zero1:
    
    iconst_1
    ifne not_zero2
    goto zero2
not_zero2:
zero2:
    
    iconst_m1
    iflt negative1
    goto not_negative1
negative1:
not_negative1:
    
    iconst_1
    ifge non_negative1
    goto negative2
non_negative1:
negative2:
    
    iconst_1
    ifgt positive1
    goto not_positive1
positive1:
not_positive1:
    
    iconst_0
    ifle non_positive1
    goto positive2
non_positive1:
positive2:
    
    ; Null checks
    aconst_null
    ifnull is_null1
    goto not_null1
is_null1:
not_null1:
    
    aconst_null
    ifnonnull not_null2
    goto is_null2
not_null2:
is_null2:
    
    ; Long comparison
    ldc2_w 100L
    ldc2_w 50L
    lcmp
    pop
    
    ; Float comparison
    ldc 10.5f
    ldc 5.2f
    fcmpl
    pop
    
    ldc 10.5f
    ldc 5.2f
    fcmpg
    pop
    
    ; Double comparison
    ldc2_w 10.5d
    ldc2_w 5.2d
    dcmpl
    pop
    
    ldc2_w 10.5d
    ldc2_w 5.2d
    dcmpg
    pop
    
    return
.end method

; Method implementing Runnable interface
.method public run()V
    .limit stack 2
    .limit locals 1
    
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "Running in thread"
    invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
    
    return
.end method

; Main method for testing
.method public static main([Ljava/lang/String;)V
    .limit stack 5
    .limit locals 3
    
    ; Create instance
    new TestClass
    dup
    ldc "Test Instance"
    ldc2_w 42.0d
    invokespecial TestClass/<init>(Ljava/lang/String;D)V
    astore_1
    
    ; Test methods
    aload_1
    bipush 10
    bipush 20
    invokevirtual TestClass/add(II)I
    istore_2
    
    getstatic java/lang/System/out Ljava/io/PrintStream;
    ldc "Addition result: "
    invokevirtual java/io/PrintStream/print(Ljava/lang/String;)V
    
    getstatic java/lang/System/out Ljava/io/PrintStream;
    iload_2
    invokevirtual java/io/PrintStream/println(I)V
    
    ; Test string operations
    aload_1
    ldc "Hello, "
    ldc "World!"
    invokevirtual TestClass/concatenateStrings(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
    astore_2
    
    getstatic java/lang/System/out Ljava/io/PrintStream;
    aload_2
    invokevirtual java/io/PrintStream/println(Ljava/lang/String;)V
    
    return
.end method

; Annotation declarations
.annotation visible Ljava/lang/Deprecated;
.end annotation

.annotation invisible Ljava/lang/SuppressWarnings;
    value = { "unchecked", "rawtypes" }
.end annotation

; Debug information
.line 1
.var 0 is this LTestClass; from start to end
.var 1 is args [Ljava/lang/String; from start to end

; End of class