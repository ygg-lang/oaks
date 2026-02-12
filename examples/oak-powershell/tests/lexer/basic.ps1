# Comprehensive PowerShell Lexer Test

# Variables
$name = "PowerShell"
$version = 7.4
$isCrossPlatform = $true
$nullValue = $null

# Arrays
$numbers = 1, 2, 3, 4, 5
$colors = @("Red", "Green", "Blue")
$mixed = @(1, "Two", 3.0)

# Hashtables
$user = @{
    Name = "JohnDoe"
    ID = 1234
    Roles = @("Admin", "User")
}

# Accessing properties
Write-Host "User: $($user.Name)"
$user["ID"] = 5678

# Strings
$singleQuoted = 'This is a literal string $name'
$doubleQuoted = "This expands variable: $name"
$escaped = "This is a backtick `" and newline `n"
$hereString = @"
This is a here-string
It can span multiple lines
Variables like $version work here
"@

# Conditionals
if ($version -ge 7.0) {
    Write-Output "Modern PowerShell"
} elseif ($version -lt 6.0) {
    Write-Output "Windows PowerShell"
} else {
    Write-Output "PowerShell Core"
}

# Loops
for ($i = 0; $i -lt 5; $i++) {
    Write-Host "Count: $i"
}

foreach ($color in $colors) {
    Write-Host "Color: $color"
}

$numbers | ForEach-Object {
    $_ * 2
}

while ($true) {
    break
}

# Functions
function Get-Greeting {
    param (
        [Parameter(Mandatory=$true)]
        [string]$Name,
        
        [int]$Count = 1
    )
    
    process {
        for ($i = 0; $i -lt $Count; $i++) {
            Write-Output "Hello, $Name!"
        }
    }
}

# Cmdlet calls
Get-Greeting -Name "World" -Count 3
Get-ChildItem -Path . -Recurse -Filter "*.ps1" | Select-Object Name, Length

# Pipeline
Get-Process | Where-Object { $_.CPU -gt 100 } | Sort-Object CPU -Descending | Select-Object -First 5

# Error Handling
try {
    $result = 1 / 0
} catch [System.DivideByZeroException] {
    Write-Error "Cannot divide by zero"
} finally {
    Write-Host "Cleanup"
}

# .NET Interop
[System.Math]::Pow(2, 3)
$date = [DateTime]::Now

# Classes (PowerShell 5.0+)
class Animal {
    [string]$Name
    
    Animal([string]$name) {
        $this.Name = $name
    }
    
    [void] Speak() {
        Write-Host "$($this.Name) makes a sound"
    }
}

class Dog : Animal {
    Dog([string]$name) : base($name) {}
    
    [void] Speak() {
        Write-Host "$($this.Name) barks"
    }
}

$dog = [Dog]::new("Rex")
$dog.Speak()

# Comments
# Single line comment
<#
    Multi-line
    block comment
#>

# Operators
$a = 10
$b = 20
$sum = $a + $b
$isEqual = $a -eq $b
$isNotEqual = $a -ne $b
$match = "Hello" -match "He.*"
$replace = "Hello" -replace "He", "Ha"
$bitwise = 1 -band 1

# Splatting
$params = @{
    Path = "C:\Temp"
    Recurse = $true
}
Get-ChildItem @params
