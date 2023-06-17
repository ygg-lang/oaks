// MSIL (Microsoft Intermediate Language) Test File
// This file tests various MSIL syntax elements for lexer testing

// Assembly metadata and declarations
.assembly extern mscorlib
{
  .publickeytoken = (B7 7A 5C 56 19 34 E0 89)
  .ver 4:0:0:0
}

.assembly extern System
{
  .publickeytoken = (B7 7A 5C 56 19 34 E0 89)
  .ver 4:0:0:0
}

.assembly extern System.Core
{
  .publickeytoken = (B7 7A 5C 56 19 34 E0 89)
  .ver 4:0:0:0
}

.assembly ComprehensiveTest
{
  .custom instance void [mscorlib]System.Reflection.AssemblyTitleAttribute::.ctor(string) = ( 01 00 12 43 6F 6D 70 72 65 68 65 6E 73 69 76 65 20 54 65 73 74 00 00 )
  .custom instance void [mscorlib]System.Reflection.AssemblyDescriptionAttribute::.ctor(string) = ( 01 00 2A 43 6F 6D 70 72 65 68 65 6E 73 69 76 65 20 4D 53 49 4C 20 54 65 73 74 20 41 73 73 65 6D 62 6C 79 00 00 )
  .custom instance void [mscorlib]System.Reflection.AssemblyConfigurationAttribute::.ctor(string) = ( 01 00 00 00 00 )
  .custom instance void [mscorlib]System.Reflection.AssemblyCompanyAttribute::.ctor(string) = ( 01 00 0B 54 65 73 74 20 43 6F 6D 70 61 6E 79 00 00 )
  .custom instance void [mscorlib]System.Reflection.AssemblyProductAttribute::.ctor(string) = ( 01 00 12 43 6F 6D 70 72 65 68 65 6E 73 69 76 65 20 54 65 73 74 00 00 )
  .custom instance void [mscorlib]System.Reflection.AssemblyCopyrightAttribute::.ctor(string) = ( 01 00 12 43 6F 70 79 72 69 67 68 74 20 32 30 32 34 00 00 )
  .custom instance void [mscorlib]System.Reflection.AssemblyTrademarkAttribute::.ctor(string) = ( 01 00 00 00 00 )
  .custom instance void [mscorlib]System.Reflection.AssemblyVersionAttribute::.ctor(string) = ( 01 00 07 31 2E 30 2E 30 2E 30 00 00 )
  .custom instance void [mscorlib]System.Reflection.AssemblyFileVersionAttribute::.ctor(string) = ( 01 00 07 31 2E 30 2E 30 2E 30 00 00 )
  .hash algorithm 0x00008004
  .ver 1:0:0:0
}

.module ComprehensiveTest.exe
.imagebase 0x00400000
.file alignment 0x00000200
.stackreserve 0x00100000
.subsystem 0x0003       // WINDOWS_CUI
.corflags 0x00000001    //  ILONLY

// Custom attributes
.custom instance void [mscorlib]System.Runtime.CompilerServices.CompilationRelaxationsAttribute::.ctor(int32) = ( 01 00 08 00 00 00 00 00 )
.custom instance void [mscorlib]System.Runtime.CompilerServices.RuntimeCompatibilityAttribute::.ctor() = ( 01 00 01 00 54 02 16 57 72 61 70 4E 6F 6E 45 78 63 65 70 74 69 6F 6E 54 68 72 6F 77 73 01 )

