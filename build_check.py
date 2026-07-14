#!/usr/bin/env python3
"""Check build-critical files for remaining old references."""

import os

root = r"C:\Users\LenovoPC\Desktop\rustdesk-master"

critical_patterns = [
    # Files that MUST have no old references for build to work
    (r"Cargo.toml", ["name", "lib", "bin", "metadata"]),
    (r"build.rs", ["build"]),
    (r"build.py", ["build"]),
    (r"libs\portable\Cargo.toml", ["portable"]),
    (r"flutter\pubspec.yaml", ["flutter"]),
    (r"flutter\linux\CMakeLists.txt", ["linux cmake"]),
    (r"flutter\windows\runner\CMakeLists.txt", ["windows cmake"]),
    (r"flutter\macos\Runner\Runner.xcodeproj\project.pbxproj", ["xcode"]),
]

for rel, desc in critical_patterns:
    fp = os.path.join(root, rel)
    if not os.path.exists(fp):
        print(f"MISSING: {rel} ({desc})")
        continue
    with open(fp, "r", encoding="utf-8", errors="replace") as f:
        content = f.read()

    has_rustdesk = "RustDesk" in content
    has_rustdesk_lower = "rustdesk" in content
    has_emudesk = "EmuDesk" in content
    has_emudesk_lower = "emudesk" in content

    status = (
        "OK"
        if (has_emudesk or has_emudesk_lower)
        and not (has_rustdesk or has_rustdesk_lower)
        else "NEEDS REVIEW"
    )
    print(f"{rel}: {status}")

print("\n--- Checking libs/ directories ---")
for dirpath, dirnames, filenames in os.walk(os.path.join(root, "libs")):
    for f in filenames:
        ext = os.path.splitext(f)[1].lower()
        if ext not in {".rs", ".toml", ".py", ".md"}:
            continue
        fp = os.path.join(dirpath, f)
        try:
            with open(fp, "r", encoding="utf-8", errors="replace") as fh:
                content = fh.read()
            for term in ["RustDesk", "rustdesk", "RUSTDESK"]:
                if term in content and term not in content:  # always false trick
                    pass
                if content.find(term) != -1:
                    print(f"  FOUND in {os.path.relpath(fp, root)}: {term}")
        except:
            pass
