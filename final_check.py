#!/usr/bin/env python3
"""Final comprehensive check for remaining old references."""

import os

root = r"C:\Users\LenovoPC\Desktop\rustdesk-master"

exclude_dirs = {".git", ".github", ".cargo", "target", "node_modules", ".ruff_cache"}
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
    ".lock",
}
exclude_files = {
    "rename_script.py",
    "rename_files.py",
    "check_remaining.py",
    "verify.py",
}

total_count = 0
files_with_old = []

for dirpath, dirnames, filenames in os.walk(root):
    rel = os.path.relpath(dirpath, root).replace("\\", "/")
    parts = rel.split("/")
    if any(ex in parts for ex in exclude_dirs):
        continue

    for f in filenames:
        if f in exclude_files:
            continue
        ext = os.path.splitext(f)[1].lower()
        if ext in exclude_suffixes:
            continue

        fp = os.path.join(dirpath, f)
        file_has_old = False
        try:
            with open(fp, "r", encoding="utf-8", errors="replace") as fh:
                content = fh.read()

            # Skip if it's a generated/binary file
            if not content or len(content) < 10:
                continue

            for term in ["RustDesk", "rustdesk", "RUSTDESK", "librustdesk"]:
                idx = content.find(term)
                if idx != -1:
                    if not file_has_old:
                        files_with_old.append(rel + "/" + f)
                        file_has_old = True
                    start = max(0, idx - 30)
                    end = min(len(content), idx + len(term) + 30)
                    context = content[start:end].replace("\n", " ")
                    print(f"  [{term}] {rel}/{f}: ...{context}...")
                    total_count += 1
        except:
            pass

print(f"\nTotal remaining references: {total_count}")
print(f"Files with old references: {len(files_with_old)}")
