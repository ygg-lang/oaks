unit DelphiTestFile;

interface

uses
  System.SysUtils, System.Classes, System.Generics.Collections,
  System.DateUtils, System.Math, System.StrUtils, System.IOUtils,
  Vcl.Graphics, Vcl.Controls, Vcl.Forms, Vcl.Dialogs, Vcl.StdCtrls,
  Vcl.ExtCtrls, Vcl.ComCtrls, Vcl.Menus, Vcl.ToolWin, Vcl.ActnList,
  Vcl.ImgList, Vcl.Grids, Vcl.DBGrids, Vcl.DBCtrls, Vcl.Mask,
  Data.DB, Data.Win.ADODB, FireDAC.Stan.Intf, FireDAC.Stan.Option,
  FireDAC.Stan.Error, FireDAC.UI.Intf, FireDAC.Phys.Intf, FireDAC.Stan.Def,
  FireDAC.Stan.Pool, FireDAC.Stan.Async, FireDAC.Phys, FireDAC.Phys.MSAcc,
  FireDAC.Phys.MSAccDef, FireDAC.VCLUI.Wait, FireDAC.Stan.Param,
  FireDAC.DatS, FireDAC.DApt.Intf, FireDAC.DApt, FireDAC.Comp.DataSet,
  FireDAC.Comp.Client;

