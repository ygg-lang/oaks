-- Ada test file
with Ada.Text_IO; use Ada.Text_IO;

procedure Hello_World is
begin
    Put_Line("Hello, World!");
end Hello_World;

-- Package specification
package Math_Operations is
    type Number is range -1000 .. 1000;
    function Add(A, B : Number) return Number;
    function Subtract(A, B : Number) return Number;
end Math_Operations;

-- Package body
package body Math_Operations is
    function Add(A, B : Number) return Number is
    begin
        return A + B;
    end Add;
    
    function Subtract(A, B : Number) return Number is
    begin
        return A - B;
    end Subtract;
end Math_Operations;