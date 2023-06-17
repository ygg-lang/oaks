{ Pascal Test File - Comprehensive Syntax Coverage }
{ This file tests various Pascal syntax elements for lexer testing }

program ComprehensivePascalTest;

{$mode objfpc}{$H+}
{$IFDEF FPC}
  {$LONGSTRINGS ON}
  {$ASSERTIONS ON}
{$ENDIF}

uses
  SysUtils, Classes, Math, StrUtils, DateUtils, Variants, 
  {$IFDEF WINDOWS}
  Windows,
  {$ENDIF}
  {$IFDEF UNIX}
  Unix, BaseUnix,
  {$ENDIF}
  Contnrs, IniFiles, RegExpr;

{ Constants }
const
  PI = 3.14159265359;
  MAX_SIZE = 1000;
  VERSION = '1.0.0';
  DEBUG = True;
  
  { String constants }
  GREETING = 'Hello, Pascal!';
  MULTILINE_TEXT = 'This is a ' +
                   'multi-line ' +
                   'string constant';
  
  { Character constants }
  TAB_CHAR = #9;
  NEWLINE_CHAR = #10;
  CARRIAGE_RETURN = #13;
  
  { Numeric constants }
  HEX_VALUE = $FF;
  OCTAL_VALUE = &377;
  BINARY_VALUE = %11111111;
  FLOAT_VALUE = 3.14E+2;
  SCIENTIFIC_NOTATION = 1.23E-4;

{ Type definitions }
type
  { Enumerated types }
  TColor = (clRed, clGreen, clBlue, clYellow, clPurple, clOrange);
  TDirection = (dirNorth, dirSouth, dirEast, dirWest);
  TLogLevel = (llTrace, llDebug, llInfo, llWarn, llError, llFatal);
  
  { Subrange types }
  TPercentage = 0..100;
  TDayOfMonth = 1..31;
  TUpperCase = 'A'..'Z';
  
  { Set types }
  TColorSet = set of TColor;
  TCharSet = set of Char;
  TIntegerSet = set of 0..255;
  
  { Array types }
  TIntArray = array[1..10] of Integer;
  TMatrix = array[1..3, 1..3] of Real;
  TDynamicArray = array of Integer;
  TStringArray = array of String;
  
  { Record types }
  TPoint = record
    X, Y: Real;
  end;
  
  TPoint3D = record
    X, Y, Z: Real;
  end;
  
  TPerson = record
    FirstName: String[50];
    LastName: String[50];
    Age: Integer;
    BirthDate: TDateTime;
    Active: Boolean;
    Salary: Currency;
  end;
  
  { Variant record }
  TShape = record
    Center: TPoint;
    case ShapeType: Integer of
      1: (Radius: Real);                    { Circle }
      2: (Width, Height: Real);             { Rectangle }
      3: (SideA, SideB, SideC: Real);       { Triangle }
  end;
  
  { Object types (old-style OOP) }
  TCounter = object
    Value: Integer;
    procedure Init;
    procedure Increment;
    procedure Decrement;
    function GetValue: Integer;
  end;
  
  { Class types (modern OOP) }
  TAnimal = class
  private
    FName: String;
    FAge: Integer;
  protected
    procedure SetName(const AName: String); virtual;
    procedure SetAge(const AAge: Integer); virtual;
  public
    constructor Create(const AName: String; AAge: Integer);
    destructor Destroy; override;
    
    property Name: String read FName write SetName;
    property Age: Integer read FAge write SetAge;
    
    procedure MakeSound; virtual; abstract;
    procedure Move; virtual;
    function ToString: String; virtual;
  end;
  
  TDog = class(TAnimal)
  private
    FBreed: String;
  public
    constructor Create(const AName: String; AAge: Integer; const ABreed: String);
    
    property Breed: String read FBreed write FBreed;
    
    procedure MakeSound; override;
    procedure Fetch;
  end;
  
  TCat = class(TAnimal)
  private
    FIndoor: Boolean;
  public
    constructor Create(const AName: String; AAge: Integer; AIndoor: Boolean);
    
    property Indoor: Boolean read FIndoor write FIndoor;
    
    procedure MakeSound; override;
    procedure Purr;
  end;
  
  { Interface types }
  IDrawable = interface
    ['{12345678-1234-1234-1234-123456789012}']
    procedure Draw;
    function GetArea: Real;
  end;
  
  IMovable = interface
    ['{87654321-4321-4321-4321-210987654321}']
    procedure MoveTo(X, Y: Real);
    function GetPosition: TPoint;
  end;
  
  { Class implementing interfaces }
  TCircle = class(TInterfacedObject, IDrawable, IMovable)
  private
    FCenter: TPoint;
    FRadius: Real;
  public
    constructor Create(ACenter: TPoint; ARadius: Real);
    
    { IDrawable }
    procedure Draw;
    function GetArea: Real;
    
    { IMovable }
    procedure MoveTo(X, Y: Real);
    function GetPosition: TPoint;
    
    property Center: TPoint read FCenter;
    property Radius: Real read FRadius write FRadius;
  end;
  
  { Pointer types }
  PInteger = ^Integer;
  PPoint = ^TPoint;
  PNode = ^TNode;
  
  { Linked list node }
  TNode = record
    Data: Integer;
    Next: PNode;
  end;
  
  { Procedural types }
  TIntegerFunction = function(X: Integer): Integer;
  TCompareFunction = function(A, B: Integer): Integer;
  TNotifyEvent = procedure(Sender: TObject);
  
  { Generic types (FPC) }
  {$IFDEF FPC}
  generic TList<T> = class
  private
    FItems: array of T;
    FCount: Integer;
  public
    procedure Add(const Item: T);
    function Get(Index: Integer): T;
    procedure Clear;
    property Count: Integer read FCount;
    property Items[Index: Integer]: T read Get; default;
  end;
  
  TIntegerList = specialize TList<Integer>;
  TStringList = specialize TList<String>;
  {$ENDIF}

