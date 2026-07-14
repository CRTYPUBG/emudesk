#!/usr/bin/env python3
"""Rename files and update content for remaining rustdesk references."""

import os
import shutil

root = r"C:\Users\LenovoPC\Desktop\rustdesk-master"

# Files to rename (old_name -> new_name)
file_renames = [
    (r"flatpak\rustdesk.json", r"flatpak\emudesk.json"),
    (
        r"flatpak\com.rustdesk.RustDesk.metainfo.xml",
        r"flatpak\com.emudesk.EmuDesk.metainfo.xml",
    ),
    (r"res\rustdesk-banner.svg", r"res\emudesk-banner.svg"),
    (r"res\pam.d\rustdesk.suse", r"res\pam.d\emudesk.suse"),
    (r"res\pam.d\rustdesk.debian", r"res\pam.d\emudesk.debian"),
    (
        r"res\msi\Package\Components\RustDesk.wxs",
        r"res\msi\Package\Components\EmuDesk.wxs",
    ),
]

# Do renames
for old_rel, new_rel in file_renames:
    old_path = os.path.join(root, old_rel)
    new_path = os.path.join(root, new_rel)

    if not os.path.exists(old_path):
        print(f"NOT FOUND: {old_rel}")
        continue

    # Read old content first
    try:
        with open(old_path, "r", encoding="utf-8", errors="replace") as f:
            content = f.read()
    except Exception as e:
        print(f"Error reading {old_rel}: {e}")
        continue

    # Apply content replacements
    content = content.replace("com.rustdesk.RustDesk", "com.emudesk.EmuDesk")
    content = content.replace("com.rustdesk", "com.emudesk")
    content = content.replace("RustDesk", "EmuDesk")
    content = content.replace("rustdesk", "emudesk")

    # Create new directory if needed
    os.makedirs(os.path.dirname(new_path), exist_ok=True)

    # Write new file
    with open(new_path, "w", encoding="utf-8") as f:
        f.write(content)

    # Remove old file
    os.remove(old_path)

    print(f"RENAMED: {old_rel} -> {new_rel}")

# Also handle files that need content changes but not name changes
extra_files = [
    r"res\logo-header.svg",
]

for rel in extra_files:
    fp = os.path.join(root, rel)
    if not os.path.exists(fp):
        print(f"NOT FOUND: {rel}")
        continue
    with open(fp, "r", encoding="utf-8", errors="replace") as f:
        content = f.read()

    # The SVG has an aria-label="RUSTDESK" that needs changing
    # But it also has path data that would be hard to fix
    # Just update the text labels
    content = content.replace("RUSTDESK", "EMUDESK")
    content = content.replace("RustDesk", "EmuDesk")

    with open(fp, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"UPDATED: {rel}")

print("\nDone!")
