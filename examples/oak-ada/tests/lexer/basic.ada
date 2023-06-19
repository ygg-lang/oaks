-- Comprehensive Ada test file for lexer testing

-- Single-line comment

-- Multi-line comments are not standard in Ada, but block comments can be simulated
-- by multiple single-line comments or by using pragmas for documentation.

with Ada.Text_IO;
with Ada.Integer_Text_IO;
with Ada.Float_Text_IO;
use Ada.Text_IO;

procedure Ada_Features_Test is

   -- Basic Data Types
   type My_Integer is range -1_000_000 .. 1_000_000;
   type My_Float is digits 7 range -1.0E9 .. 1.0E9;
   type My_Boolean is (False, True);
   type My_Character is ('A' .. 'Z');
   type My_String is array (Positive range <>) of Character;

   -- Numeric Literals
   Decimal_Int    : constant Integer := 123_456;
   Based_Int      : constant Integer := 16#FF#;
   Real_Literal   : constant Float   := 3.141_59;
   Exp_Real       : constant Float   := 6.022E23;

   -- String and Character Literals
   Greeting       : constant String := "Hello, Ada!";
   First_Char     : constant Character := 'X';

   -- Variables
   Count          : My_Integer := 0;
   Temperature    : My_Float   := 25.5;
   Is_Active      : My_Boolean := True;
   Name           : My_String(1 .. 5) := "Alice";

   -- Constants
   Max_Value      : constant Integer := 100;
   Pi             : constant Float   := 3.1415926535;

   -- Arrays
   type Int_Array is array (1 .. 5) of Integer;
   Scores         : Int_Array := (10, 20, 30, 40, 50);
   Matrix         : array (1 .. 2, 1 .. 2) of Float := ((1.0, 2.0), (3.0, 4.0));

   -- Records
   type Point is record
      X : Float;
      Y : Float;
   end record;
   Origin         : Point := (X => 0.0, Y => 0.0);
   Current_Pos    : Point;

   -- Access Types (Pointers)
   type Node;
   type Node_Access is access Node;
   type Node is record
      Value : Integer;
      Next  : Node_Access;
   end record;
   Head           : Node_Access;

   -- Subprograms (Procedures and Functions)
   function Add (Left, Right : in Integer) return Integer is
   begin
      return Left + Right;
   end Add;

   procedure Display_Message (Message : in String) is
   begin
      Put_Line (Message);
   end Display_Message;

   -- Control Structures
   procedure Test_Control_Structures is
      X : Integer := 10;
      Y : Integer := 5;
   begin
      -- If-Then-Else
      if X > Y then
         Display_Message ("X is greater than Y");
      elsif X < Y then
         Display_Message ("X is less than Y");
      else
         Display_Message ("X is equal to Y");
      end if;

      -- Loop
      loop
         Count := Count + 1;
         exit when Count >= 3;
      end loop;
      Display_Message ("Loop finished. Count: " & Integer'Image(Count));

      -- While Loop
      while Y > 0 loop
         Display_Message ("While loop iteration. Y: " & Integer'Image(Y));
         Y := Y - 1;
      end loop;

      -- For Loop
      for I in 1 .. 3 loop
         Display_Message ("For loop iteration. I: " & Integer'Image(I));
      end loop;
   end Test_Control_Structures;

   -- Exceptions
   procedure Test_Exceptions is
      Denominator : Integer := 0;
   begin
      Put_Line ("Testing exceptions...");
      declare
         Result : Integer;
      begin
         Result := 10 / Denominator; -- This will raise Constraint_Error
         Put_Line ("Result: " & Integer'Image(Result));
      exception
         when Constraint_Error =>
            Put_Line ("Caught Constraint_Error: Division by zero");
         when others =>
            Put_Line ("Caught other exception");
      end;
   end Test_Exceptions;

   -- Generics
   generic
      type Item is private;
      with function "+" (Left, Right : Item) return Item is <>;
   package Generic_Sum is
      function Sum_Two (A, B : Item) return Item;
   end Generic_Sum;

   package body Generic_Sum is
      function Sum_Two (A, B : Item) return Item is
      begin
         return A + B;
      end Sum_Two;
   end Generic_Sum;

   package Integer_Sum is new Generic_Sum (Item => Integer, "+" => Add);

   -- Tasks (Concurrency)
   task type My_Task;
   task body My_Task is
   begin
      Display_Message ("My_Task is running");
      delay 0.1; -- Wait for 100 milliseconds
      Display_Message ("My_Task finished");
   end My_Task;

   Task1 : My_Task;

   -- Protected Objects
   protected type Shared_Counter is
      entry Increment;
      entry Decrement;
      function Get_Value return Integer;
   private
      Current_Value : Integer := 0;
   end Shared_Counter;

   protected body Shared_Counter is
      entry Increment when True is
      begin
         Current_Value := Current_Value + 1;
      end Increment;

      entry Decrement when True is
      begin
         Current_Value := Current_Value - 1;
      end Decrement;

      function Get_Value return Integer is
      begin
         return Current_Value;
      end Get_Value;
   end Shared_Counter;

   Counter : Shared_Counter;

begin -- Ada_Features_Test
   Display_Message ("Starting Ada Features Test...");

   -- Test basic types and variables
   Ada.Integer_Text_IO.Put (Decimal_Int);
   Put_Line ("");
   Ada.Float_Text_IO.Put (Real_Literal);
   Put_Line ("");
   Display_Message (Greeting);
   Display_Message ("Is_Active: " & Boolean'Image(Is_Active));

   -- Test records
   Current_Pos := (X => 10.0, Y => 20.0);
   Display_Message ("Current_Pos: (" & Float'Image(Current_Pos.X) & ", " & Float'Image(Current_Pos.Y) & ")");

   -- Test subprograms
   Display_Message ("Add(5, 3) = " & Integer'Image(Add(5, 3)));

   -- Test control structures
   Test_Control_Structures;

   -- Test exceptions
   Test_Exceptions;

   -- Test generics
   Display_Message ("Integer_Sum.Sum_Two(10, 20) = " & Integer'Image(Integer_Sum.Sum_Two(10, 20)));

   -- Test tasks
   -- Task1 will start automatically
   delay 0.2; -- Give tasks time to run

   -- Test protected objects
   Counter.Increment;
   Counter.Increment;
   Counter.Decrement;
   Display_Message ("Counter value: " & Integer'Image(Counter.Get_Value));

   Display_Message ("Ada Features Test Finished.");

end Ada_Features_Test;