type
  // Basic types and enumerations
  TUserRole = (urGuest, urUser, urAdmin, urSupervisor);
  TAccessLevel = (alNone = 0, alRead = 1, alWrite = 2, alFull = 3);
  TStatusCode = 100..599;
  TCustomerID = type Integer;
  TPercentage = 0..100;
  TFileName = type string;
  TEmailAddress = type string;

  // Record types
  TPerson = record
    FirstName: string;
    LastName: string;
    BirthDate: TDateTime;
    Age: Integer;
    Email: TEmailAddress;
    Active: Boolean;
    Role: TUserRole;
    AccessLevel: TAccessLevel;
  end;

  TAddress = record
    Street: string;
    City: string;
    State: string;
    ZipCode: string;
    Country: string;
    procedure Clear;
    function FullAddress: string;
  end;

  // Class types
  TCustomer = class(TObject)
  private
    FID: TCustomerID;
    FName: string;
    FEmail: string;
    FPhone: string;
    FAddress: TAddress;
    FBalance: Currency;
    FCreated: TDateTime;
    FModified: TDateTime;
    FActive: Boolean;
    FDiscount: TPercentage;
    FTags: TStringList;
    procedure SetBalance(const Value: Currency);
    procedure SetEmail(const Value: string);
    function GetFullName: string;
  protected
    procedure DoModify; virtual;
    function ValidateEmail(const Email: string): Boolean;
  public
    constructor Create;
    destructor Destroy; override;
    procedure Assign(Source: TCustomer); virtual;
    function Clone: TCustomer;
    procedure LoadFromDataset(Dataset: TDataSet);
    procedure SaveToDataset(Dataset: TDataSet);
    function CalculateDiscount(Amount: Currency): Currency;
    class function GetCustomerByID(ID: TCustomerID): TCustomer; static;
    property ID: TCustomerID read FID write FID;
    property Name: string read FName write FName;
    property Email: string read FEmail write SetEmail;
    property Phone: string read FPhone write FPhone;
    property Address: TAddress read FAddress write FAddress;
    property Balance: Currency read FBalance write SetBalance;
    property Created: TDateTime read FCreated write FCreated;
    property Modified: TDateTime read FModified;
    property Active: Boolean read FActive write FActive;
    property Discount: TPercentage read FDiscount write FDiscount;
    property FullName: string read GetFullName;
    property Tags: TStringList read FTags;
  end;

  // Generic class
  TGenericList<T> = class(TObject)
  private
    FItems: array of T;
    FCount: Integer;
    FCapacity: Integer;
    procedure Grow;
    function GetItem(Index: Integer): T;
    procedure SetItem(Index: Integer; const Value: T);
  public
    constructor Create;
    destructor Destroy; override;
    procedure Add(const Item: T);
    procedure Insert(Index: Integer; const Item: T);
    procedure Delete(Index: Integer);
    function Remove(const Item: T): Integer;
    function IndexOf(const Item: T): Integer;
    procedure Clear;
    procedure Sort(Compare: TComparer<T>); overload;
    procedure Sort; overload;
    property Count: Integer read FCount;
    property Items[Index: Integer]: T read GetItem write SetItem; default;
    property Capacity: Integer read FCapacity write FCapacity;
  end;

  // Interface definition
  ICustomerService = interface(IInterface)
    ['{B8F4D4E0-7A3C-4B9E-9F2D-1A5C7E9B8D3F}']
    function GetCustomer(ID: TCustomerID): TCustomer;
    procedure SaveCustomer(Customer: TCustomer);
    procedure DeleteCustomer(ID: TCustomerID);
    function GetAllCustomers: TArray<TCustomer>;
    function FindCustomers(const Filter: string): TArray<TCustomer>;
    function GetCustomerCount: Integer;
  end;

  // Class implementing interface
  TCustomerService = class(TInterfacedObject, ICustomerService)
  private
    FConnection: TFDConnection;
    FQuery: TFDQuery;
    procedure PrepareConnection;
    procedure ValidateCustomer(Customer: TCustomer);
  public
    constructor Create;
    destructor Destroy; override;
    function GetCustomer(ID: TCustomerID): TCustomer;
    procedure SaveCustomer(Customer: TCustomer);
    procedure DeleteCustomer(ID: TCustomerID);
    function GetAllCustomers: TArray<TCustomer>;
    function FindCustomers(const Filter: string): TArray<TCustomer>;
    function GetCustomerCount: Integer;
  end;

  // Helper class with class methods and class variables
  TMathHelper = class
  private
    class var FPI: Double;
    class var FEpsilon: Double;
    class constructor Create;
  public
    class function Factorial(N: Integer): Int64;
    class function Power(Base, Exponent: Double): Double;
    class function IsPrime(Number: Integer): Boolean;
    class function Fibonacci(N: Integer): Int64;
    class function GCD(A, B: Integer): Integer;
    class function LCM(A, B: Integer): Integer;
    class property PI: Double read FPI;
    class property Epsilon: Double read FEpsilon;
  end;

  // Form class
  TMainForm = class(TForm)
    MainMenu: TMainMenu;
    StatusBar: TStatusBar;
    ToolBar: TToolBar;
    ActionList: TActionList;
    ImageList: TImageList;
    PageControl: TPageControl;
    CustomerTab: TTabSheet;
    ReportTab: TTabSheet;
    CustomerGrid: TDBGrid;
    CustomerNavigator: TDBNavigator;
    FilterEdit: TEdit;
    FilterLabel: TLabel;
    SearchButton: TButton;
    ClearButton: TButton;
    ExportButton: TButton;
    PrintButton: TButton;
    procedure FormCreate(Sender: TObject);
    procedure FormDestroy(Sender: TObject);
    procedure SearchButtonClick(Sender: TObject);
    procedure ClearButtonClick(Sender: TObject);
    procedure ExportButtonClick(Sender: TObject);
    procedure PrintButtonClick(Sender: TObject);
    procedure FilterEditChange(Sender: TObject);
  private
    FCustomerService: ICustomerService;
    FCurrentCustomer: TCustomer;
    FFilterText: string;
    procedure InitializeComponents;
    procedure SetupDatabase;
    procedure LoadCustomers;
    procedure UpdateStatusBar;
    procedure ShowCustomerDetails(Customer: TCustomer);
    function ValidateForm: Boolean;
  protected
    procedure DoShow; override;
    procedure DoHide; override;
  public
    property CustomerService: ICustomerService read FCustomerService;
    property CurrentCustomer: TCustomer read FCurrentCustomer;
  end;

  // Thread class
  TBackgroundWorker = class(TThread)
  private
    FProgress: Integer;
    FMessage: string;
    FOnProgress: TNotifyEvent;
    FOnMessage: TNotifyEvent;
    procedure SetProgress(Value: Integer);
    procedure SetMessage(const Value: string);
  protected
    procedure Execute; override;
    procedure DoProgress;
    procedure DoMessage;
  public
    constructor Create(CreateSuspended: Boolean);
    property Progress: Integer read FProgress write SetProgress;
    property Message: string read FMessage write SetMessage;
    property OnProgress: TNotifyEvent read FOnProgress write FOnProgress;
    property OnMessage: TNotifyEvent read FOnMessage write FOnMessage;
  end;

  // Exception class
  ECustomerException = class(Exception)
  private
    FErrorCode: Integer;
    FCustomerID: TCustomerID;
  public
    constructor Create(const Msg: string; ErrorCode: Integer; CustomerID: TCustomerID);
    property ErrorCode: Integer read FErrorCode;
    property CustomerID: TCustomerID read FCustomerID;
  end;

  // Set type
  TCharSet = set of Char;
  TIntSet = set of 1..100;
  TDaySet = set of (Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday);

  // Array types
  TIntArray = array of Integer;
  TMatrix = array of array of Double;
  TCustomerArray = array of TCustomer;
  TFixedArray = array[1..10] of string;
  TMultiDimArray = array[1..5, 1..10] of Integer;

  // Pointer types
  PInteger = ^Integer;
  PCustomer = ^TCustomer;
  PPerson = ^TPerson;

  // Procedural types
  TNotifyInt = procedure(Value: Integer) of object;
  TCompareFunc = function(const A, B: string): Integer;
  TMathOperation = function(X, Y: Double): Double;
  TFilterFunc<T> = function(const Item: T): Boolean;

  // Const section
