#!/usr/bin/env python3
import os
import sys

root = r"C:\Users\LenovoPC\Desktop\emudesk-master"

extensions = {
    ".rs",
    ".toml",
    ".py",
    ".dart",
    ".xml",
    ".yaml",
    ".yml",
    ".json",
    ".desktop",
    ".service",
    ".spec",
    ".sh",
    ".md",
    ".txt",
    ".conf",
    ".cc",
    ".mm",
    ".cpp",
    ".h",
    ".cmake",
    ".rc",
    ".properties",
    ".cfg",
    ".iss",
    ".po",
    ".svg",
    ".css",
    ".html",
    ".js",
    ".ts",
}

exclude_dirs = {
    ".git",
    ".github",
    ".cargo",
    "target",
    "build",
    "node_modules",
    "fastlane/metadata/android",
}
exclude_files = {"Cargo.lock"}
exclude_suffixes = {
    ".png",
    ".ico",
    ".jpg",
    ".gif",
    ".bmp",
    ".woff",
    ".woff2",
    ".ttf",
    ".eot",
    ".zip",
    ".gz",
}

replacements = [
    ("com.emudesk.emudesk", "com.emudesk.emudesk"),
    ("com.emudesk.EmuDesk", "com.emudesk.EmuDesk"),
    ("org.emudesk.EmuDesk", "org.emudesk.EmuDesk"),
    ("org.emudesk.emudesk", "org.emudesk.emudesk"),
    ("EmuDeskIddDriver", "EmuDeskIddDriver"),
    ("EmuDeskIdd", "EmuDeskIdd"),
    ("EmuDeskPrivacyWindowClass", "EmuDeskPrivacyWindowClass"),
    ("EmuDeskPrivacyWindow", "EmuDeskPrivacyWindow"),
    ("WM_EMUDESK_SHOW_WINDOWS", "WM_EMUDESK_SHOW_WINDOWS"),
    ("WM_EMUDESK_HIDE_WINDOWS", "WM_EMUDESK_HIDE_WINDOWS"),
    ("EMUDESK_IDD_DEVICE_STRING", "EMUDESK_IDD_DEVICE_STRING"),
    ("IDD_IMPL_EMUDESK", "IDD_IMPL_EMUDESK"),
    ("__EMUDESK_SUDO_E_TEST_", "__EMUDESK_SUDO_E_TEST_"),
    ("EMUDESK_APPNAME", "EMUDESK_APPNAME"),
    ("emudesk_idd", "emudesk_idd"),
    ("EmuDeskInterval", "EmuDeskInterval"),
    ("MsgToEmuDesk", "MsgToEmuDesk"),
    ("msg_to_emudesk", "msg_to_emudesk"),
    ("delete_emudesk_test_certs", "delete_emudesk_test_certs"),
    ("DeleteEmuDeskTestCerts", "DeleteEmuDeskTestCerts"),
    ("is_emudesk", "is_emudesk"),
    ("EmuDeskMultiWindowManager", "EmuDeskMultiWindowManager"),
    ("emuDeskWinManager", "emuDeskWinManager"),
    ("EmuDeskVirtualDisplays", "EmuDeskVirtualDisplays"),
    (
        "kPlatformAdditionsEmuDeskVirtualDisplays",
        "kPlatformAdditionsEmuDeskVirtualDisplays",
    ),
    ("isEmuDeskIdd", "isEmuDeskIdd"),
    ("EmuDesk.app", "EmuDesk.app"),
    ("EmuDesk-Admin", "EmuDesk-Admin"),
    ("EmuDesk Server Pro", "EmuDesk Server Pro"),
    ("EmuDesk Doc", "EmuDesk Doc"),
    ("EmuDesk UI", "EmuDesk UI"),
    ("EmuDesk", "EmuDesk"),
    ("emudesk", "emudesk"),
    ("libemudesk", "libemudesk"),
]

modified_files = []

for dirpath, dirnames, filenames in os.walk(root):
    rel = os.path.relpath(dirpath, root).replace("\\", "/")
    parts = rel.split("/")
    skip = False
    for ex in exclude_dirs:
        if ex in parts:
            skip = True
            break
    if skip:
        continue

    for filename in filenames:
        if filename in exclude_files:
            continue
        ext = os.path.splitext(filename)[1].lower()
        if ext in exclude_suffixes:
            continue
        if ext not in extensions:
            continue

        filepath = os.path.join(dirpath, filename)
        try:
            with open(filepath, "r", encoding="utf-8", errors="replace") as f:
                content = f.read()
            original = content
            for old, new in replacements:
                content = content.replace(old, new)
            if content != original:
                with open(filepath, "w", encoding="utf-8") as f:
                    f.write(content)
                modified_files.append(os.path.relpath(filepath, root))
                print(f"  M: {os.path.relpath(filepath, root)}")
        except Exception as e:
            sys.stderr.write(f"  ERR: {os.path.relpath(filepath, root)}: {e}\n")

print(f"\nTotal modified files: {len(modified_files)}")
