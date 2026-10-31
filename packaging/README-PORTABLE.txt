EzMouse - share one mouse and keyboard between two computers
=============================================================

NOTHING TO INSTALL. Everything lives in this folder.

1. Unzip this folder anywhere you can write to (Desktop, Documents, a USB stick).
   Don't put it in C:\Program Files.
2. Do the same on the second computer.
3. Double-click ezmouse.exe on BOTH computers.
4. Windows Firewall will ask for permission the first time. Tick "Private
   networks" and click Allow (EzMouse talks over UDP port 4252).

CONNECTING THE TWO COMPUTERS
  Computer A has the mouse + keyboard plugged in; Computer B is the one you
  want to reach by pushing the mouse off the screen edge.

  On A: click "Add", type B's computer name (or pick it if it's auto-discovered),
        and choose which side of A's screen B sits on (left / right / top / bottom).
  On B: under "Incoming Connections", click "Authorize" for A.
        (A's fingerprint is shown in A's "General" section - check they match.)
  Now push the mouse off that edge of A's screen and it appears on B.

  Want to share in both directions? Repeat the steps with A and B swapped.

EMERGENCY RELEASE
  If the mouse seems "stuck" on the other computer, press the release
  keys (default: A + S + D + F together) on the keyboard, or unplug the network.

WHERE YOUR SETTINGS ARE
  The "data" folder next to ezmouse.exe holds your settings, this computer's
  identity certificate, and crash logs. Copy the whole EzMouse folder to move
  everything, or delete the folder to remove EzMouse completely. EzMouse does
  not touch the registry, Start Menu, or your user profile.

  Closing the window hides EzMouse to the system tray (near the clock) and keeps
  sharing. Right-click the tray icon > Quit to stop it.

EzMouse is free software (GPL-3.0). It is a modified version of Mousehop, itself
a fork of lan-mouse. See LICENSE and NOTICE.
