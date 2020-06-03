function Get-Config() {
    return Get-Content (Join-Path $PSScriptRoot "./config.json") | ConvertFrom-Json
}

function Get-GcpDockerImageName($ProjectId, $PackageName) {
    return "gcr.io/$ProjectId/$PackageName"
}