// Namespace and class definitions
.namespace ComprehensiveTest
{
  // Enum definition
  .class public auto ansi sealed Status
         extends [mscorlib]System.Enum
  {
    .field public specialname rtspecialname int32 value__
    .field public static literal valuetype ComprehensiveTest.Status Pending = int32(0)
    .field public static literal valuetype ComprehensiveTest.Status Active = int32(1)
    .field public static literal valuetype ComprehensiveTest.Status Inactive = int32(2)
    .field public static literal valuetype ComprehensiveTest.Status Completed = int32(3)
  }

  // Interface definition
  .class interface public abstract auto ansi ITestInterface
  {
    .method public hidebysig newslot abstract virtual 
            instance int32  Calculate(int32 x, int32 y) cil managed
    {
    }

    .method public hidebysig newslot abstract virtual 
            instance string  GetDescription() cil managed
    {
    }
  }

  // Delegate definition
  .class public auto ansi sealed TestDelegate
         extends [mscorlib]System.MulticastDelegate
  {
    .method public hidebysig specialname rtspecialname 
            instance void  .ctor(object 'object',
                                  native int 'method') runtime managed
    {
    }

    .method public hidebysig newslot virtual 
            instance void  Invoke(string message) runtime managed
    {
    }

    .method public hidebysig newslot virtual 
            instance class [mscorlib]System.IAsyncResult 
            BeginInvoke(string message,
                        class [mscorlib]System.AsyncCallback callback,
                        object 'object') runtime managed
    {
    }

    .method public hidebysig newslot virtual 
            instance void  EndInvoke(class [mscorlib]System.IAsyncResult result) runtime managed
    {
    }
  }

  // Generic class definition
  .class public auto ansi beforefieldinit 'GenericContainer`1'<T>
         extends [mscorlib]System.Object
  {
    .field private !T m_value
    .field private static int32 s_count

    // Static constructor
    .method private hidebysig specialname rtspecialname static 
            void  .cctor() cil managed
    {
      .maxstack  8
      IL_0000:  ldc.i4.0
      IL_0001:  stsfld     int32 class ComprehensiveTest.'GenericContainer`1'<!T>::s_count
      IL_0006:  ret
    }

    // Instance constructor
    .method public hidebysig specialname rtspecialname 
            instance void  .ctor() cil managed
    {
      .maxstack  8
      IL_0000:  ldarg.0
      IL_0001:  call       instance void [mscorlib]System.Object::.ctor()
      IL_0006:  ldsfld     int32 class ComprehensiveTest.'GenericContainer`1'<!T>::s_count
      IL_000b:  ldc.i4.1
      IL_000c:  add
      IL_000d:  stsfld     int32 class ComprehensiveTest.'GenericContainer`1'<!T>::s_count
      IL_0012:  ret
    }

    // Constructor with parameter
    .method public hidebysig specialname rtspecialname 
            instance void  .ctor(!T value) cil managed
    {
      .maxstack  8
      IL_0000:  ldarg.0
      IL_0001:  call       instance void [mscorlib]System.Object::.ctor()
      IL_0006:  ldarg.0
      IL_0007:  ldarg.1
      IL_0008:  stfld      !0 class ComprehensiveTest.'GenericContainer`1'<!T>::m_value
      IL_000d:  ldsfld     int32 class ComprehensiveTest.'GenericContainer`1'<!T>::s_count
      IL_0012:  ldc.i4.1
      IL_0013:  add
      IL_0014:  stsfld     int32 class ComprehensiveTest.'GenericContainer`1'<!T>::s_count
      IL_0019:  ret
    }

    // Property getter
    .method public hidebysig specialname 
            instance !T  get_Value() cil managed
    {
      .maxstack  1
      .locals init (!T V_0)
      IL_0000:  ldarg.0
      IL_0001:  ldfld      !0 class ComprehensiveTest.'GenericContainer`1'<!T>::m_value
      IL_0006:  stloc.0
      IL_0007:  br.s       IL_0009
      IL_0009:  ldloc.0
      IL_000a:  ret
    }

    // Property setter
    .method public hidebysig specialname 
            instance void  set_Value(!T 'value') cil managed
    {
      .maxstack  8
      IL_0000:  ldarg.0
      IL_0001:  ldarg.1
      IL_0002:  stfld      !0 class ComprehensiveTest.'GenericContainer`1'<!T>::m_value
      IL_0007:  ret
    }

    // Property declaration
    .property instance !T Value()
    {
      .get instance !T ComprehensiveTest.'GenericContainer`1'::get_Value()
      .set instance void ComprehensiveTest.'GenericContainer`1'::set_Value(!T)
    }

    // Static property
    .method public hidebysig specialname static 
            int32  get_Count() cil managed
    {
      .maxstack  1
      .locals init (int32 V_0)
      IL_0000:  ldsfld     int32 class ComprehensiveTest.'GenericContainer`1'<!T>::s_count
      IL_0005:  stloc.0
      IL_0006:  br.s       IL_0008
      IL_0008:  ldloc.0
      IL_0009:  ret
    }

    .property int32 Count()
    {
      .get int32 ComprehensiveTest.'GenericContainer`1'::get_Count()
    }
  }

  // Main test class
  .class public auto ansi beforefieldinit TestClass
         extends [mscorlib]System.Object
         implements ComprehensiveTest.ITestInterface
  {
    // Fields
    .field private int32 m_instanceField
    .field private static string s_staticField
    .field public string PublicField
    .field assembly string InternalField
    .field family string ProtectedField
    .field famandassem string ProtectedInternalField
    .field private static literal int32 CONSTANT_VALUE = int32(42)

    // Events
    .field private class ComprehensiveTest.TestDelegate TestEvent
    .event ComprehensiveTest.TestDelegate TestEvent
    {
      .addon instance void ComprehensiveTest.TestClass::add_TestEvent(class ComprehensiveTest.TestDelegate)
      .removeon instance void ComprehensiveTest.TestClass::remove_TestEvent(class ComprehensiveTest.TestDelegate)
    }

    // Static constructor
    .method private hidebysig specialname rtspecialname static 
            void  .cctor() cil managed
    {
      .maxstack  8
      IL_0000:  ldstr      "Static initialization"
      IL_0005:  stsfld     string ComprehensiveTest.TestClass::s_staticField
      IL_000a:  ret
    }

    // Instance constructor
    .method public hidebysig specialname rtspecialname 
            instance void  .ctor() cil managed
    {
      .maxstack  8
      IL_0000:  ldarg.0
      IL_0001:  call       instance void [mscorlib]System.Object::.ctor()
      IL_0006:  ldarg.0
      IL_0007:  ldc.i4.0
      IL_0008:  stfld      int32 ComprehensiveTest.TestClass::m_instanceField
      IL_000d:  ldarg.0
      IL_000e:  ldstr      "Default public field"
      IL_0013:  stfld      string ComprehensiveTest.TestClass::PublicField
      IL_0018:  ret
    }

    // Constructor with parameters
    .method public hidebysig specialname rtspecialname 
            instance void  .ctor(int32 value, string text) cil managed
    {
      .maxstack  8
      IL_0000:  ldarg.0
      IL_0001:  call       instance void [mscorlib]System.Object::.ctor()
      IL_0006:  ldarg.0
      IL_0007:  ldarg.1
      IL_0008:  stfld      int32 ComprehensiveTest.TestClass::m_instanceField
      IL_000d:  ldarg.0
      IL_000e:  ldarg.2
      IL_000f:  stfld      string ComprehensiveTest.TestClass::PublicField
      IL_0014:  ret
    }

    // Interface implementation
    .method public hidebysig newslot virtual final 
            instance int32  Calculate(int32 x, int32 y) cil managed
    {
      .override ComprehensiveTest.ITestInterface::Calculate
      .maxstack  2
      .locals init (int32 V_0)
      IL_0000:  ldarg.1
      IL_0001:  ldarg.2
      IL_0002:  add
      IL_0003:  stloc.0
      IL_0004:  br.s       IL_0006
      IL_0006:  ldloc.0
      IL_0007:  ret
    }

    .method public hidebysig newslot virtual final 
            instance string  GetDescription() cil managed
    {
      .override ComprehensiveTest.ITestInterface::GetDescription
      .maxstack  1
      .locals init (string V_0)
      IL_0000:  ldstr      "TestClass implementation"
      IL_0005:  stloc.0
      IL_0006:  br.s       IL_0008
      IL_0008:  ldloc.0
      IL_0009:  ret
    }

    // Event accessors
    .method public hidebysig specialname 
            instance void  add_TestEvent(class ComprehensiveTest.TestDelegate 'value') cil managed
    {
      .maxstack  3
      .locals init (class ComprehensiveTest.TestDelegate V_0,
                    class ComprehensiveTest.TestDelegate V_1,
                    class ComprehensiveTest.TestDelegate V_2)
      IL_0000:  ldarg.0
      IL_0001:  ldfld      class ComprehensiveTest.TestDelegate ComprehensiveTest.TestClass::TestEvent
      IL_0006:  stloc.0
      IL_0007:  ldloc.0
      IL_0008:  stloc.1
      IL_0009:  ldloc.1
      IL_000a:  ldarg.1
      IL_000b:  call       class [mscorlib]System.Delegate [mscorlib]System.Delegate::Combine(class [mscorlib]System.Delegate,
                                                                                                class [mscorlib]System.Delegate)
      IL_0010:  castclass  ComprehensiveTest.TestDelegate
      IL_0015:  stloc.2
      IL_0016:  ldarg.0
      IL_0017:  ldflda     class ComprehensiveTest.TestDelegate ComprehensiveTest.TestClass::TestEvent
      IL_001c:  ldloc.2
      IL_001d:  ldloc.1
      IL_001e:  call       !!0 [mscorlib]System.Threading.Interlocked::CompareExchange<class ComprehensiveTest.TestDelegate>(!!0&,
                                                                                                                              !!0,
                                                                                                                              !!0)
      IL_0023:  stloc.0
      IL_0024:  ldloc.0
      IL_0025:  ldloc.1
      IL_0026:  bne.un.s   IL_0007
      IL_0028:  ret
    }

    .method public hidebysig specialname 
            instance void  remove_TestEvent(class ComprehensiveTest.TestDelegate 'value') cil managed
    {
      .maxstack  3
      .locals init (class ComprehensiveTest.TestDelegate V_0,
                    class ComprehensiveTest.TestDelegate V_1,
                    class ComprehensiveTest.TestDelegate V_2)
      IL_0000:  ldarg.0
      IL_0001:  ldfld      class ComprehensiveTest.TestDelegate ComprehensiveTest.TestClass::TestEvent
      IL_0006:  stloc.0
      IL_0007:  ldloc.0
      IL_0008:  stloc.1
      IL_0009:  ldloc.1
      IL_000a:  ldarg.1
      IL_000b:  call       class [mscorlib]System.Delegate [mscorlib]System.Delegate::Remove(class [mscorlib]System.Delegate,
                                                                                              class [mscorlib]System.Delegate)
      IL_0010:  castclass  ComprehensiveTest.TestDelegate
      IL_0015:  stloc.2
      IL_0016:  ldarg.0
      IL_0017:  ldflda     class ComprehensiveTest.TestDelegate ComprehensiveTest.TestClass::TestEvent
      IL_001c:  ldloc.2
      IL_001d:  ldloc.1
      IL_001e:  call       !!0 [mscorlib]System.Threading.Interlocked::CompareExchange<class ComprehensiveTest.TestDelegate>(!!0&,
                                                                                                                              !!0,
                                                                                                                              !!0)
      IL_0023:  stloc.0
      IL_0024:  ldloc.0
      IL_0025:  ldloc.1
      IL_0026:  bne.un.s   IL_0007
      IL_0028:  ret
    }

    // Method with various instruction types
    .method public hidebysig 
            instance void  ComplexMethod(int32 param1, 
                                         string param2, 
                                         bool param3) cil managed
    {
      .maxstack  3
      .locals init (int32 V_0,
                    string V_1,
                    bool V_2,
                    int32 V_3,
                    bool V_4)

      // Load constants
      IL_0000:  ldc.i4.0
      IL_0001:  stloc.0
      IL_0002:  ldstr      ""
      IL_0007:  stloc.1
      IL_0008:  ldc.i4.0
      IL_0009:  stloc.2

      // Arithmetic operations
      IL_000a:  ldarg.1
      IL_000b:  ldc.i4.s   10
      IL_000d:  add
      IL_000e:  stloc.3

      // Conditional branching
      IL_000f:  ldarg.3
      IL_0010:  brfalse.s  IL_0025

      // String operations
      IL_0012:  ldarg.2
      IL_0013:  ldstr      " - processed"
      IL_0018:  call       string [mscorlib]System.String::Concat(string, string)
      IL_001d:  stloc.1
      IL_001e:  ldc.i4.1
      IL_001f:  stloc.2
      IL_0020:  br.s       IL_002b

      IL_0022:  ldarg.2
      IL_0023:  stloc.1

      // Loop example
      IL_0025:  ldc.i4.0
      IL_0026:  stloc.0
      IL_0027:  br.s       IL_0035

      IL_0029:  ldloc.0
      IL_002a:  ldc.i4.1
      IL_002b:  add
      IL_002c:  stloc.0

      IL_002d:  ldloc.0
      IL_002e:  ldc.i4.s   10
      IL_0030:  clt
      IL_0032:  stloc.s    V_4
      IL_0034:  ldloc.s    V_4
      IL_0036:  brtrue.s   IL_0029

      // Method call
      IL_0038:  ldarg.0
      IL_0039:  ldloc.3
      IL_003a:  ldc.i4.5
      IL_003b:  call       instance int32 ComprehensiveTest.TestClass::Calculate(int32, int32)
      IL_0040:  pop

      IL_0041:  ret
    }

    // Method with exception handling
    .method public hidebysig 
            instance int32  DivideWithExceptionHandling(int32 numerator, 
                                                         int32 denominator) cil managed
    {
      .maxstack  2
      .locals init (int32 V_0,
                    class [mscorlib]System.Exception V_1)

      .try
      {
        IL_0000:  ldarg.1
        IL_0001:  ldarg.2
        IL_0002:  div
        IL_0003:  stloc.0
        IL_0004:  leave.s    IL_001a
      }
      catch [mscorlib]System.DivideByZeroException
      {
        IL_0006:  pop
        IL_0007:  ldc.i4.0
        IL_0008:  stloc.0
        IL_0009:  leave.s    IL_001a
      }
      catch [mscorlib]System.Exception
      {
        IL_000b:  stloc.1
        IL_000c:  ldloc.1
        IL_000d:  callvirt   instance string [mscorlib]System.Object::ToString()
        IL_0012:  call       void [mscorlib]System.Console::WriteLine(string)
        IL_0017:  ldc.i4.m1
        IL_0018:  stloc.0
        IL_0019:  leave.s    IL_001a
      }

      IL_001a:  ldloc.0
      IL_001b:  ret
    }

    // Method with finally block
    .method public hidebysig 
            instance void  MethodWithFinally() cil managed
    {
      .maxstack  1
      .locals init (bool V_0)

      .try
      {
        IL_0000:  ldstr      "In try block"
        IL_0005:  call       void [mscorlib]System.Console::WriteLine(string)
        IL_000a:  leave.s    IL_001c
      }
      finally
      {
        IL_000c:  ldstr      "In finally block"
        IL_0011:  call       void [mscorlib]System.Console::WriteLine(string)
        IL_0016:  endfinally
      }

      IL_0017:  ldstr      "After try-finally"
      IL_001c:  call       void [mscorlib]System.Console::WriteLine(string)
      IL_0021:  ret
    }

    // Generic method
    .method public hidebysig 
            instance !!T  GenericMethod<T>(!!T input) cil managed
    {
      .maxstack  1
      .locals init (!!T V_0)
      IL_0000:  ldarg.1
      IL_0001:  stloc.0
      IL_0002:  br.s       IL_0004
      IL_0004:  ldloc.0
      IL_0005:  ret
    }

    // Method with ref and out parameters
    .method public hidebysig 
            instance void  RefOutMethod(int32& refParam, 
                                        [out] int32& outParam) cil managed
    {
      .param [2]
      .custom instance void [mscorlib]System.Runtime.InteropServices.OutAttribute::.ctor() = ( 01 00 00 00 )
      .maxstack  3
      IL_0000:  ldarg.1
      IL_0001:  ldarg.1
      IL_0002:  ldind.i4
      IL_0003:  ldc.i4.1
      IL_0004:  add
      IL_0005:  stind.i4
      IL_0006:  ldarg.2
      IL_0007:  ldc.i4.s   42
      IL_0009:  stind.i4
      IL_000a:  ret
    }

    // Async method (simplified)
    .method public hidebysig 
            instance class [mscorlib]System.Threading.Tasks.Task 
            AsyncMethod() cil managed
    {
      .custom instance void [mscorlib]System.Runtime.CompilerServices.AsyncStateMachineAttribute::.ctor(class [mscorlib]System.Type) = ( 01 00 2B 43 6F 6D 70 72 65 68 65 6E 73 69 76 65 54 65 73 74 2E 54 65 73 74 43 6C 61 73 73 2B 3C 41 73 79 6E 63 4D 65 74 68 6F 64 3E 64 5F 5F 31 00 00 )
      .maxstack  2
      .locals init (class [mscorlib]System.Threading.Tasks.Task V_0)
      IL_0000:  call       class [mscorlib]System.Threading.Tasks.Task [mscorlib]System.Threading.Tasks.Task::get_CompletedTask()
      IL_0005:  stloc.0
      IL_0006:  br.s       IL_0008
      IL_0008:  ldloc.0
      IL_0009:  ret
    }
  }

  // Program entry point
  .class private auto ansi beforefieldinit Program
         extends [mscorlib]System.Object
  {
    .method private hidebysig static void  Main(string[] args) cil managed
    {
      .entrypoint
      .maxstack  4
      .locals init (class ComprehensiveTest.TestClass V_0,
                    class ComprehensiveTest.'GenericContainer`1'<int32> V_1,
                    class ComprehensiveTest.'GenericContainer`1'<string> V_2,
                    int32 V_3,
                    int32 V_4,
                    string V_5)

      // Create test instance
      IL_0000:  newobj     instance void ComprehensiveTest.TestClass::.ctor()
      IL_0005:  stloc.0

      // Test basic functionality
      IL_0006:  ldloc.0
      IL_0007:  ldc.i4.s   10
      IL_0009:  ldc.i4.s   20
      IL_000b:  callvirt   instance int32 ComprehensiveTest.TestClass::Calculate(int32, int32)
      IL_0010:  stloc.3

      // Test generic container with int
      IL_0011:  ldc.i4.s   42
      IL_0013:  newobj     instance void class ComprehensiveTest.'GenericContainer`1'<int32>::.ctor(!0)
      IL_0018:  stloc.1

      // Test generic container with string
      IL_0019:  ldstr      "Hello World"
      IL_001e:  newobj     instance void class ComprehensiveTest.'GenericContainer`1'<string>::.ctor(!0)
      IL_0023:  stloc.2

      // Test exception handling
      IL_0024:  ldloc.0
      IL_0025:  ldc.i4.s   10
      IL_0027:  ldc.i4.0
      IL_0028:  call       instance int32 ComprehensiveTest.TestClass::DivideWithExceptionHandling(int32, int32)
      IL_002d:  stloc.s    V_4

      // Test ref/out parameters
      IL_002f:  ldloca.s   V_3
      IL_0031:  ldloca.s   V_4
      IL_0033:  ldloc.0
      IL_0034:  call       instance void ComprehensiveTest.TestClass::RefOutMethod(int32&, int32&)

      // Test complex method
      IL_0039:  ldloc.0
      IL_003a:  ldc.i4.s   100
      IL_003c:  ldstr      "Test message"
      IL_0041:  ldc.i4.1
      IL_0042:  call       instance void ComprehensiveTest.TestClass::ComplexMethod(int32, string, bool)

      // Test finally method
      IL_0047:  ldloc.0
      IL_0048:  call       instance void ComprehensiveTest.TestClass::MethodWithFinally()

      // Test generic method
      IL_004d:  ldloc.0
      IL_004e:  ldstr      "Generic test"
      IL_0053:  call       instance !!0 ComprehensiveTest.TestClass::GenericMethod<string>(!!0)
      IL_0058:  stloc.s    V_5

      // Output results
      IL_005a:  ldstr      "Calculation result: {0}"
      IL_005f:  ldloc.3
      IL_0060:  box        [mscorlib]System.Int32
      IL_0065:  call       void [mscorlib]System.Console::WriteLine(string, object)

      IL_006a:  ldstr      "Generic int value: {0}"
      IL_006f:  ldloc.1
      IL_0070:  call       instance !0 class ComprehensiveTest.'GenericContainer`1'<int32>::get_Value()
      IL_0075:  box        [mscorlib]System.Int32
      IL_007a:  call       void [mscorlib]System.Console::WriteLine(string, object)

      IL_007f:  ldstr      "Generic string value: {0}"
      IL_0084:  ldloc.2
      IL_0085:  call       instance !0 class ComprehensiveTest.'GenericContainer`1'<string>::get_Value()
      IL_008a:  call       void [mscorlib]System.Console::WriteLine(string, object)

      IL_008f:  ldstr      "Division result: {0}"
      IL_0094:  ldloc.s    V_4
      IL_0096:  box        [mscorlib]System.Int32
      IL_009b:  call       void [mscorlib]System.Console::WriteLine(string, object)

      IL_00a0:  ldstr      "Ref parameter: {0}, Out parameter: {1}"
      IL_00a5:  ldloc.3
      IL_00a6:  box        [mscorlib]System.Int32
      IL_00ab:  ldloc.s    V_4
      IL_00ad:  box        [mscorlib]System.Int32
      IL_00b2:  call       void [mscorlib]System.Console::WriteLine(string, object, object)

      IL_00b7:  ldstr      "Generic method result: {0}"
      IL_00bc:  ldloc.s    V_5
      IL_00be:  call       void [mscorlib]System.Console::WriteLine(string, object)

      IL_00c3:  ldstr      "Container count: {0}"
      IL_00c8:  call       int32 class ComprehensiveTest.'GenericContainer`1'<int32>::get_Count()
      IL_00cd:  box        [mscorlib]System.Int32
      IL_00d2:  call       void [mscorlib]System.Console::WriteLine(string, object)

      IL_00d7:  ldstr      "Press any key to exit..."
      IL_00dc:  call       void [mscorlib]System.Console::WriteLine(string)
      IL_00e1:  call       valuetype [mscorlib]System.ConsoleKeyInfo [mscorlib]System.Console::ReadKey()
      IL_00e6:  pop

      IL_00e7:  ret
    }

    .method public hidebysig specialname rtspecialname 
            instance void  .ctor() cil managed
    {
      .maxstack  8
      IL_0000:  ldarg.0
      IL_0001:  call       instance void [mscorlib]System.Object::.ctor()
      IL_0006:  ret
    }
  }
}

// Global methods (outside namespace)
.method public hidebysig static void GlobalMethod() cil managed
{
  .maxstack  1
  IL_0000:  ldstr      "Global method called"
  IL_0005:  call       void [mscorlib]System.Console::WriteLine(string)
  IL_000a:  ret
}

// P/Invoke declaration
.method public hidebysig static pinvokeimpl("kernel32.dll" winapi) 
        bool GetComputerNameA(class [mscorlib]System.Text.StringBuilder lpBuffer,
                              uint32& nSize) cil managed preservesig
{
}

// Unmanaged function pointer
.method public hidebysig static 
        method void *(int32) GetFunctionPointer() cil managed
{
  .maxstack  1
  .locals init (method void *(int32) V_0)
  IL_0000:  ldnull
  IL_0001:  stloc.0
  IL_0002:  br.s       IL_0004
  IL_0004:  ldloc.0
  IL_0005:  ret
}

// Data section
.data cil I_00003000 = bytearray (
    48 65 6C 6C 6F 20 57 6F 72 6C 64 00 // "Hello World\0"
)

.data cil I_00003010 = bytearray (
    01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10
)

// Version info
.ver 1:0:0:0