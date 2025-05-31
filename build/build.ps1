# logs a status message
function Write-Status([string] $message, [switch] $ForError) {
    if ($forError) {
        Write-Host -NoNewLine "[ERROR] " -ForegroundColor DarkRed
    }
    else {
        Write-Host -NoNewLine "[INFO] " -ForegroundColor DarkCyan
        Write-Host -NoNewLine '✓ ' -ForegroundColor DarkGreen
    }
    Write-Host $message -ForegroundColor Gray
}

# runs a command, shows it's progress, and logs a status message
function Write-CommandProgress([string] $command, [string] $message) {
    # show initial message block
    Write-Host -NoNewLine "[INFO] " -ForegroundColor DarkCyan
    Write-Host -NoNewLine "⣾ " -ForegroundColor White
    Write-Host -NoNewLine "$message" -ForegroundColor Gray

    # run the command as a separate process without creating a new window
    $commandProcess = Start-Process powershell.exe -NoNewWindow -ArgumentList $command -PassThru -RedirectStandardOutput "NUL"

    # run the loading animation while the command is running
    $loadingDots = @("⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷")
    $initialPosition = $host.UI.RawUI.CursorPosition
    $animationPosition = $initialPosition
    $animationPosition.X -= $message.Length + 2
    $idx = 0
    while ($commandProcess.HasExited -ne $true) {
        $host.UI.RawUI.CursorPosition = $animationPosition
        Write-Host -NoNewline $loadingDots[$idx] -ForegroundColor White
        $idx++
        if ($idx -ge $loadingDots.Length) { $idx = 0 }
        Start-Sleep -Milliseconds 100
    }
    $host.UI.RawUI.CursorPosition = $animationPosition
    Write-Host -NoNewLine '✓ ' -ForegroundColor DarkGreen
    $host.UI.RawUI.CursorPosition = $initialPosition
    Write-Host ''
}

# test all paths that are necessary for the build process, and
# exits the script if one of them does not exist.
function Test-AllBuildFilePaths([string] $virtualBoxPath) {
    Write-Host ""
    Write-Status "confirming all necessary build paths exist"
    $buildPaths = @($virtualBoxPath)
    foreach($buildPath in $buildPaths) {
        if (-not (Test-Path -Path $buildPath)) {
            Write-Status -ForError "Path '$buildPath' not found"
            exit 1
        }
    }
}

# cleans up any left over files from previous builds, and removes
# any cached virtual box configurations from previous boots.
function Unpublish-PreviousBuilds {
    Write-Status "cleaning up any previous builds"
    # change directory to be able to run 'cargo' commands
    Push-Location $micrusRootPath
    # run 'cargo clean' to remove any leftover files from previous builds
    & cargo clean 2>&1 | Out-Null
    # remove any leftover virtualbox xml information for the old builds
    Remove-Item -ErrorAction SilentlyContinue -Path "C:\Users\evang\.VirtualBox\VirtualBox.xml"
}

# builds the micrus microkernel boot image, and exits the script on failure.
function Build-BootImage {
    # run 'cargo bootimage' to create a new microkernel .bin file
    Write-CommandProgress "cargo bootimage" "starting micrus microkernel build"
    if ($LASTEXITCODE -ne 0) {
        Write-Status -ForError "micrus microkernel build failed"
        exit $LASTEXITCODE
    }
}

# pads the .bin microkernel file and converts to .img using dd,
# because virtualbox needs at least a 2MB file to convert to .vdi.
function Convert-MicroKernelImage([string] $micrusRootPath, [string] $virtualBoxPath) {
    # build necessary paths to files for converting the microkernel image
    $ddPath = "C:\Program Files\Git\usr\bin\dd.exe"
    $binFile = Join-Path -Path $micrusRootPath -ChildPath "target/x86_64-micrus/debug/bootimage-micrus.bin"
    $imgFile = Join-Path -Path $micrusRootPath -ChildPath "target/x86_64-micrus/debug/bootimage-micrus.img"
    $vdiFile = Join-Path -Path $micrusRootPath -ChildPath "target/x86_64-micrus/debug/bootimage-micrus.vdi"
    $virtualBoxManageFile = Join-Path -Path $virtualBoxPath -ChildPath "VBoxManage.exe"
    # create an empty 2MB file
    & $ddPath if=/dev/zero of=$imgFile bs=1M count=2 2>&1 | Out-Null
    # copy the .bin into the start of the 2MB .img
    & $ddPath if=$binFile of=$imgFile bs=512 conv=notrunc seek=0 2>&1 | Out-Null
    # use VBoxManage.exe to convert from .img to .vdi
    & $virtualBoxManageFile convertfromraw --format VDI $imgFile $vdiFile 2>&1 | Out-Null
    Write-Status "micrus microkernel image ready: $vdiFile`n"
}

# builds the micrus microkernel from start to finish.
function Build-MicroKernel([string] $micrusRootPath, [string] $virtualBoxPath) {
    # try to run the build sequence
    try {
        Test-AllBuildFilePaths $virtualBoxPath
        Unpublish-PreviousBuilds
        Build-BootImage
        Convert-MicroKernelImage $micrusRootPath $virtualBoxPath
    }
    # report any unexpected errors
    catch {
        Write-Status -ForError $_
        exit 1
    }
    # always pop the location back, even through an error or exit
    finally { Pop-Location -ErrorAction SilentlyContinue }
}

Build-MicroKernel "C:/projects/micrus" "C:/Program Files/Oracle/VirtualBox"