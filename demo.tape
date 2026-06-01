# ApeGuard Demo GIF — VHS Tape
# Generates an animated terminal GIF for the README
#
# Prerequisites: brew install vhs
# Usage:       vhs demo.tape
#
# For best results, run in a clean terminal with a dark theme.
# The demo shows basic commands: version check, help, and a scan.

Output apeguard-demo.gif

Set FontSize 16
Set Width 800
Set Height 480
Set Padding 32
Set Framerate 12
Set Theme "Dracula"
Set TypingSpeed 80ms
Set Loop 0

# === Frame 1: Version ===
Type "apeguard version" 300ms
Sleep 500ms
Enter
Sleep 1500ms

# === Frame 2: Help ===
Type "apeguard scan --help" 300ms
Sleep 500ms
Enter
Sleep 2000ms

# === Frame 3: Quick scan ===
Type "apeguard scan --layers 1 --format summary" 300ms
Sleep 500ms
Enter
Sleep 3000ms

# === Frame 4: Final output ===
Sleep 2000ms
