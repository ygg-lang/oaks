# PowerShell test file
param(
    [string]$Name = "World",
    [int]$Count = 1
)

Write-Host "Hello, $Name!" -ForegroundColor Green

# Variables
$numbers = @(1, 2, 3, 4, 5)
$hashtable = @{
    "key1" = "value1"
    "key2" = "value2"
}

# Functions
function Get-Square {
    param([int]$Number)
    return $Number * $Number
}

# Loops
for ($i = 0; $i -lt $Count; $i++) {
    $result = Get-Square -Number ($i + 1)
    Write-Output "Square of $($i + 1) is $result"
}

# Conditional
if ($Count -gt 5) {
    Write-Warning "Count is greater than 5"
} else {
    Write-Information "Count is acceptable"
}

# Pipeline
Get-Process | Where-Object { $_.ProcessName -like "powershell*" } | Select-Object Name, Id