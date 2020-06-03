param (
    [Parameter(Mandatory=$true)]
    [String]
    $PackageName,

    [Parameter()]
    [String]
    $BuildMethod
)

enum BuildMethodType {
    Docker
}

function Invoke-DockerBuild($ProjectId) {
    $ImageName = Get-GcpDockerImageName -ProjectId $ProjectId -PackageName $PackageName

    docker build -t $ImageName -f "./$PackageName/Dockerfile" .
}

# Import common helpers.
. (Join-Path $PSScriptRoot "./shared/shared.ps1")

$ProjectId = (Get-Config).PROJECT_ID.ToString()

try {
    if ($BuildMethod -eq $null) {
        Write-Warning "No BuildMethod provided; defaulting to Docker"

        $BuildMethod = "Docker"
    }

    $BuildMethod = [BuildMethodType]$BuildMethod

    switch ($BuildMethod) {
        ([BuildMethodType]::Docker) {
            Invoke-DockerBuild -ProjectId $ProjectId
        }
        Default {
            throw "Unknown build method '$BuildMethod'!"
        }
    }
}
catch {
    Write-Error "Something went wrong while building package '$PackageName': $($Error[0])"
}