{ Global variables }
var
  GlobalCounter: Integer;
  GlobalMessage: String;
  GlobalArray: TIntArray;
  GlobalMatrix: TMatrix;
  GlobalColors: TColorSet;
  GlobalPersons: array of TPerson;

{ Forward declarations }
procedure ProcessData(const Data: String); forward;
function CalculateSum(const Numbers: array of Integer): Integer; forward;

{ Object method implementations }
procedure TCounter.Init;
begin
  Value := 0;
end;

procedure TCounter.Increment;
begin
  Inc(Value);
end;

procedure TCounter.Decrement;
begin
  Dec(Value);
end;

function TCounter.GetValue: Integer;
begin
  Result := Value;
end;

{ Class method implementations }
constructor TAnimal.Create(const AName: String; AAge: Integer);
begin
  inherited Create;
  FName := AName;
  FAge := AAge;
end;

destructor TAnimal.Destroy;
begin
  WriteLn('Animal ', FName, ' destroyed');
  inherited Destroy;
end;

procedure TAnimal.SetName(const AName: String);
begin
  FName := AName;
end;

procedure TAnimal.SetAge(const AAge: Integer);
begin
  if (AAge >= 0) and (AAge <= 100) then
    FAge := AAge;
end;

procedure TAnimal.Move;
begin
  WriteLn(FName, ' is moving');
end;

function TAnimal.ToString: String;
begin
  Result := Format('%s (age %d)', [FName, FAge]);
end;

{ TDog implementation }
constructor TDog.Create(const AName: String; AAge: Integer; const ABreed: String);
begin
  inherited Create(AName, AAge);
  FBreed := ABreed;
end;

procedure TDog.MakeSound;
begin
  WriteLn(Name, ' the ', Breed, ' says: Woof!');
end;

procedure TDog.Fetch;
begin
  WriteLn(Name, ' is fetching the ball');
end;

{ TCat implementation }
constructor TCat.Create(const AName: String; AAge: Integer; AIndoor: Boolean);
begin
  inherited Create(AName, AAge);
  FIndoor := AIndoor;
end;

procedure TCat.MakeSound;
begin
  WriteLn(Name, ' says: Meow!');
end;

procedure TCat.Purr;
begin
  WriteLn(Name, ' is purring contentedly');
end;

{ TCircle implementation }
constructor TCircle.Create(ACenter: TPoint; ARadius: Real);
begin
  inherited Create;
  FCenter := ACenter;
  FRadius := ARadius;
end;

procedure TCircle.Draw;
begin
  WriteLn(Format('Drawing circle at (%.2f, %.2f) with radius %.2f', 
                 [FCenter.X, FCenter.Y, FRadius]));
end;

function TCircle.GetArea: Real;
begin
  Result := PI * FRadius * FRadius;
end;

