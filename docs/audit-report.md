# EmuDesk — Denetim Raporu (Audit Report)

Tarih: 2026-11-18
Kapsam: `D:\Games\Projeler\uzaktan_baglantı` kökü, cross-platform + GitHub CI/CD talimatı uygulanmadan önceki durum.

## 1. Depo durumu

| Öğe | Durum |
|-----|-------|
| Git | **Yok** — `git init` gerekli; GitHub remote yok, push yapılamaz. |
| Cargo workspace | members: `emudesk-core`, `emudesk-server`, `emudesk` |
| `emudesk-core` | identity + config — derlendi, 3 test geçti |
| `emudesk-server` | HTTP + WebSocket koordinasyon sunucusu (axum 0.7) — derlendi, 10 test geçti |
| `emudesk` | masaüstü mstsc launcher (egui/eframe) — derlendi, 11 test geçti |
| Dağıtım | Dockerfile, docker-compose.yml, .env.example, Procfile, railway.json |

## 2. Build / test özeti

- `emudesk-core`: 3 test ✓
- `emudesk-server`: 10 test ✓ (health, register/lookup, lookup 404, kalıcılık, online, ws hello)
- `emudesk`: 11 test ✓ (profile roundtrip/kalıcılık/doğrulama, rdp argüman formatı, .rdp'de parola yokluğu, sanitize, theme, help)
- Workspace toplam: 24 test, hepsi geçti.

## 3. Talimata göre eksikler

- [ ] Cross-platform (Linux): RDP backend soyutlaması yok; yalnızca Windows mstsc.exe. Linux FreeRDP gerekli.
- [ ] `cfg(target_os=...)` ayrımı.
- [ ] GitHub REST API entegrasyonu (release okuma, güncelleme kontrolü).
- [ ] CI/CD `.github/workflows/` (ci.yml, release.yml).
- [ ] SemVer + Release Please (release-please-config.json, .release-please-manifest.json).
- [ ] Paketleme (zip/tar.gz/SHA256SUMS) betikleri.
- [ ] Dokümantasyon (README, CHANGELOG, CONTRIBUTING, SECURITY, docs/*).

## 4. Güvenlik

- Parola command-line / log / düz metin `.rdp`'ye yazılmaz.
- `mstsc.exe` yamalanmaz / yeniden dağıtılmaz.
- Masaüstüne GitHub token gömülmez; yalnızca public release okuma.
- RDP 3389 gelişigüzel internete açılmaz; VPN/Gateway/relay ayrı aşama.

## 5. Not

Bu rapordaki tüm build/test sonuçları gerçek komut çıktılarıyla doğrulanmıştır. Linux GUI başlatma ve gerçek RDP bağlantısı bu Windows ortamında test edilememiştir; ayrı olarak raporlanır.