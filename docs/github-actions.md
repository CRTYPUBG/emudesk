# GitHub Actions

Bu repo, projeyi doğrulamak ve sürüm paketleri üretmek için iki GitHub Actions
workflow'u kullanır:

- `.github/workflows/ci.yml` — her push / pull request'te doğrulama.
- `.github/workflows/release.yml` — `v*` etiketi (tag) itildiğinde release paketleri.

---

## CI — `ci.yml`

### Tetikleyiciler
- `push` → `main` ve `master` dallarına.
- `pull_request` → tüm PR'ler.
- `workflow_dispatch` → GitHub UI üzerinden elle tetiklenebilir.

### Matrix
- İşletim sistemleri: `ubuntu-latest` ve `windows-latest`.
- Her iki platformda da aynı adımlar çalışır.

### İzinler
- İş seviyesinde `permissions: contents: read`. CI yalnızca okur; hiçbir şey
  yazmaz, yayınlamaz. En az ayrıcalık ilkesi uygulanır.

### Cache
- `Swatinem/rust-cache@v2` kullanılır (güncel upstream sürümü). Bu, Cargo
  kayıtlarını ve hedef dizinleri cache'ler ve hem `cargo fmt`/`clippy`/`test`
  hem de release build adımlarını hızlandırır.

### Adımlar
1. `actions/checkout@v4` — repo içeriğini çeker.
2. `dtolnay/rust-toolchain@stable` — Rust **stable** kurulur, `rustfmt` ve
   `clippy` component'leri eklenir.
3. `Swatinem/rust-cache@v2` — Cargo cache.
4. **Linux OS bağımlılıkları** — egui/eframe ve ring (OpenSSL) gereksinimleri:
   ```
   sudo apt-get update
   sudo apt-get install -y libgtk-3-dev libxcb-render0-dev \
       libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
   ```
   Bu adım `if: runner.os == 'Linux'` ile sadece Ubuntu runner'da çalışır.
5. **Format check** — `cargo fmt --all -- --check`
6. **Clippy** — `cargo clippy --workspace --all-targets -- -D warnings`
7. **Test** — `cargo test --workspace`
8. **Release build** — `cargo build --release`

> **Not:** `continue-on-error` kullanılmaz. Herhangi bir adım başarısız olursa
> iş **başarısız** sayılır ve PR engellenir.

---

## Release — `release.yml`

### Tetikleyiciler
- `push` → `v*` etiketleri (örn. `v0.2.0`).
- `workflow_dispatch` → elle tetiklenebilir.

### İzinler
- `permissions: contents: write` **sadece bu işte** (release workflow'unda).
- `release` işi ayrıca `permissions: contents: write` ile release oluşturmak
  için gereken yazma yetkisine sahiptir.

### İş akışı (iki iş)

**1. `release-build`** (matrix: `ubuntu-latest`, `windows-latest`)
- `cargo build --release` ile release binary üretir.
- `VERSION` tag'dan çıkarılır: `${GITHUB_REF_NAME#v}` (örn. `v0.2.0` → `0.2.0`).
- Windows → `scripts/package.ps1` ile `EmuDesk-v{VERSION}-windows-x86_64.zip`.
- Linux → `scripts/package.sh` ile `EmuDesk-v{VERSION}-linux-x86_64.tar.gz`.
- Her iki betik de `SHA256SUMS.txt` üretir.
- Paketler `actions/upload-artifact@v4` ile bir sonraki işe aktarılır.

**2. `release`** (`needs: release-build`, `ubuntu-latest`)
- `actions/download-artifact@v4` ile tüm platform paketlerini indirir
  (`merge-multiple: true`).
- `softprops/action-gh-release@v2` (güncel upstream) ile bir GitHub Release
  oluşturur: `draft: true`, `generate_release_notes: true`. ZIP, tar.gz ve
  `SHA256SUMS.txt` asset olarak yüklenir.

### Duplicate-safe notu
Release aynı etiket için yeniden tetiklenirse, `softprops/action-gh-release@v2`
mevcut release'i günceller (taslak olsa dahi) ve aynı asset adlarını üzerine
yazar. Taslak (`draft: true`) olduğu için içerik yayınlanmadan önce
incelenebilir.

---

## Paketleme betikleri

- `scripts/package.ps1` — Windows (`pwsh`). `target/release/emudesk.exe`'yi
  alır, `EmuDesk-v{VERSION}-windows-x86_64.zip` yapar. LICENSE ve README.md
  **varsa** pakete ekler (yoksa uyarı basar, hata vermez), `SHA256SUMS.txt`
  üretir. Dosya adlarını ve varlığını doğrular.
- `scripts/package.sh` — Linux (`bash`). `target/release/emudesk`'i alır,
  `EmuDesk-v{VERSION}-linux-x86_64.tar.gz` yapar. LICENSE ve README.md **varsa**
  ekler, `sha256sum` ile `SHA256SUMS.txt` üretir.