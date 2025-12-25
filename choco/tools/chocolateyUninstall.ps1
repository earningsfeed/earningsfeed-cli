$ErrorActionPreference = 'Stop'

$packageName = 'earningsfeed'
$softwareName = 'earningsfeed-cli*'
$installerType = 'msi'
$silentArgs = '/qn /norestart'
$validExitCodes = @(0, 1605, 3010)

$uninstalled = $false
[array]$key = Get-UninstallRegistryKey -SoftwareName $softwareName

if ($key.Count -eq 1) {
  $key | ForEach-Object {
    $file = "$($_.UninstallString)"

    if ($installerType -eq 'msi') {
      $silentArgs = "$($_.PSChildName) $silentArgs"
      $file = ''
    }

    Uninstall-ChocolateyPackage -PackageName $packageName `
                                -FileType $installerType `
                                -SilentArgs "$silentArgs" `
                                -ValidExitCodes $validExitCodes `
                                -File "$file"
  }
} elseif ($key.Count -eq 0) {
  Write-Warning "$packageName has already been uninstalled by other means."
} elseif ($key.Count -gt 1) {
  Write-Warning "$($key.Count) matches found!"
  Write-Warning "To prevent accidental data loss, no programs will be uninstalled."
  Write-Warning "Please alert the package maintainer."
}
