#!/usr/bin/env python3
import os

root = r"C:\Users\LenovoPC\Desktop\rustdesk-master"

print("--- Checking build.py for old rustdesk refs ---")
with open(os.path.join(root, "build.py"), "r") as f:
    content = f.read()

import re

for pattern in ["rustdesk", "RustDesk"]:
    for m in re.finditer(r".{0,40}" + pattern + r".{0,40}", content):
        line = m.group().strip()
        if "emudesk" not in line.lower() and "EmuDesk" not in line:
            print(f"  OLD: {line}")

print("--- Checking libs/ directories ---")
for dirpath, dirnames, filenames in os.walk(os.path.join(root, "libs")):
    for f in filenames:
        ext = os.path.splitext(f)[1].lower()
        if ext not in {".rs", ".toml", ".py", ".md"}:
            continue
        fp = os.path.join(dirpath, f)
        try:
            with open(fp, "r", encoding="utf-8", errors="replace") as fh:
                text = fh.read()
            for term in ["RustDesk", "rustdesk", "RUSTDESK"]:
                if term in text:
                    rel = os.path.relpath(fp, root)
                    print(f"  {rel}: contains {term}")
        except:
            pass

print("--- Checking flatpak/ ---")
for dirpath, dirnames, filenames in os.walk(os.path.join(root, "flatpak")):
    for f in filenames:
        fp = os.path.join(dirpath, f)
        try:
            with open(fp, "r", encoding="utf-8", errors="replace") as fh:
                text = fh.read()
            for term in ["RustDesk", "rustdesk"]:
                if term in text:
                    rel = os.path.relpath(fp, root)
                    print(f"  {rel}: contains {term}")
        except:
            pass

print("--- Checking docs/ ---")
for dirpath, dirnames, filenames in os.walk(os.path.join(root, "docs")):
    for f in filenames:
        ext = os.path.splitext(f)[1].lower()
        if ext not in {".md"}:
            continue
        fp = os.path.join(dirpath, f)
        try:
            with open(fp, "r", encoding="utf-8", errors="replace") as fh:
                text = fh.read()
            for term in ["RustDesk", "rustdesk"]:
                if term in text:
                    rel = os.path.relpath(fp, root)
                    print(f"  {rel}: contains {term}")
        except:
            pass

print("\nDone!")