const
  APP_NAME = 'Delphi Test Application';
  VERSION = '1.0.0';
  MAX_CUSTOMERS = 10000;
  DEFAULT_DISCOUNT = 0.05;
  COMPANY_NAME = 'Test Company Inc.';
  COPYRIGHT_YEAR = 2024;
  PI_VALUE = 3.141592653589793;
  E_VALUE = 2.718281828459045;

  // Resource string
resourcestring
  SWelcomeMessage = 'Welcome to %s version %s';
  SErrorLoadingData = 'Error loading data: %s';
  SConfirmDelete = 'Are you sure you want to delete this customer?';
  SFileNotFound = 'File "%s" not found';
  SInvalidEmail = 'Invalid email address: %s';
  SSuccessMessage = 'Operation completed successfully';

var
  GlobalCustomerList: TGenericList<TCustomer>;
  GlobalSettings: TStringList;
  ApplicationPath: string;
  IsInitialized: Boolean = False;

implementation

{$R *.dfm}

{ TAddress }

procedure TAddress.Clear;
begin
  Street := '';
  City := '';
  State := '';
  ZipCode := '';
  Country := '';
end;

function TAddress.FullAddress: string;
begin
  Result := Format('%s, %s, %s %s, %s', [Street, City, State, ZipCode, Country]);
end;

{ TCustomer }

constructor TCustomer.Create;
begin
  inherited;
  FTags := TStringList.Create;
  FCreated := Now;
  FModified := Now;
  FActive := True;
  FDiscount := DEFAULT_DISCOUNT;
end;

destructor TCustomer.Destroy;
begin
  FTags.Free;
  inherited;
end;

procedure TCustomer.Assign(Source: TCustomer);
begin
  if Assigned(Source) then
  begin
    FID := Source.FID;
    FName := Source.FName;
    FEmail := Source.FEmail;
    FPhone := Source.FPhone;
    FAddress := Source.FAddress;
    FBalance := Source.FBalance;
    FCreated := Source.FCreated;
    FModified := Source.FModified;
    FActive := Source.FActive;
    FDiscount := Source.FDiscount;
    FTags.Assign(Source.FTags);
  end;
end;

