# GitHub Release Kurulumu

Bu doküman, `v*` etiketli sürüm yayınlama akışını (Release Please) ve gerekli
token izinlerini açıklar.

---

## Genel akış

1. **Release Please** (`release-please-config.json` + `.release-please-manifest.json`)
   commit mesajlarından (Conventional Commits) sürümü belirler, `CHANGELOG.md`
   ve `v*` etiketini oluşturur.
2. Etiket itildiğinde `.github/workflows/release.yml` tetiklenir.
3. Workflow, Windows ve Linux için release paketlerini üretir ve GitHub Release
   oluşturur (taslak olarak).

> **Not:** `.github/workflows/release.yml` içindeki `softprops/action-gh-release@v2`
> ile `permissions: contents: write` gereklidir. Release Please'ın kendisi bir
> **bot/action** olarak ayrıca izinlere ihtiyaç duyar (aşağıya bakın).

---

## Yapılandırma dosyaları

- `release-please-config.json` — `"release-type": "rust"`, `"cargo-workspace"`
  plugin'i ile workspace'teki tüm crate'lerin sürümünü senkronize eder.
  `bump-minor-pre-major` ve `bump-patch-for-minor-pre-major` 0.x sürümlerinde
  daha agresif artışlar sağlar.
- `.release-please-manifest.json` — her paketin **mevcut** sürümünü tutar:
  `{ ".": "0.1.0" }`. Release Please bu dosyayı günceller.

### Workspace'e Release Please eklemek

Release Please'ı bir GitHub Action olarak çalıştırmak için ayrı bir workflow
gerekir (bu repoda bulunmayan, isteğe bağlı `release-please.yml`). Örnek:

```yaml
on:
  push:
    branches: [main]
permissions:
  contents: write
  pull-requests: write
  issues: write
jobs:
  release-please:
    runs-on: ubuntu-latest
    steps:
      - uses: googleapis/release-please-action@v4
        with:
          config-file: release-please-config.json
          manifest-file: .release-please-manifest.json
```

> Yukarıdaki izinler yalnızca Release Please action'ı içindir. Etikete bağlı
> **release paketleme** workflow'u `.github/workflows/release.yml`'dedir.

---

## Token izinleri (GITHUB_TOKEN)

`GITHUB_TOKEN` varsayılan olarak yalnızca çalışan depoya özgü izinlere sahiptir.
Aşağıdaki görevler için ilgili izinleri açmanız gerekir.

### Release paketleme işi (`.github/workflows/release.yml`)
| İzin | Gerekçe |
|------|---------|
| `contents: write` | GitHub Release oluşturmak ve asset yüklemek için. |

### Release Please action'ı (isteğe bağlı, `release-please.yml`)
| İzin | Gerekçe |
|------|---------|
| `contents: write` | `CHANGELOG.md` ve manifesti güncellemek, `v*` etiketi oluşturmak. |
| `pull-requests: write` | Sürüm PR'ı oluşturmak/güncellemek. |
| `issues: write` | PR/issue'larla ilişkili bot yorumları ve etiketler için. |

Bunları ilgili işin (veya workflow'un) `permissions:` bloğuna **açıkça**
ekleyin. En az ayrıcalık ilkesini koruyun: sadece gereken işlerde verin.

---

## GITHUB_TOKEN vs. klasik PAT

- **Varsayılan: `GITHUB_TOKEN`** kullanın. Depo sınırlı, otomatik olarak
  oluşturulur ve süresi dolmaz.
- **Klasik PAT'yi varsayılan YAPMAYIN.** Klasik PAT'ler tüm depolarınıza
  erişebilir ve süreleri dolarak bakım yükü ve güvenlik riski oluşturur.
- Bir **fine-grained (kişiselleştirilmiş) GitHub App token'ı** kullanmanız
  gerekiyorsa, yalnızca bu depoya ve gerekli izinlere
  (`contents: write`, `pull-requests: write`, `issues: write`) sınırlayın.

### Secret değerlerini asla yazmayın
Token'ları asla doğrudan workflow dosyasına veya bu dokümana yazmayın. GitHub
**Settings → Secrets and variables → Actions** içinde `D.SECRET_ADI` olarak
tanımlayın ve workflow'da şöyle kullanın:

```yaml
env:
  GITHUB_TOKEN: ${{ secrets.SECRET_ADI }}
```

Gerçek secret değerini hiçbir dosyada, komutta veya logda paylaşmayın.

---

## `v*` etiket akışı

1. Release Please `v{sürüm}` etiketini (örn. `v0.2.0`) oluşturur.
2. Etiket push'u `.github/workflows/release.yml`'yi tetikler.
3. Workflow `${GITHUB_REF_NAME#v}` ile `0.2.0`'ı çıkarır ve paket adlarında
   kullanır: `EmuDesk-v0.2.0-windows-x86_64.zip` vb.
4. Release **taslak** (`draft: true`) olarak oluşturulur; yayınlamadan önce
   içeriği inceleyebilirsiniz.

> Tag'lar iş (job) tetiklendikten sonra değiştirilirse, `softprops/action-gh-release@v2`
> aynı etiket için mevcut release'i günceller; bu da duplicate-safe davranıştır.