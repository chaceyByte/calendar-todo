# Node.js Upgrade Script
Write-Host "Downloading Node.js 20 LTS..." -ForegroundColor Cyan

$downloadUrl = "https://nodejs.org/dist/v20.11.0/node-v20.11.0-x64.msi"
$installerPath = "$env:TEMP\node-v20.11.0-x64.msi"

try {
    # Download Node.js 20 LTS
    Invoke-WebRequest -Uri $downloadUrl -OutFile $installerPath -UseBasicParsing
    Write-Host "Download complete" -ForegroundColor Green
    
    Write-Host "Installing Node.js 20..." -ForegroundColor Cyan
    # Silent install
    Start-Process msiexec.exe -Wait -ArgumentList "/i `"$installerPath`" /quiet /norestart"
    
    Write-Host "Installation complete!" -ForegroundColor Green
    Write-Host "Please close and reopen your terminal, then run:" -ForegroundColor Yellow
    Write-Host "  node --version" -ForegroundColor White
    
    # Cleanup installer
    Remove-Item $installerPath -Force
}
catch {
    Write-Host "Installation failed: $_" -ForegroundColor Red
    Write-Host "Please download manually from: https://nodejs.org/" -ForegroundColor Yellow
}
