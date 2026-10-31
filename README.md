# EzMouse

Share one mouse and keyboard between two computers on the same network.
**Portable: no installer.** Unzip the folder, double-click `ezmouse.exe`.

> EzMouse is a modified version of [Mousehop](https://github.com/jondkinney/mousehop),
> which is a fork of [lan-mouse](https://github.com/feschber/lan-mouse) by Ferdinand
> Schober and contributors. Licensed under GPL-3.0-or-later. See `NOTICE`.

## What's different from Mousehop

- **Portable.** All settings, the device identity certificate and crash logs live
  in a `data` folder next to `ezmouse.exe` (Windows). Copy the folder to move it,
  delete it to uninstall. Nothing is written to `%LOCALAPPDATA%`, the registry or
  the Start Menu. Set `EZMOUSE_DATA_DIR` to put the data folder somewhere else.
- Renamed (`ezmouse.exe`, `ezmouse.pem`, `_ezmouse._udp` discovery), so EzMouse
  and Mousehop installs won't talk to each other.

Everything else (encrypted DTLS transport, edge-crossing, clipboard text sync,
release hotkey, multi-monitor layouts) is inherited from Mousehop.

## Getting a build (Windows)

You don't need to set up a compiler:

1. Put this folder in a GitHub repository.
2. Go to **Actions → "Build EzMouse (Windows portable)" → Run workflow**.
   (First run builds GTK and takes about an hour; later runs take minutes.)
3. Download the `EzMouse-windows-x86_64` artifact. Unzip it on both PCs.

To build locally instead, follow "Windows" under *Dependencies* in the upstream
Mousehop README (Rust + GTK4/libadwaita via gvsbuild), then `cargo build --release`.
Copy `target\release\ezmouse.exe` and the GTK DLLs from
`C:\gtk-build\gtk\x64\release\bin` into one folder.

## Using it

See `packaging/README-PORTABLE.txt` (also included inside the zip).

Windows Firewall must allow UDP port 4252 on private networks.
