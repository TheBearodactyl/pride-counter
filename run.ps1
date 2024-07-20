$ErrorActionPreference = "Stop"

$OUT_DIR = "."

if (Get-Command cargo -ErrorAction SilentlyContinue)
{
  cargo run --quiet --release
} else
{
  Write-Host "install rust"
}