procedure TCircle.MoveTo(X, Y: Real);
begin
  FCenter.X := X;
  FCenter.Y := Y;
end;

function TCircle.GetPosition: TPoint;
begin
  Result := FCenter;
end;

{ Generic class implementation }
{$IFDEF FPC}
procedure TList.Add(const Item: T);
begin
  SetLength(FItems, FCount + 1);
  FItems[FCount] := Item;
  Inc(FCount);
end;

function TList.Get(Index: Integer): T;
begin
  if (Index >= 0) and (Index < FCount) then
    Result := FItems[Index]
  else
    raise Exception.CreateFmt('Index %d out of bounds', [Index]);
end;

procedure TList.Clear;
begin
  SetLength(FItems, 0);
  FCount := 0;
end;
{$ENDIF}

{ Utility functions and procedures }
function Add(A, B: Integer): Integer;
begin
  Result := A + B;
end;

function Subtract(A, B: Integer): Integer;
begin
  Result := A - B;
end;

function Multiply(A, B: Integer): Integer;
begin
  Result := A * B;
end;

function Divide(A, B: Real): Real;
begin
  if B <> 0 then
    Result := A / B
  else
    raise Exception.Create('Division by zero');
end;

{ Function with default parameters }
function Power(Base: Real; Exponent: Real = 2.0): Real;
begin
  Result := Math.Power(Base, Exponent);
end;

{ Overloaded functions }
function Max(A, B: Integer): Integer; overload;
begin
  if A > B then
    Result := A
  else
    Result := B;
end;

function Max(A, B: Real): Real; overload;
begin
  if A > B then
    Result := A
  else
    Result := B;
end;

function Max(A, B, C: Integer): Integer; overload;
begin
  Result := Max(Max(A, B), C);
end;

{ Procedure with var parameters }
procedure Swap(var A, B: Integer);
var
  Temp: Integer;
begin
  Temp := A;
  A := B;
  B := Temp;
end;

{ Procedure with out parameters }
procedure DivMod(Dividend, Divisor: Integer; out Quotient, Remainder: Integer);
begin
  Quotient := Dividend div Divisor;
  Remainder := Dividend mod Divisor;
end;

{ Procedure with const parameters }
procedure PrintArray(const Arr: array of Integer);
var
  I: Integer;
begin
  Write('[');
  for I := Low(Arr) to High(Arr) do
  begin
    Write(Arr[I]);
    if I < High(Arr) then
      Write(', ');
  end;
  WriteLn(']');
end;

{ Recursive function }
function Factorial(N: Integer): Int64;
begin
  if N <= 1 then
    Result := 1
  else
    Result := N * Factorial(N - 1);
end;

{ Function returning a function }
function GetMathFunction(Operation: Char): TIntegerFunction;
begin
  case Operation of
    '+': Result := @Add;
    '-': Result := @Subtract;
    '*': Result := @Multiply;
  else
    Result := nil;
  end;
end;

{ String manipulation functions }
function ReverseString(const S: String): String;
var
  I: Integer;
begin
  Result := '';
  for I := Length(S) downto 1 do
    Result := Result + S[I];
end;

function CountWords(const S: String): Integer;
var
  Words: TStringArray;
begin
  Words := SplitString(Trim(S), ' ');
  Result := Length(Words);
end;

function IsValidEmail(const Email: String): Boolean;
var
  RegEx: TRegExpr;
begin
  RegEx := TRegExpr.Create;
  try
    RegEx.Expression := '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$';
    Result := RegEx.Exec(Email);
  finally
    RegEx.Free;
  end;
end;

{ Array manipulation procedures }
procedure InitializeArray(var Arr: TIntArray);
var
  I: Integer;
begin
  for I := Low(Arr) to High(Arr) do
    Arr[I] := I;
end;

procedure SortArray(var Arr: array of Integer);
var
  I, J, Temp: Integer;
begin
  { Bubble sort }
  for I := Low(Arr) to High(Arr) - 1 do
    for J := Low(Arr) to High(Arr) - I - 1 do
      if Arr[J] > Arr[J + 1] then
      begin
        Temp := Arr[J];
        Arr[J] := Arr[J + 1];
        Arr[J + 1] := Temp;
      end;
end;

function BinarySearch(const Arr: array of Integer; Value: Integer): Integer;
var
  Left, Right, Mid: Integer;
