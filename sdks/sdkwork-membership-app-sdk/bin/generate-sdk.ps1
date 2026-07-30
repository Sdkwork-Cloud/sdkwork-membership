param(
    [string[]]$Languages = @("typescript"),
    [string]$BaseUrl = "http://127.0.0.1:18096",
    [string]$SdkVersion = "0.1.0"
)

$ErrorActionPreference = "Stop"

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$Arguments = @(
    "--languages", ($Languages -join ","),
    "--base-url", $BaseUrl,
    "--sdk-version", $SdkVersion
)

& node (Join-Path $ScriptDir "generate-sdk.mjs") @Arguments
exit $LASTEXITCODE
