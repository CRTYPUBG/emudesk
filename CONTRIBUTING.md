# Katkı Rehberi

EmuDesk'e katkıda bulunduğunuz için teşekkürler! Bu rehber, projeye katkı sürecinde izlemeniz gereken akışı ve kuralları açıklar.

## İçindekiler

- [Davranış Beklentisi](#davranış-beklentisi)
- [Başlarken](#başlarken)
- [Branch ve PR Akışı](#branch-ve-pr-akışı)
- [Conventional Commits](#conventional-commits)
- [Test Etme](#test-etme)
- [Biçimlendirme ve Clippy](#biçimlendirme-ve-clippy)
- [Güvenlik](#güvenlik)

## Davranış Beklentisi

Katkılar nazik, kapsayıcı ve yapıcı bir ortamda yürütülür. Baskıcı, aşağılayıcı veya saldırgan dil kullanılmaz. Yapıcı eleştiriler teknik içeriğe odaklanır.

## Başlarken

1. Depoyu klonlayın ve bir geliştirme dalı açın.
2. Yerel ortamınızda Rust (`rustup`) ve Cargo'un hazır olduğundan emin olun.
3. Değişikliğinize başlamadan önce, büyük veya tartışmalı değişiklikler için önce bir issue açarak yaklaşımınızı doğrulayın.

## Branch ve PR Akışı

Proje, basit bir branch + pull request akışı kullanır:

1. `main` dalı kararlı olarak kabul edilir.
2. Her değişiklik için `main`'den bir **feature branch** açın:

   ```bash
   git checkout main
   git pull
   git checkout -b feat/ozellik-adi
   ```

3. Değişikliğinizi yapın, test edin, biçimlendirin ve clippy'i temiz bırakın.
4. Branch'inizi push edin ve `main` üzerine bir **Pull Request (PR)** açın.
5. PR başlığı ve açıklaması değişikliğin ne olduğunu ve nedenini net biçimde anlatmalıdır.
6. En az bir gözden geçirenin onayıyla PR birleştirilebilir.

## Conventional Commits

Commit mesajları [Conventional Commits](https://www.conventionalcommits.org/) kuralına uyar:

- **`feat:`** — yeni bir özellik ekler.
- **`fix:`** — bir hatayı düzeltir.
- **`docs:`** — dokümantasyon değişikliği.
- **`refactor:`** — davranışı değiştirmeyen kod yeniden düzenlemesi.
- **`test:`** — test ekleme/düzeltme.
- **`chore:`** — bakım, derleme, bağımlılık işleri.
- **`BREAKING CHANGE:`** — uyumsuz bir davranış değişikliği yapar.

Sürüm numaralandırması [SemVer](https://semver.org/) ilkelerini izler:

- `BREAKING CHANGE` → ana sürüm artar (ör. `1.0.0` → `2.0.0`).
- `feat` → alt sürüm artar (ör. `1.0.0` → `1.1.0`).
- `fix` → yama sürümü artar (ör. `1.0.0` → `1.0.1`).

Örnek:

```bash
feat(emudesk): çoklu monitör desteğini profillere ekle

BREAKING CHANGE: profil şeması yeni monitör alanı içerir.
```

## Test Etme

- Workspace'teki tüm testleri çalıştırın:

  ```bash
  cargo test --workspace
  ```

- Yalnızca tek bir paketi test etmek için:

  ```bash
  cargo test -p emudesk-core
  cargo test -p emudesk-server
  cargo test -p emudesk
  ```

- Yeni mantık eklerken ilgili birim testleri ekleyin.

## Biçimlendirme ve Clippy

- Kodu `rustfmt` ile biçimlendirin:

  ```bash
  cargo fmt --all -- --check
  ```

- `clippy` uyarılarını temiz tutun:

  ```bash
  cargo clippy --workspace --all-targets -- -D warnings
  ```

PR'ler yalnızca format ve clippy temizse kabul edilir.

## Güvenlik

Güvenlikle ilgili herhangi bir bulgu için **public issue açmayın**; [SECURITY.md](SECURITY.md) içinde belirtilen özel bildirim yolunu kullanın. Ayrıca projenin güvenlik ilkelerine (parolaların komut satırına/loga/.rdp dosyasına yazılmaması, `mstsc.exe`'nin yamalanmaması, EmuDesk ID'nin tek başına erişim hakkı sayılmaması) uyulması zorunludur.