#!/usr/bin/env bash
set -euo pipefail

# Kullanım: ./scripts/package.sh <VERSION>   (örn. 0.2.0)
VERSION="${1:?VERSION argümanı zorunludur}"

BIN="target/release/emudesk"
TARBALL="EmuDesk-v${VERSION}-linux-x86_64.tar.gz"
SUMS="SHA256SUMS.txt"
STAGING="$(mktemp -d)"
trap 'rm -rf "$STAGING"' EXIT

if [[ ! -f "$BIN" ]]; then
    echo "HATA: Derlenmiş binary bulunamadı: $BIN" >&2
    exit 1
fi

cp "$BIN" "$STAGING/emudesk"

# LICENSE ve README varsa ekle (yoksa zarifçe atla)
if [[ -f "LICENSE" ]]; then
    cp "LICENSE" "$STAGING/"
else
    echo "UYARI: LICENSE bulunamadı; pakete dahil edilmiyor." >&2
fi
if [[ -f "README.md" ]]; then
    cp "README.md" "$STAGING/"
else
    echo "UYARI: README.md bulunamadı; pakete dahil edilmiyor." >&2
fi

tar -czf "$TARBALL" -C "$STAGING" .

# SHA256 özetini üret
sha256sum "$TARBALL" > "$SUMS"

echo "Paket oluşturuldu: $TARBALL"
echo "Özet dosyası: $SUMS"