begin
  Left := Low(Arr);
  Right := High(Arr);
  Result := -1;
  
  while Left <= Right do
  begin
    Mid := (Left + Right) div 2;
    if Arr[Mid] = Value then
    begin
      Result := Mid;
      Break;
    end
    else if Arr[Mid] < Value then
      Left := Mid + 1
    else
      Right := Mid - 1;
  end;
end;

{ Matrix operations }
procedure InitializeMatrix(var Matrix: TMatrix);
var
  I, J: Integer;
begin
  for I := 1 to 3 do
    for J := 1 to 3 do
      Matrix[I, J] := I * J;
end;

procedure PrintMatrix(const Matrix: TMatrix);
var
  I, J: Integer;
begin
  for I := 1 to 3 do
  begin
    for J := 1 to 3 do
      Write(Format('%8.2f', [Matrix[I, J]]));
    WriteLn;
  end;
end;

{ Set operations }
procedure DemonstrateSetOperations;
var
  Set1, Set2, UnionSet, IntersectionSet: TColorSet;
begin
  Set1 := [clRed, clGreen, clBlue];
  Set2 := [clBlue, clYellow, clPurple];
  
  UnionSet := Set1 + Set2;
  IntersectionSet := Set1 * Set2;
  
  WriteLn('Set operations:');
  WriteLn('Set1 contains Red: ', clRed in Set1);
  WriteLn('Set2 contains Red: ', clRed in Set2);
  WriteLn('Union has ', SizeOf(UnionSet) * 8, ' possible elements');
  WriteLn('Intersection has Blue: ', clBlue in IntersectionSet);
end;

{ Record operations }
function CreatePerson(const FirstName, LastName: String; Age: Integer): TPerson;
begin
  with Result do
  begin
    FirstName := FirstName;
    LastName := LastName;
    Age := Age;
    BirthDate := Now - Age * 365.25;
    Active := True;
    Salary := 50000.0;
  end;
end;

procedure PrintPerson(const Person: TPerson);
begin
  with Person do
    WriteLn(Format('%s %s, Age: %d, Salary: %m', 
                   [FirstName, LastName, Age, Salary]));
end;

{ Pointer operations }
procedure DemonstratePointers;
var
  P: PInteger;
  Value: Integer;
begin
  Value := 42;
  P := @Value;
  
  WriteLn('Value: ', Value);
  WriteLn('Pointer value: ', P^);
  WriteLn('Pointer address: ', IntPtr(P));
  
  P^ := 100;
  WriteLn('Modified value: ', Value);
end;

{ Linked list operations }
procedure AddNode(var Head: PNode; Data: Integer);
var
  NewNode: PNode;
begin
  New(NewNode);
  NewNode^.Data := Data;
  NewNode^.Next := Head;
  Head := NewNode;
end;

procedure PrintList(Head: PNode);
var
  Current: PNode;
begin
  Current := Head;
  Write('List: ');
  while Current <> nil do
  begin
    Write(Current^.Data);
    Current := Current^.Next;
    if Current <> nil then
      Write(' -> ');
  end;
  WriteLn;
end;

procedure FreeList(var Head: PNode);
var
  Current, Next: PNode;
begin
  Current := Head;
  while Current <> nil do
  begin
    Next := Current^.Next;
    Dispose(Current);
    Current := Next;
  end;
  Head := nil;
end;

{ File operations }
procedure DemonstrateFileOperations;
var
  F: TextFile;
  Line: String;
  FileName: String;
begin
  FileName := 'test_output.txt';
  
  { Write to file }
  AssignFile(F, FileName);
  Rewrite(F);
  try
    WriteLn(F, 'Hello, Pascal!');
    WriteLn(F, 'This is a test file.');
    WriteLn(F, 'Generated at: ', DateTimeToStr(Now));
  finally
    CloseFile(F);
  end;
  
  { Read from file }
  if FileExists(FileName) then
  begin
    AssignFile(F, FileName);
    Reset(F);
    try
      WriteLn('File contents:');
      while not Eof(F) do
      begin
        ReadLn(F, Line);
        WriteLn('  ', Line);
      end;
    finally
      CloseFile(F);
    end;
    
    { Clean up }
    DeleteFile(FileName);
  end;
end;

{ Exception handling }
procedure DemonstrateTryExcept;
var
  A, B, Result: Integer;
begin
  A := 10;
  B := 0;
  
  try
    Result := A div B;
    WriteLn('Result: ', Result);
  except
    on E: EDivByZero do
      WriteLn('Division by zero error: ', E.Message);
    on E: Exception do
      WriteLn('General error: ', E.Message);
  end;
