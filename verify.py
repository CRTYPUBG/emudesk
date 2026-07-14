#!/usr/bin/env python3
import os

root = r"C:\Users\LenovoPC\Desktop\rustdesk-master"
files = [
    r"src\auth_2fa.rs",
    r"src\common.rs",
    r"src\lang.rs",
    r"build.py",
    r"res\rustdesk.desktop",
    r"res\rustdesk.service",
    r"flutter\lib\consts.dart",
    r"flutter\lib\desktop\pages\desktop_setting_page.dart",
    r"README.md",
]
for f in files:
    fp = os.path.join(root, f)
    if not os.path.exists(fp):
        print(f"NOT FOUND: {f}")
        continue
    with open(fp, "r", encoding="utf-8", errors="replace") as fh:
        content = fh.read()
    has_old = "RustDesk" in content or "rustdesk" in content
    has_new = "EmuDesk" in content or "emudesk" in content
    print(f"{f}: old={has_old}, new={has_new}")