function TCustomer.Clone: TCustomer;
begin
  Result := TCustomer.Create;
  Result.Assign(Self);
end;

procedure TCustomer.SetBalance(const Value: Currency);
begin
  if FBalance <> Value then
  begin
    FBalance := Value;
    DoModify;
  end;
end;

procedure TCustomer.SetEmail(const Value: string);
begin
  if FEmail <> Value then
  begin
    if ValidateEmail(Value) then
    begin
      FEmail := Value;
      DoModify;
    end
    else
      raise ECustomerException.CreateFmt(SInvalidEmail, [Value], 1001, FID);
  end;
end;

function TCustomer.GetFullName: string;
begin
  Result := Format('%s %s', [FName, FEmail]);
end;

procedure TCustomer.DoModify;
begin
  FModified := Now;
end;

function TCustomer.ValidateEmail(const Email: string): Boolean;
var
  Regex: TPerlRegEx;
begin
  Regex := TPerlRegEx.Create;
  try
    Regex.RegEx := '^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$';
    Regex.Subject := Email;
    Result := Regex.Match;
  finally
    Regex.Free;
  end;
end;

procedure TCustomer.LoadFromDataset(Dataset: TDataSet);
begin
  if Assigned(Dataset) and not Dataset.IsEmpty then
  begin
    FID := Dataset.FieldByName('ID').AsInteger;
    FName := Dataset.FieldByName('Name').AsString;
    FEmail := Dataset.FieldByName('Email').AsString;
    FPhone := Dataset.FieldByName('Phone').AsString;
    FAddress.Street := Dataset.FieldByName('Street').AsString;
    FAddress.City := Dataset.FieldByName('City').AsString;
    FAddress.State := Dataset.FieldByName('State').AsString;
    FAddress.ZipCode := Dataset.FieldByName('ZipCode').AsString;
    FAddress.Country := Dataset.FieldByName('Country').AsString;
    FBalance := Dataset.FieldByName('Balance').AsCurrency;
    FCreated := Dataset.FieldByName('Created').AsDateTime;
    FModified := Dataset.FieldByName('Modified').AsDateTime;
    FActive := Dataset.FieldByName('Active').AsBoolean;
    FDiscount := Dataset.FieldByName('Discount').AsFloat;
    FTags.CommaText := Dataset.FieldByName('Tags').AsString;
  end;
end;

procedure TCustomer.SaveToDataset(Dataset: TDataSet);
begin
  if Assigned(Dataset) then
  begin
    Dataset.FieldByName('ID').AsInteger := FID;
    Dataset.FieldByName('Name').AsString := FName;
    Dataset.FieldByName('Email').AsString := FEmail;
    Dataset.FieldByName('Phone').AsString := FPhone;
    Dataset.FieldByName('Street').AsString := FAddress.Street;
    Dataset.FieldByName('City').AsString := FAddress.City;
    Dataset.FieldByName('State').AsString := FAddress.State;
    Dataset.FieldByName('ZipCode').AsString := FAddress.ZipCode;
    Dataset.FieldByName('Country').AsString := FAddress.Country;
    Dataset.FieldByName('Balance').AsCurrency := FBalance;
    Dataset.FieldByName('Created').AsDateTime := FCreated;
    Dataset.FieldByName('Modified').AsDateTime := FModified;
    Dataset.FieldByName('Active').AsBoolean := FActive;
    Dataset.FieldByName('Discount').AsFloat := FDiscount;
    Dataset.FieldByName('Tags').AsString := FTags.CommaText;
  end;
end;

function TCustomer.CalculateDiscount(Amount: Currency): Currency;
begin
  Result := Amount * (1 - FDiscount);
end;

class function TCustomer.GetCustomerByID(ID: TCustomerID): TCustomer;
var
  Service: ICustomerService;
begin
  Service := TCustomerService.Create;
  Result := Service.GetCustomer(ID);
end;

{ TGenericList<T> }

constructor TGenericList<T>.Create;
begin
  inherited;
  FCount := 0;
  FCapacity := 16;
  SetLength(FItems, FCapacity);
