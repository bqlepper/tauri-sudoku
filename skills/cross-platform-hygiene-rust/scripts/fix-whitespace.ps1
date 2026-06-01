param()

$ErrorActionPreference = "Stop"

if (-not (Test-Path ".git")) {
    throw "Run this script from repository root."
}

$trackedAndUntracked = git ls-files --cached --others --exclude-standard
$textExt = @(
    ".rs", ".js", ".css", ".html", ".json", ".toml",
    ".md", ".txt", ".yaml", ".yml",
    ".gitignore", ".gitattributes", ".editorconfig"
)
$specialNames = @(".gitignore", ".gitattributes", ".editorconfig")
$changed = @()

foreach ($file in $trackedAndUntracked) {
    if (-not (Test-Path -LiteralPath $file)) {
        continue
    }

    $ext = [System.IO.Path]::GetExtension($file).ToLowerInvariant()
    $name = [System.IO.Path]::GetFileName($file).ToLowerInvariant()
    if (($textExt -notcontains $ext) -and ($specialNames -notcontains $name)) {
        continue
    }

    $path = (Resolve-Path $file).Path
    $raw = [System.IO.File]::ReadAllText($path)
    $updated = $raw -replace "`r`n", "`n" -replace "`r", "`n"
    $updated = $updated -replace "`t", "    "
    $updated = [System.Text.RegularExpressions.Regex]::Replace($updated, "[ \t]+(?=\n)", "")
    $updated = [System.Text.RegularExpressions.Regex]::Replace($updated, "(?m)^[ \t]+$", "")

    if ($updated -ne $raw) {
        $utf8NoBom = New-Object System.Text.UTF8Encoding($false)
        [System.IO.File]::WriteAllText($path, $updated, $utf8NoBom)
        $changed += $file
    }
}

if ($changed.Count -eq 0) {
    Write-Host "No whitespace fixes needed."
    exit 0
}

Write-Host "Updated files:"
$changed | Sort-Object | ForEach-Object { Write-Host $_ }
exit 0

