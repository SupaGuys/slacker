#!/usr/bin/env pwsh
$env:PROJECT_PATH   = (Get-Item $PSScriptRoot ).parent.parent.FullName
$env:LOCAL_PATH     = (Get-Item $PSScriptRoot ).FullName
$env:BOT_NAME       = (Get-Item $PSScriptRoot).BaseName
$env:BOT_VERSION    = $(Get-Content $env:PROJECT_PATH\version -First 1)

Set-Location $env:PROJECT_PATH
docker-compose up