end;

procedure DemonstrateTryFinally;
var
  F: TextFile;
begin
  AssignFile(F, 'temp.txt');
  try
    Rewrite(F);
    WriteLn(F, 'Temporary data');
    { Simulate an error }
    raise Exception.Create('Simulated error');
  finally
    CloseFile(F);
    if FileExists('temp.txt') then
      DeleteFile('temp.txt');
    WriteLn('Cleanup completed');
  end;
end;

{ Control structures }
procedure DemonstrateControlStructures;
var
  I, J: Integer;
  Ch: Char;
  Colors: TColorSet;
  Numbers: array[1..5] of Integer;
begin
  WriteLn('=== Control Structures ===');
  
  { If-then-else }
  I := 15;
  if I > 10 then
    WriteLn('I is greater than 10')
  else if I > 5 then
    WriteLn('I is greater than 5')
  else
    WriteLn('I is 5 or less');
  
  { Case statement }
  Ch := 'B';
  case Ch of
    'A': WriteLn('Letter A');
    'B': WriteLn('Letter B');
    'C'..'Z': WriteLn('Letter C through Z');
  else
    WriteLn('Not a capital letter');
  end;
  
  { For loops }
  Write('For loop (1 to 5): ');
  for I := 1 to 5 do
    Write(I, ' ');
  WriteLn;
  
  Write('For loop (5 downto 1): ');
  for I := 5 downto 1 do
    Write(I, ' ');
  WriteLn;
  
  { While loop }
  I := 1;
  Write('While loop: ');
  while I <= 5 do
  begin
    Write(I, ' ');
    Inc(I);
  end;
  WriteLn;
  
  { Repeat-until loop }
  I := 1;
  Write('Repeat-until loop: ');
  repeat
    Write(I, ' ');
    Inc(I);
  until I > 5;
  WriteLn;
  
  { For-in loop (FPC) }
  {$IFDEF FPC}
  Numbers[1] := 10;
  Numbers[2] := 20;
  Numbers[3] := 30;
  Numbers[4] := 40;
  Numbers[5] := 50;
  
  Write('For-in loop: ');
  for I in Numbers do
    Write(I, ' ');
  WriteLn;
  {$ENDIF}
  
  { Nested loops with labels }
  WriteLn('Nested loops with break:');
  for I := 1 to 3 do
  begin
    for J := 1 to 3 do
    begin
      if (I = 2) and (J = 2) then
        goto LoopEnd;
      Write('(', I, ',', J, ') ');
    end;
    WriteLn;
  end;
  
  LoopEnd:
  WriteLn('Broke out of nested loops');
end;

{ Advanced features }
procedure DemonstrateAdvancedFeatures;
var
  Animals: array of TAnimal;
  Dog: TDog;
  Cat: TCat;
  Circle: TCircle;
  Drawable: IDrawable;
  I: Integer;
  {$IFDEF FPC}
  IntList: TIntegerList;
  {$ENDIF}
begin
  WriteLn('=== Advanced Features ===');
  
  { Polymorphism }
  SetLength(Animals, 2);
  Animals[0] := TDog.Create('Buddy', 3, 'Golden Retriever');
  Animals[1] := TCat.Create('Whiskers', 2, True);
  
  for I := 0 to High(Animals) do
  begin
    Animals[I].MakeSound;
    Animals[I].Move;
  end;
  
  { Type casting and is/as operators }
  for I := 0 to High(Animals) do
  begin
    if Animals[I] is TDog then
    begin
      Dog := Animals[I] as TDog;
      Dog.Fetch;
    end
    else if Animals[I] is TCat then
    begin
      Cat := Animals[I] as TCat;
      Cat.Purr;
    end;
  end;
  
  { Interface usage }
  Circle := TCircle.Create(Point(100, 100), 50);
  Drawable := Circle;
  Drawable.Draw;
  WriteLn('Circle area: ', Drawable.GetArea:0:2);
  
  { Generic collections }
  {$IFDEF FPC}
  IntList := TIntegerList.Create;
  try
    IntList.Add(10);
    IntList.Add(20);
    IntList.Add(30);
    
    WriteLn('Generic list contents:');
    for I := 0 to IntList.Count - 1 do
      WriteLn('  Item ', I, ': ', IntList[I]);
  finally
    IntList.Free;
  end;
  {$ENDIF}
  
  { Cleanup }
  for I := 0 to High(Animals) do
    Animals[I].Free;
  Circle.Free;
