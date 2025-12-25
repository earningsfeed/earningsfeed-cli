$ErrorActionPreference = 'Stop'

$packageName = 'earningsfeed'
$url64 = "https://github.com/earningsfeed/earningsfeed-cli/releases/download/v$($env:chocolateyPackageVersion)/earningsfeed-cli-x86_64-pc-windows-msvc.msi"

$packageArgs = @{
  packageName    = $packageName
  fileType       = 'msi'
  url64bit       = $url64
  softwareName   = 'earningsfeed-cli*'
  checksum64     = '$CHECKSUM$'
  checksumType64 = 'sha256'
  silentArgs     = '/qn /norestart'
  validExitCodes = @(0, 3010, 1641)
}

Install-ChocolateyPackage @packageArgs