end;

destructor TGenericList<T>.Destroy;
begin
  SetLength(FItems, 0);
  inherited;
end;

procedure TGenericList<T>.Add(const Item: T);
begin
  if FCount = FCapacity then
    Grow;
  FItems[FCount] := Item;
  Inc(FCount);
end;

procedure TGenericList<T>.Insert(Index: Integer; const Item: T);
var
  I: Integer;
begin
  if (Index < 0) or (Index > FCount) then
    raise EListError.Create('Index out of bounds');
    
  if FCount = FCapacity then
    Grow;
    
  for I := FCount downto Index + 1 do
    FItems[I] := FItems[I - 1];
    
  FItems[Index] := Item;
  Inc(FCount);
end;

procedure TGenericList<T>.Delete(Index: Integer);
var
  I: Integer;
begin
  if (Index < 0) or (Index >= FCount) then
    raise EListError.Create('Index out of bounds');
    
  for I := Index to FCount - 2 do
    FItems[I] := FItems[I + 1];
    
  Dec(FCount);
end;

function TGenericList<T>.Remove(const Item: T): Integer;
begin
  Result := IndexOf(Item);
  if Result >= 0 then
    Delete(Result);
end;

function TGenericList<T>.IndexOf(const Item: T): Integer;
var
  I: Integer;
  Comparer: IEqualityComparer<T>;
begin
  Comparer := TEqualityComparer<T>.Default;
  for I := 0 to FCount - 1 do
    if Comparer.Equals(FItems[I], Item) then
      Exit(I);
  Result := -1;
end;

procedure TGenericList<T>.Clear;
begin
  FCount := 0;
end;

procedure TGenericList<T>.Sort(Compare: TComparer<T>);
var
  Temp: T;
  I, J: Integer;
begin
  for I := 0 to FCount - 2 do
    for J := I + 1 to FCount - 1 do
      if Compare.Compare(FItems[I], FItems[J]) > 0 then
      begin
        Temp := FItems[I];
        FItems[I] := FItems[J];
        FItems[J] := Temp;
      end;
end;

procedure TGenericList<T>.Sort;
begin
  Sort(TComparer<T>.Default);
end;

procedure TGenericList<T>.Grow;
begin
  FCapacity := FCapacity * 2;
  SetLength(FItems, FCapacity);
end;

function TGenericList<T>.GetItem(Index: Integer): T;
begin
  if (Index < 0) or (Index >= FCount) then
    raise EListError.Create('Index out of bounds');
  Result := FItems[Index];
end;

procedure TGenericList<T>.SetItem(Index: Integer; const Value: T);
begin
  if (Index < 0) or (Index >= FCount) then
    raise EListError.Create('Index out of bounds');
  FItems[Index] := Value;
end;

{ TCustomerService }

constructor TCustomerService.Create;
begin
  inherited;
  PrepareConnection;
end;

destructor TCustomerService.Destroy;
begin
  FQuery.Free;
  FConnection.Free;
  inherited;
end;

procedure TCustomerService.PrepareConnection;
begin
  FConnection := TFDConnection.Create(nil);
  FQuery := TFDQuery.Create(nil);
  FQuery.Connection := FConnection;
  
  // Configure connection
  FConnection.Params.Values['DriverID'] := 'SQLite';
  FConnection.Params.Values['Database'] := 'customers.db';
  FConnection.LoginPrompt := False;
  
  try
    FConnection.Connected := True;
  except
    on E: Exception do
      raise ECustomerException.CreateFmt('Database connection failed: %s', [E.Message], 2001, 0);
  end;
end;

function TCustomerService.GetCustomer(ID: TCustomerID): TCustomer;
begin
  FQuery.SQL.Text := 'SELECT * FROM Customers WHERE ID = :ID';
  FQuery.ParamByName('ID').AsInteger := ID;
  FQuery.Open;
  
  try
    if not FQuery.IsEmpty then
    begin
      Result := TCustomer.Create;
      Result.LoadFromDataset(FQuery);
    end
    else
      Result := nil;
  finally
    FQuery.Close;
  end;
