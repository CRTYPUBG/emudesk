param(
    [Parameter(Mandatory = $true)]
    [string]$Version
)

$ErrorActionPreference = "Stop"

$ReleaseDir = "target/release"
$Exe = Join-Path $ReleaseDir "emudesk.exe"
$ZipName = "EmuDesk-v$Version-windows-x86_64.zip"
$SumsName = "SHA256SUMS.txt"
$Staging = Join-Path $PWD "staging-windows"

# Varlığı doğrula
if (-not (Test-Path $Exe)) {
    Write-Error "Derlenmiş binary bulunamadı: $Exe"
    exit 1
}

# Paketleme için temiz bir staging dizini hazırla
if (Test-Path $Staging) {
    Remove-Item $Staging -Recurse -Force
}
New-Item -ItemType Directory -Path $Staging -Force | Out-Null

# Binary'yi kopyala
Copy-Item $Exe -Destination $Staging

# LICENSE ve README varsa ekle (yoksa zarifçe atla)
if (Test-Path "LICENSE") {
    Copy-Item "LICENSE" -Destination $Staging
} else {
    Write-Warning "LICENSE bulunamadı; pakete dahil edilmiyor."
}
if (Test-Path "README.md") {
    Copy-Item "README.md" -Destination $Staging
} else {
    Write-Warning "README.md bulunamadı; pakete dahil edilmiyor."
}

# ZIP oluştur (eski dosyayı ez)
if (Test-Path $ZipName) {
    Remove-Item $ZipName -Force
}
Compress-Archive -Path (Join-Path $Staging "*") -DestinationPath $ZipName

# SHA256 özetini üret
$Hash = (Get-FileHash -Path $ZipName -Algorithm SHA256).Hash.ToLowerInvariant()
"$Hash  $ZipName" | Set-Content -Path $SumsName

# Temizlik
Remove-Item $Staging -Recurse -Force

Write-Host "Paket oluşturuldu: $ZipName"
Write-Host "Özet dosyası: $SumsName"