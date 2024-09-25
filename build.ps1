$Script:HelpText = @"
================================================================================
spike-rs build.ps1

USAGE:
./spike-rs.ps1 COMMAND

COMMANDS:

scoop         install scoop
python        install python (scoop is required)
rust          install the rust toolchain

================================================================================
"@


if ($IsWindows) {
  switch ($args[0]) {
    "scoop" {
      Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
      Invoke-RestMethod -Uri https://get.scoop.sh | Invoke-Expression
      scoop install python
    }

    "python" {
      scoop install python
    }

    "rust" {
      Invoke-WebRequest -Uri https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe -OutFile rustup-init.exe
      ./rustup-init.exe
    }

    default {
      Write-Host $Script:HelpText
    }
  }
}

if ($IsLinux) {
  switch ($args[0]) {
    default {
      Write-Host $Script:HelpText
    }
  }
}
