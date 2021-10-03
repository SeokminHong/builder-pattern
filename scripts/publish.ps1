Push-Location $PSScriptRoot\..\builder-pattern-macro
cargo publish
Pop-Location
Push-Location $PSScriptRoot\..\builder-pattern
cargo publish
Pop-Location
