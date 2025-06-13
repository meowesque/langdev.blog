function Build-CSS {
    param (
        [switch]$Watch = $true
    )
    
    $inputFile = "./css/styles.css"
    $outputFile = "./static/styles.css"
    
    if ($Watch) {
        npx @tailwindcss/cli -i $inputFile -o $outputFile --watch
    } else {
        npx @tailwindcss/cli -i $inputFile -o $outputFile
    }
}

function Start-MailServer {
    mailtutan 
}

function Show-Help {
    Write-Host "Usage: .\meta.ps1 [target]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Available targets:" -ForegroundColor Yellow
    Write-Host "  css      - Build CSS with Tailwind and watch for changes" -ForegroundColor Cyan
    Write-Host "  css-once - Build CSS with Tailwind once without watching" -ForegroundColor Cyan
    Write-Host "  mail     - Start a development mail server" -ForegroundColor Cyan
    Write-Host "  help     - Show this help message" -ForegroundColor Cyan
    Write-Host ""
}

$target = $args[0]

switch ($target) {
    "css" {
        Build-CSS -Watch $true
    }
    "css-once" {
        Build-CSS -Watch $false
    }
    "mail" {
        Start-MailServer 
    }
    "help" {
        Show-Help
    }
    $null {
        Write-Host "No target specified. Use 'help' to see available targets." -ForegroundColor Red
        Show-Help
    }
    default {
        Write-Host "Unknown target: $target" -ForegroundColor Red
        Show-Help
    }
}