end;

{ Forward declaration implementations }
procedure ProcessData(const Data: String);
begin
  WriteLn('Processing data: ', Data);
  WriteLn('Data length: ', Length(Data));
  WriteLn('Word count: ', CountWords(Data));
  WriteLn('Reversed: ', ReverseString(Data));
end;

function CalculateSum(const Numbers: array of Integer): Integer;
var
  I: Integer;
begin
  Result := 0;
  for I := Low(Numbers) to High(Numbers) do
    Result := Result + Numbers[I];
end;

{ Main program }
begin
  WriteLn('=== Pascal Comprehensive Test ===');
  WriteLn('Version: ', VERSION);
  WriteLn('Debug mode: ', DEBUG);
  WriteLn;
  
  { Initialize global variables }
  GlobalCounter := 0;
  GlobalMessage := GREETING;
  GlobalColors := [clRed, clGreen, clBlue];
  
  { Test basic operations }
  WriteLn('Basic arithmetic:');
  WriteLn('5 + 3 = ', Add(5, 3));
  WriteLn('5 - 3 = ', Subtract(5, 3));
  WriteLn('5 * 3 = ', Multiply(5, 3));
  WriteLn('5.0 / 2.0 = ', Divide(5.0, 2.0):0:2);
  WriteLn('2^3 = ', Power(2, 3):0:0);
  WriteLn('5! = ', Factorial(5));
  WriteLn;
  
  { Test overloaded functions }
  WriteLn('Max(10, 20) = ', Max(10, 20));
  WriteLn('Max(3.14, 2.71) = ', Max(3.14, 2.71):0:2);
  WriteLn('Max(5, 10, 3) = ', Max(5, 10, 3));
  WriteLn;
  
  { Test string operations }
  ProcessData('Hello, Pascal World!');
  WriteLn('Email validation:');
  WriteLn('  test@example.com: ', IsValidEmail('test@example.com'));
  WriteLn('  invalid-email: ', IsValidEmail('invalid-email'));
  WriteLn;
  
  { Test arrays }
  InitializeArray(GlobalArray);
  Write('Initialized array: ');
  PrintArray(GlobalArray);
  
  SortArray(GlobalArray);
  Write('Sorted array: ');
  PrintArray(GlobalArray);
  
  WriteLn('Binary search for 5: ', BinarySearch(GlobalArray, 5));
  WriteLn;
  
  { Test matrix }
  InitializeMatrix(GlobalMatrix);
  WriteLn('Matrix:');
  PrintMatrix(GlobalMatrix);
  WriteLn;
  
  { Test sets }
  DemonstrateSetOperations;
  WriteLn;
  
  { Test records }
  SetLength(GlobalPersons, 2);
  GlobalPersons[0] := CreatePerson('John', 'Doe', 30);
  GlobalPersons[1] := CreatePerson('Jane', 'Smith', 25);
  
  WriteLn('Persons:');
  PrintPerson(GlobalPersons[0]);
  PrintPerson(GlobalPersons[1]);
  WriteLn;
  
  { Test pointers }
  DemonstratePointers;
  WriteLn;
  
  { Test linked list }
  var
    Head: PNode;
    I: Integer;
  begin
    Head := nil;
    for I := 1 to 5 do
      AddNode(Head, I * 10);
    
    PrintList(Head);
    FreeList(Head);
  end;
  WriteLn;
  
  { Test control structures }
  DemonstrateControlStructures;
  WriteLn;
  
  { Test file operations }
  DemonstrateFileOperations;
  WriteLn;
  
  { Test exception handling }
  WriteLn('Exception handling:');
  DemonstrateTryExcept;
  
  try
    DemonstrateTryFinally;
  except
    on E: Exception do
      WriteLn('Caught exception: ', E.Message);
  end;
  WriteLn;
  
  { Test advanced features }
  DemonstrateAdvancedFeatures;
  WriteLn;
  
  { Test object }
  var
    Counter: TCounter;
  begin
    Counter.Init;
    Counter.Increment;
    Counter.Increment;
    WriteLn('Counter value: ', Counter.GetValue);
  end;
  
  WriteLn('=== Test completed ===');
  
  { Wait for user input }
  WriteLn('Press Enter to exit...');
  ReadLn;
end.