end;

procedure TCustomerService.SaveCustomer(Customer: TCustomer);
begin
  ValidateCustomer(Customer);
  
  if Customer.ID = 0 then
  begin
    // Insert new customer
    FQuery.SQL.Text := 
      'INSERT INTO Customers (Name, Email, Phone, Street, City, State, ZipCode, Country, ' +
      'Balance, Created, Modified, Active, Discount, Tags) VALUES ' +
      '(:Name, :Email, :Phone, :Street, :City, :State, :ZipCode, :Country, ' +
      ':Balance, :Created, :Modified, :Active, :Discount, :Tags)';
  end
  else
  begin
    // Update existing customer
    FQuery.SQL.Text := 
      'UPDATE Customers SET Name = :Name, Email = :Email, Phone = :Phone, ' +
      'Street = :Street, City = :City, State = :State, ZipCode = :ZipCode, ' +
      'Country = :Country, Balance = :Balance, Modified = :Modified, ' +
      'Active = :Active, Discount = :Discount, Tags = :Tags ' +
      'WHERE ID = :ID';
    FQuery.ParamByName('ID').AsInteger := Customer.ID;
  end;
  
  Customer.SaveToDataset(FQuery);
  FQuery.ExecSQL;
end;

procedure TCustomerService.DeleteCustomer(ID: TCustomerID);
begin
  FQuery.SQL.Text := 'DELETE FROM Customers WHERE ID = :ID';
  FQuery.ParamByName('ID').AsInteger := ID;
  FQuery.ExecSQL;
  
  if FQuery.RowsAffected = 0 then
    raise ECustomerException.Create('Customer not found', 2002, ID);
end;

function TCustomerService.GetAllCustomers: TArray<TCustomer>;
var
  Customer: TCustomer;
begin
  FQuery.SQL.Text := 'SELECT * FROM Customers ORDER BY Name';
  FQuery.Open;
  
  try
    SetLength(Result, FQuery.RecordCount);
    var I := 0;
    while not FQuery.Eof do
    begin
      Customer := TCustomer.Create;
      Customer.LoadFromDataset(FQuery);
      Result[I] := Customer;
      Inc(I);
      FQuery.Next;
    end;
  finally
    FQuery.Close;
  end;
end;

function TCustomerService.FindCustomers(const Filter: string): TArray<TCustomer>;
var
  Customer: TCustomer;
  FilterSQL: string;
begin
  FilterSQL := 'SELECT * FROM Customers WHERE Name LIKE :Filter OR Email LIKE :Filter ORDER BY Name';
  FQuery.SQL.Text := FilterSQL;
  FQuery.ParamByName('Filter').AsString := '%' + Filter + '%';
  FQuery.Open;
  
  try
    SetLength(Result, FQuery.RecordCount);
    var I := 0;
    while not FQuery.Eof do
    begin
      Customer := TCustomer.Create;
      Customer.LoadFromDataset(FQuery);
      Result[I] := Customer;
      Inc(I);
      FQuery.Next;
    end;
  finally
    FQuery.Close;
  end;
end;

function TCustomerService.GetCustomerCount: Integer;
begin
  FQuery.SQL.Text := 'SELECT COUNT(*) FROM Customers';
  FQuery.Open;
  
  try
    Result := FQuery.Fields[0].AsInteger;
  finally
    FQuery.Close;
  end;
end;

procedure TCustomerService.ValidateCustomer(Customer: TCustomer);
begin
  if Customer.Name = '' then
    raise ECustomerException.Create('Customer name is required', 2003, Customer.ID);
    
  if not Customer.ValidateEmail(Customer.Email) then
    raise ECustomerException.CreateFmt(SInvalidEmail, [Customer.Email], 2004, Customer.ID);
    
  if Customer.Balance < 0 then
    raise ECustomerException.Create('Customer balance cannot be negative', 2005, Customer.ID);
