param(
    [switch]$RunTests
)

$ErrorActionPreference = "Stop"

function Write-Section([string]$Message) {
    Write-Host ""
    Write-Host "=== $Message ==="
}

function Run-OptionalNoMatch([scriptblock]$Command) {
    & $Command
    $code = $LASTEXITCODE
    if ($code -eq 1) {
        return @()
    }
    if ($code -ne 0) {
        throw "Command failed with exit code $code"
    }
}

Write-Section "Cross-Platform Hygiene Audit"
$issues = @()

if (-not (Test-Path ".git")) {
    throw "Run this script from repository root."
}

Write-Section "Line Endings (LF expected)"
$eol = git ls-files --eol
$badEol = $eol | Where-Object { $_ -match "w/crlf|i/crlf" }
if ($badEol.Count -gt 0) {
    $issues += "Found CRLF line endings in tracked files."
    $badEol | ForEach-Object { Write-Host $_ }
} else {
    Write-Host "OK: tracked text files are LF."
}

Write-Section "Tabs In Tracked Text Files"
$tabs = Run-OptionalNoMatch { git grep -nI -P "\t" }
if ($tabs.Count -gt 0) {
    $issues += "Found tab characters in tracked text files."
    $tabs | ForEach-Object { Write-Host $_ }
} else {
    Write-Host "OK: no tab characters in tracked text files."
}

Write-Section "Trailing Whitespace"
$trailing = Run-OptionalNoMatch { git grep -nI -P "[ \t]+$" }
if ($trailing.Count -gt 0) {
    $issues += "Found trailing whitespace."
    $trailing | ForEach-Object { Write-Host $_ }
} else {
    Write-Host "OK: no trailing whitespace."
}

Write-Section "Case-Collision Check"
$paths = git ls-files
$seen = @{}
$caseCollision = @()
foreach ($path in $paths) {
    $key = $path.ToLowerInvariant()
    if ($seen.ContainsKey($key) -and $seen[$key] -ne $path) {
        $caseCollision += "$($seen[$key]) <-> $path"
    } else {
        $seen[$key] = $path
    }
}
if ($caseCollision.Count -gt 0) {
    $issues += "Found case-colliding tracked paths (Linux hazard)."
    $caseCollision | ForEach-Object { Write-Host $_ }
} else {
    Write-Host "OK: no case-colliding tracked paths."
}

Write-Section "Path/Portability Smells"
$smells = Run-OptionalNoMatch { rg -n "src-rust-sudoku|C:\\\\|\\\\\\\\|\\.exe" src-tauri src-ui docs test-harness.md readme.md }
if ($smells.Count -gt 0) {
    Write-Host "Review these matches for portability impact:"
    $smells | ForEach-Object { Write-Host $_ }
} else {
    Write-Host "OK: no obvious hardcoded path smells found."
}

if ($RunTests) {
    Write-Section "Rust Validation"
    cargo test --manifest-path src-tauri/Cargo.toml
    if ($LASTEXITCODE -ne 0) {
        $issues += "cargo test failed."
    }

    cargo run --manifest-path src-tauri/Cargo.toml --bin test_runner -- test
    if ($LASTEXITCODE -ne 0) {
        $issues += "test_runner harness failed."
    }
}

Write-Section "Summary"
if ($issues.Count -eq 0) {
    Write-Host "PASS: No blocking hygiene issues found."
    exit 0
}

Write-Host "FAIL: Found issues:"
$issues | ForEach-Object { Write-Host "- $_" }
exit 1

