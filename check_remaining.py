#!/usr/bin/env python3
import os
import sys

root = r"C:\Users\LenovoPC\Desktop\rustdesk-master"
count = 0
for dirpath, dirnames, filenames in os.walk(root):
    rel = os.path.relpath(dirpath, root).replace("\\", "/")
    parts = rel.split("/")
    if ".git" in parts or "target" in parts or ".cargo" in parts:
        continue
    for f in filenames:
        ext = os.path.splitext(f)[1].lower()
        if ext in {".png", ".ico", ".jpg", ".lock"}:
            continue
        fp = os.path.join(dirpath, f)
        try:
            with open(fp, "r", encoding="utf-8", errors="replace") as fh:
                content = fh.read()
            for term in ["RustDesk", "rustdesk", "RUSTDESK", "librustdesk"]:
                idx = content.find(term)
                if idx != -1:
                    start = max(0, idx - 20)
                    end = min(len(content), idx + len(term) + 20)
                    context = content[start:end].replace("\n", " ")
                    print(f"{os.path.relpath(fp, root)}:{idx} ...{context}...")
                    count += 1
        except Exception as e:
            pass
print(f"\nTotal remaining refs: {count}")