end;

{ TMathHelper }

class constructor TMathHelper.Create;
begin
  FPI := System.PI;
  FEpsilon := 1e-10;
end;

class function TMathHelper.Factorial(N: Integer): Int64;
var
  I: Integer;
begin
  if N < 0 then
    raise EMathError.Create('Factorial is not defined for negative numbers');
    
  Result := 1;
  for I := 2 to N do
    Result := Result * I;
end;

class function TMathHelper.Power(Base, Exponent: Double): Double;
begin
  if (Base = 0) and (Exponent <= 0) then
    raise EMathError.Create('Invalid power operation');
    
  Result := System.Math.Power(Base, Exponent);
end;

class function TMathHelper.IsPrime(Number: Integer): Boolean;
var
  I: Integer;
begin
  if Number <= 1 then
    Exit(False);
    
  if Number <= 3 then
    Exit(True);
    
  if (Number mod 2 = 0) or (Number mod 3 = 0) then
    Exit(False);
    
  I := 5;
  while I * I <= Number do
  begin
    if (Number mod I = 0) or (Number mod (I + 2) = 0) then
      Exit(False);
    Inc(I, 6);
  end;
  
  Result := True;
end;

class function TMathHelper.Fibonacci(N: Integer): Int64;
var
  A, B, Temp: Int64;
  I: Integer;
begin
  if N <= 0 then
    Exit(0);
    
  if N = 1 then
    Exit(1);
    
  A := 0;
  B := 1;
  
  for I := 2 to N do
  begin
    Temp := A + B;
    A := B;
    B := Temp;
  end;
  
  Result := B;
end;

class function TMathHelper.GCD(A, B: Integer): Integer;
var
  Temp: Integer;
begin
  A := Abs(A);
  B := Abs(B);
  
  while B <> 0 do
  begin
    Temp := B;
    B := A mod B;
    A := Temp;
  end;
  
  Result := A;
end;

class function TMathHelper.LCM(A, B: Integer): Integer;
begin
  if (A = 0) or (B = 0) then
    Result := 0
  else
    Result := Abs(A * B) div GCD(A, B);
end;

{ TBackgroundWorker }

constructor TBackgroundWorker.Create(CreateSuspended: Boolean);
begin
  inherited Create(CreateSuspended);
  FProgress := 0;
  FMessage := '';
  FreeOnTerminate := True;
end;

procedure TBackgroundWorker.Execute;
var
  I: Integer;
begin
  try
    for I := 1 to 100 do
    begin
      if Terminated then
        Break;
        
      Sleep(50); // Simulate work
      Progress := I;
      Message := Format('Processing item %d of 100', [I]);
    end;
    
    Message := 'Background work completed';
  except
    on E: Exception do
    begin
      Message := 'Error: ' + E.Message;
    end;
  end;
end;

procedure TBackgroundWorker.SetProgress(Value: Integer);
begin
  if FProgress <> Value then
  begin
    FProgress := Value;
    Synchronize(DoProgress);
  end;
end;

procedure TBackgroundWorker.SetMessage(const Value: string);
begin
  if FMessage <> Value then
  begin
    FMessage := Value;
    Synchronize(DoMessage);
  end;
end;

procedure TBackgroundWorker.DoProgress;
begin
  if Assigned(FOnProgress) then
    FOnProgress(Self);
end;

procedure TBackgroundWorker.DoMessage;
begin
  if Assigned(FOnMessage) then
    FOnMessage(Self);
end;

{ ECustomerException }

constructor ECustomerException.Create(const Msg: string; ErrorCode: Integer; CustomerID: TCustomerID);
begin
  inherited Create(Msg);
  FErrorCode := ErrorCode;
  FCustomerID := CustomerID;
end;

{ TMainForm }

procedure TMainForm.FormCreate(Sender: TObject);
begin
  InitializeComponents;
  SetupDatabase;
  LoadCustomers;
  UpdateStatusBar;
end;

