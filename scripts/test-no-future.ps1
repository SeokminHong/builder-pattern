$ErrorActionPreference = 'SilentlyContinue'
Push-Location $PSScriptRoot\..\test-no-future\
Get-ChildItem examples -Filter *.rs |
Foreach-Object {
  cargo run --example $_.BaseName
  if ($_.BaseName.StartsWith('fail-')) {
    if ($LASTEXITCODE -eq 0) {
      Write-Output "Error: example '$_' should fail!"
      $Host.SetShouldExit(1)
      Pop-Location
      exit
    }
  } else {
    if ($LASTEXITCODE -ne 0) {
      $Host.SetShouldExit($LASTEXITCODE)
      Pop-Location
      exit
    }
  }
}
Pop-Location
