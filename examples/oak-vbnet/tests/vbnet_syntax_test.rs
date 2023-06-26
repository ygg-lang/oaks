use oak_vbnet::parse;

#[test]
fn test_vbnet_syntax() {
    let source = r#"' Test file for VB.NET syntax features

'''
This is a block comment
'''

Imports System
Imports System.Collections.Generic

Namespace TestNamespace
    ' Test class with generics
    Public Class TestClass(Of T)
        Inherits BaseClass
        Implements ITestInterface

        ' Test constant
        Public Const PI As Double = 3.14159

        ' Test fields
        Private _name As String
        Private _value As T

        ' Test property
        Public Property Name() As String
            Get
                Return _name
            End Get
            Set(ByVal value As String)
                _name = value
            End Set
        End Property

        ' Test function with generics
        Public Function GetValue(Of U)(ByVal input As U) As T
            Return _value
        End Function

        ' Test sub with generics
        Public Sub SetValue(Of U)(ByVal input As U)
            _value = CType(input, T)
        End Sub

        ' Test interface implementation
        Public Sub TestMethod() Implements ITestInterface.TestMethod
            Console.WriteLine("TestMethod called")
        End Sub

        ' Test if statement
        Public Sub TestIf(ByVal value As Integer)
            If value > 0 Then
                Console.WriteLine("Positive")
            ElseIf value < 0 Then
                Console.WriteLine("Negative")
            Else
                Console.WriteLine("Zero")
            End If
        End Sub

        ' Test for loop
        Public Sub TestFor()
            For i As Integer = 1 To 10 Step 2
                Console.WriteLine(i)
            Next
        End Sub

        ' Test foreach loop
        Public Sub TestForEach(ByVal items As List(Of String))
            For Each item As String In items
                Console.WriteLine(item)
            Next
        End Sub

        ' Test while loop
        Public Sub TestWhile(ByVal count As Integer)
            While count > 0
                Console.WriteLine(count)
                count -= 1
            End While
        End Sub

        ' Test do-while
        Public Sub TestDoWhile(ByVal count As Integer)
            Do While count > 0
                Console.WriteLine(count)
                count -= 1
            Loop
        End Sub

        ' Test select case
        Public Sub TestSelectCase(ByVal value As Integer)
            Select Case value
                Case 1
                    Console.WriteLine("One")
                Case 2
                    Console.WriteLine("Two")
                Case Else
                    Console.WriteLine("Other")
            End Select
        End Sub

        ' Test try-catch
        Public Sub TestTryCatch()
            Try
                Dim x As Integer = 10 / 0
            Catch ex As Exception
                Console.WriteLine("Error: " & ex.Message)
            Finally
                Console.WriteLine("Finally block")
            End Try
        End Sub

        ' Test with statement
        Public Sub TestWith(ByVal obj As TestClass(Of Integer))
            With obj
                .Name = "Test"
                .SetValue(42)
            End With
        End Sub

        ' Test exit and continue
        Public Sub TestExitContinue()
            For i As Integer = 1 To 10
                If i = 5 Then
                    Continue For
                End If
                If i = 8 Then
                    Exit For
                End If
                Console.WriteLine(i)
            Next
        End Sub

        ' Test expressions
        Public Function TestExpressions(ByVal a As Integer, ByVal b As Integer) As Integer
            Dim sum As Integer = a + b
            Dim product As Integer = a * b
            Dim negative As Integer = -a
            Dim notResult As Boolean = Not (a > b)
            Dim result As Integer = 0
            result = sum + product
            Dim obj As Object = "test"
            If TypeOf obj Is String Then
                Console.WriteLine("obj is a string")
            End If
            Return result
        End Function

        ' Test simple function
        Public Function TestSimple(ByVal numbers As List(Of Integer)) As List(Of Integer)
            Return numbers
        End Function
    End Class

    ' Test interface
    Public Interface ITestInterface
        Sub TestMethod()
    End Interface

    ' Test structure
    Public Structure TestStructure
        Public X As Integer
        Public Y As Integer
    End Structure

    ' Test enum
    Public Enum TestEnum
        Value1
        Value2
        Value3 = 10
    End Enum

    ' Test module
    Public Module TestModule
        Public Sub ModuleMethod()
            Console.WriteLine("Module method called")
        End Sub
    End Module
End Namespace
"#;

    let result = parse(source);
    assert!(result.is_ok(), "Parsing failed: {:?}", result.err());
}