procedure TMainForm.FormDestroy(Sender: TObject);
begin
  if Assigned(FCurrentCustomer) then
    FCurrentCustomer.Free;
end;

procedure TMainForm.InitializeComponents;
begin
  Caption := Format('%s - %s', [APP_NAME, VERSION]);
  
  // Initialize form components
  FilterEdit.Text := '';
  FilterLabel.Caption := 'Filter:';
  SearchButton.Caption := 'Search';
  ClearButton.Caption := 'Clear';
  ExportButton.Caption := 'Export';
  PrintButton.Caption := 'Print';
  
  // Setup grid
  CustomerGrid.Options := CustomerGrid.Options + [dgRowSelect, dgAlwaysShowSelection];
  CustomerGrid.Align := alClient;
end;

procedure TMainForm.SetupDatabase;
begin
  FCustomerService := TCustomerService.Create;
end;

procedure TMainForm.LoadCustomers;
var
  Customers: TArray<TCustomer>;
  I: Integer;
begin
  try
    if FFilterText <> '' then
      Customers := FCustomerService.FindCustomers(FFilterText)
    else
      Customers := FCustomerService.GetAllCustomers;
      
    // Load customers into grid or other UI components
    // This is a simplified version - in real app would use data binding
    for I := 0 to High(Customers) do
    begin
      // Add to grid or list
      if Assigned(Customers[I]) then
        Customers[I].Free;
    end;
  except
    on E: Exception do
      MessageDlg(Format(SErrorLoadingData, [E.Message]), mtError, [mbOK], 0);
  end;
end;

procedure TMainForm.UpdateStatusBar;
begin
  StatusBar.Panels[0].Text := Format('Total customers: %d', [FCustomerService.GetCustomerCount]);
  StatusBar.Panels[1].Text := Format('Filter: %s', [FFilterText]);
  StatusBar.Panels[2].Text := Format('Date: %s', [FormatDateTime('yyyy-mm-dd', Now)]);
end;

procedure TMainForm.SearchButtonClick(Sender: TObject);
begin
  FFilterText := FilterEdit.Text;
  LoadCustomers;
  UpdateStatusBar;
end;

procedure TMainForm.ClearButtonClick(Sender: TObject);
begin
  FilterEdit.Text := '';
  FFilterText := '';
  LoadCustomers;
  UpdateStatusBar;
end;

procedure TMainForm.ExportButtonClick(Sender: TObject);
begin
  // Export customers to CSV or other format
  // Implementation would go here
  ShowMessage('Export functionality would be implemented here');
end;

procedure TMainForm.PrintButtonClick(Sender: TObject);
begin
  // Print customer list
  // Implementation would go here
  ShowMessage('Print functionality would be implemented here');
end;

procedure TMainForm.FilterEditChange(Sender: TObject);
begin
  // Auto-search as user types (with debouncing)
  // Implementation would include timer-based debouncing
end;

procedure TMainForm.DoShow;
begin
  inherited;
  ShowMessage(Format(SWelcomeMessage, [APP_NAME, VERSION]));
end;

procedure TMainForm.DoHide;
begin
  inherited;
  // Cleanup when form is hidden
end;

procedure TMainForm.ShowCustomerDetails(Customer: TCustomer);
begin
  if Assigned(Customer) then
  begin
    if Assigned(FCurrentCustomer) then
      FCurrentCustomer.Free;
      
    FCurrentCustomer := Customer.Clone;
    // Update UI with customer details
    // Implementation would go here
  end;
end;

function TMainForm.ValidateForm: Boolean;
begin
  Result := True;
  // Form validation logic would go here
end;

initialization
  GlobalCustomerList := TGenericList<TCustomer>.Create;
  GlobalSettings := TStringList.Create;
  ApplicationPath := ExtractFilePath(ParamStr(0));
  IsInitialized := True;

finalization
  if Assigned(GlobalCustomerList) then
    GlobalCustomerList.Free;
  if Assigned(GlobalSettings) then
    GlobalSettings.Free;

end.