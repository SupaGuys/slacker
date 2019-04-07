#!/usr/bin/env pwsh
${PROJECT_PATH}=(get-item $PSScriptRoot ).parent.FullName
Set-Location ${PROJECT_PATH}

$env:SLKR_VERSION = $(Get-Content ${PROJECT_PATH}\version -First 1)
docker-compose build
