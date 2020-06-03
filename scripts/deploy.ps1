param (
    [Parameter(Mandatory=$true)]
    [String]
    $PackageName
)

# Import common helpers.
. (Join-Path $PSScriptRoot "./shared/shared.ps1")

$ProjectId = (Get-Config).PROJECT_ID.ToString()

try {
    $ImageName = Get-GcpDockerImageName -ProjectId $ProjectId -PackageName $PackageName

    docker push $ImageName
}
catch {
    Write-Error "Something went wrong while deploying package '$PackageName': $($Error[0])"
}