# EmuDesk — Cross-Platform Rust RDP Client
## Windows + Linux • GitHub API • Automatic Versioning • CI/CD • Releases

> **Görev:** Bu talimatı mevcut EmuDesk deposunun kökünde çalışan kodlama ajanı uygular. Yalnızca öneri yazıp durma; repoyu incele, kodu ve yapılandırmaları gerçek dosyalar üzerinde uygula, test et ve sonuçları raporla. GitHub hesabına/deposuna erişimin yoksa bunu açıkça belirt; erişimin varmış gibi davranma.

## 0. Ürün tanımı ve temel kararlar

EmuDesk, Rust ile geliştirilen, Windows ve Linux üzerinde derlenebilen, özgün arayüze sahip bir uzak masaüstü/RDP istemcisidir. Windows RDP bağlantısında Windows'un yerleşik `mstsc.exe` istemcisi kullanılabilir. Linux'ta `mstsc.exe` bulunmadığı için RDP motoru platform adaptörü arkasında soyutlanmalı; ilk sürümde sistemde mevcutsa FreeRDP istemcisi (`xfreerdp`, dağıtıma göre `xfreerdp3`) kullanılmalıdır.

Ürün kapsamını doğru ayır:

- **Windows build:** EmuDesk Rust arayüzü + Windows RDP bağlantı adaptörü.
- **Linux build:** Aynı Rust arayüzü + Linux RDP bağlantı adaptörü; FreeRDP bağımlılığını kontrol et, sürümünü/komut yolunu algıla ve kurulum talimatı göster.
- **Yerel kullanım:** EmuDesk ID sunucusu yokken IP/hostname ile doğrudan RDP bağlantısı çalışabilmeli.
- **EmuDesk ID:** ID, kayıt/eşleştirme mekanizmasıdır; kendiliğinden RDP tüneli değildir. ID'nin IP/hostname çözümlemesi, güvenli erişim yetkisi ve NAT arkasından bağlantı birbirinden ayrı konulardır.
- **Bulut koordinatörü:** Railway benzeri bir container platformunda çalışabilecek ayrı servis. Koordinatör çalışmıyorsa doğrudan yerel RDP bağlantısı bozulmamalı.
- **İnternet üzerinden RDP:** Doğrudan erişim, VPN, RD Gateway veya güvenli tünel/relay ihtiyacı açıkça değerlendirilmelidir. RDP 3389 portunu gelişigüzel internete açmayı önerme. Railway'de HTTP hizmetinin çalışması, genel amaçlı uzun ömürlü RDP relay'in otomatik olarak çalıştığı anlamına gelmez.

Mevcut projede başka bir yapı zaten varsa yeniden yazmak yerine önce onu incele ve ölçülebilir gerekçe olmadan değiştirme.

## 1. Başlamadan önce depo denetimi

İlk değişiklikten önce:

1. Kök dizini ve önemli tüm klasörleri incele.
2. `README.md`, `AGENTS.md`, `Cargo.toml`, `Cargo.lock`, varsa `rust-toolchain.toml`, `.cargo/config.toml`, `src/`, `tests/`, `.github/workflows/`, lisanslar, paketleme betikleri ve mevcut sürüm notlarını oku.
3. Mevcut branch, git durumu, son commit, mevcut tag/release düzeni ve çalışma ağacındaki kullanıcı değişikliklerini tespit et. Kullanıcı değişikliklerini silme veya üzerine yazma.
4. Var olan build/test komutlarını çalıştır ve başlangıç sonuçlarını kaydet.
5. Windows'a özel bağımlılıkların Linux derlemesini engelleyip engellemediğini kontrol et. Platforma özel kodu `cfg(target_os = "windows")` / `cfg(target_os = "linux")` arkasında ayır.
6. Mevcut sürümü ve GitHub API kullanımını belirle.
7. Önce kısa bir `docs/audit-report.md` veya eşdeğer rapor oluştur; sonra bağımsız geliştirmeye devam et. Sırf rapor üretmek için durma.

Dosya/depo erişimi yoksa kaynak kodu varmış veya işlemler yapılmış gibi iddia etme. Mümkün olan genel şablonları üret, erişim eksikliğini final raporunda belirt.

## 2. Rust uygulaması ve platform adaptörleri

### Mimari

Mevcut yapıya göre aşağıdaki sorumlulukları ayır:

- `app/ui`: EmuDesk arayüzü ve kullanıcı etkileşimi.
- `rdp/backend`: ortak bağlantı arayüzü ve bağlantı profilleri.
- `rdp/windows`: Windows üzerinde `mstsc.exe` ve desteklenen Windows RDP entegrasyonları.
- `rdp/linux`: FreeRDP istemci tespiti/çalıştırılması; `xfreerdp3` ve `xfreerdp` adlarını sırayla araştır, komut satırı seçeneklerini kurulu sürümle doğrula.
- `device_id`: kalıcı cihaz ID'si ve yerel kayıt.
- `github`: herkese açık repo/release/asset/update sorguları.
- `coordinator`: varsa ID kayıt/eşleştirme API'si; masaüstü istemcisinden ayrı dağıtım.

Bu isimleri mevcut proje mimarisiyle uyumlu olacak şekilde düzenle; gereksiz crate/workspace bölünmesi oluşturma.

### Arayüz

- Öncelik mevcut UI framework'ünü korumaktır; yeni proje ise `egui/eframe` değerlendirilebilir.
- Modern, sade, özgün EmuDesk tasarımı; koyu lacivert/siyah zemin, mavi/camgöbeği vurgular, temiz kartlar ve erişilebilir kontrast.
- Ana sayfa, IP/hostname/EmuDesk ID giriş alanı, bağlan düğmesi, kayıtlı cihazlar, bağlantı geçmişi, ayarlar, RDP yardım rehberi ve sürüm güncellemesi alanı.
- Bağlantı profilleri; ekran çözünürlüğü, pencere/tam ekran ve port seçenekleri. Yalnızca gerçek motorun desteklediği seçenekleri etkin göster.
- Kullanıcıya uzaktaki hedefin adresini/bağlantı durumunu göster; gizli bağlantı veya kullanıcıdan habersiz kontrol yapma.

### RDP bağlantı güvenliği

- Windows'ta Windows'un güvenilir RDP istemcisini kullan; `mstsc.exe` dosyasını yamalama, değiştirme veya Microsoft ürünüymüş gibi dağıtma.
- Linux'ta FreeRDP sistemde yoksa anlaşılır kurulum yönergesi göster; sessizce uzaktan binary indirme/çalıştırma yapma.
- Komut çalıştırırken shell string birleştirme yapma; `std::process::Command` ile ayrı argümanlar kullan.
- Parolayı command-line argümanlarına, loglara veya düz metin `.rdp` dosyasına koyma. Güvenli kimlik bilgisi akışını kullan; mümkün değilse parola saklama yerine kullanıcının ilgili RDP istemcisinde girmesini iste.
- Sertifika/güvenlik uyarılarını gizleme veya otomatik geçme; güvenlik seviyesini düşüren parametreleri varsayılan yapma.
- Dış programın yolu, exit code'u, hata mesajı ve sürüm tespiti test edilsin. Kullanıcı girişi shell komutu olarak yorumlanmasın.

## 3. Kalıcı cihaz ID'si ve yerel mod

- Kalıcı `device_id` ile geçici `session_id` ve süreli `pairing_code` kavramlarını ayrı tut.
- ID, kimlik doğrulama veya erişim yetkisi değildir.
- ID'yi işletim sistemine uygun kullanıcı yapılandırma alanında sakla; izinleri sıkı tut.
- Cihaz yeniden başlatıldığında ID aynı kalmalı; sıfırlama/yeniden kaydetme akışı açıkça tasarlanmalı.
- Bulut koordinatörü olmadan doğrudan IP/hostname üzerinden RDP bağlantısı çalışmalı.
- Aynı LAN'da hedefe erişmek için ID çözümleme tek başına yeterli sayılmamalı; gerçek DNS/IP, port, firewall ve hedefte RDP etkinliği doğrulanmalı.
- Koordinatör API'si ID çözümleme ve cihaz durumunu sağlayabilir; parola veya uzak ekran görüntüsü depolamamalı.
- NAT arkasında internet bağlantısı gerekiyorsa bunu ayrı, tehdit modeli ve maliyet değerlendirmesi olan bir aşama olarak ele al.

## 4. Windows ve Linux derleme hedefleri

En az şu hedefleri sürekli derle:

- Windows x64: `x86_64-pc-windows-msvc` (GitHub Windows runner; gerekli MSVC toolchain).
- Linux x64: `x86_64-unknown-linux-gnu` (GitHub Ubuntu runner).

Eğer mevcut kod/bağımlılıklar destekliyorsa ilave mimarileri ayrı aşama olarak değerlendir; ilk sürümde temel iki hedefi kararlı yapmadan ARM/32-bit ekleme.

Her platformda:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` (mevcut workspace/feature yapısına uyarla; gerçek gerekçe olmadan uyarıları topluca bastırma)
- `cargo test --workspace --all-features`
- `cargo build --release`
- Hedefli bağımlılık/özellik testi ve mümkünse smoke test.

Linux GUI bağımlılıklarını mevcut framework ve lockfile'a göre belirle; runner üzerinde gereken OS paketlerini workflow'da kur. Kütüphane isimlerini varsaymak yerine temiz bir GitHub Ubuntu runner'ında gerçekten dene.

Windows'ta gerçek GUI/RDP uçtan uca testi normal headless CI'da mümkün değilse build/test ile UI/RDP manuel testini ayır ve raporla. Çalıştırılmayan testi geçmiş gibi işaretleme.

## 5. Paketleme ve yayın dosyaları

Her sürüm için mümkün olan minimum güvenilir paketler:

- Windows: `EmuDesk-vX.Y.Z-windows-x86_64.zip` — EXE, LICENSE, README/kurulum notları. Mevcut paketleme sistemi uygunsa ayrıca imzasız bir installer üret; imzalıymış gibi gösterme.
- Linux: `EmuDesk-vX.Y.Z-linux-x86_64.tar.gz` — çalıştırılabilir dosya, LICENSE, README ve bağımlılık açıklaması.
- İsteğe bağlı `.deb` yalnızca paket kurulum/bağımlılık ve testleri gerçekten tamamlandığında yayınlansın.
- Tüm sürüm asset'leri için `SHA256SUMS.txt` oluştur ve doğrula.
- Dosya adları, sürüm numarası ve manifest birbiriyle uyuşmalı.
- Her paketin içine hassas token veya gerçek `.env` girmediğini kontrol et.
- Üçüncü taraf RDP motorlarını uygulama içine paketlemeden önce lisans, dağıtım şartları, güvenlik güncellemeleri ve bağımlılık listesini araştır. İlk sürümde Linux'ta sistem FreeRDP'sini kullanmak tercih edilir.

## 6. GitHub Actions: sürekli entegrasyon

`.github/workflows/ci.yml` veya mevcut yapıya uygun bir iş akışı oluştur/güncelle.

Tetikleyiciler:

- `pull_request`
- Ana branch'e `push`
- Manuel `workflow_dispatch` gerekiyorsa

İşler:

1. Linux ve Windows üzerinde matrix build/test.
2. Rust stable toolchain; repo bir `rust-toolchain.toml` sabitlemişse ona uy.
3. Rust derleme önbelleği.
4. Format, Clippy, test ve release build.
5. Linux OS bağımlılıklarını kur.
6. Release harici CI'da nihai GitHub Release yayımlama yapma.
7. Hata olduğunda log ve exit code'u koru; `continue-on-error` ile kritik adımları saklama.

Güvenlik:

- Workflow ve job bazında `permissions` değerlerini minimumda tut.
- CI testleri için çoğunlukla `contents: read` yeterlidir.
- `pull_request_target` kullanıp güvenilmeyen PR kodunu yazma yetkili token ile çalıştırma.
- Üçüncü taraf GitHub Actions'ı mümkün olduğunda tam commit SHA ile sabitle; yanına kullanılan upstream sürümünü yorum olarak yaz. Değişiklikleri resmi kaynakları kontrol ederek güncelle.
- Artifact'leri başka workflow'dan indirirken workflow/run/commit eşleşmesini doğrula.

## 7. Otomatik sürüm yönetimi

Semantic Versioning kullan: `MAJOR.MINOR.PATCH`.

- `fix:` → patch.
- `feat:` → minor.
- `BREAKING CHANGE:` → major.
- Ön sürümler `vX.Y.Z-alpha.N`, `vX.Y.Z-beta.N` veya `vX.Y.Z-rc.N` biçiminde etiketlenebilir.

`Cargo.toml` sürümü kaynak sürüm olsun. Workspace varsa workspace sürümlerini ve path bağımlılıklarını tutarlı yönet. Ayrı bir sabit sürüm numarasını kaynak kodda ikinci kez tutma; gerekiyorsa build-time metadata kullan.

Önerilen otomasyon `Release Please` (Rust stratejisi) + Conventional Commits'tir:

1. Ana branch'e merge edilen conventional commit mesajlarından otomatik release PR'si aç/güncelle.
2. Release PR'si Cargo sürümünü, `CHANGELOG.md` ve gerekiyorsa manifest/installer sürümünü güncellesin.
3. Release PR'si merge edilince `vX.Y.Z` tag'i ve release notları oluşturulsun.
4. Release tag'i Windows ve Linux release build'lerini başlatsın.
5. Tüm zorunlu build ve paketleme işleri başarılı olmadan sürüm nihai olarak yayınlanmasın. Mümkünse draft release oluştur, tüm asset/checksum'lar yüklendikten sonra yayınla.
6. Tekrar çalışan iş akışı aynı tag için ikinci sürüm oluşturmamalı; var olan release'i güvenli/tekrarlanabilir şekilde güncellemeli veya çakışmada açık hata vermeli.
7. Tag ile `Cargo.toml` sürümü eşleşmiyorsa release durmalı.

GitHub'ın `GITHUB_TOKEN` ile tetiklenen olaylar için workflow kısıtlarını dikkate al. Tag push'undan ayrı workflow tetiklenmesi gerekiyorsa bunu gerçek repoda test et; gereken durumda yalnızca ilgili repo erişimleri olan GitHub App installation token'ı tercih et. Fine-grained token seçilecekse sadece gereken depoda minimum izinleri ver. Classic PAT'i varsayılan çözüm yapma. Gerekli secret adlarını ve kurulum adımlarını `docs/github-release-setup.md` içinde belgele; secret değerlerini asla istemeden, loglamadan veya repoya yazmadan ilerle.

`release-please` entegrasyonu mevcut repo geçmişine göre doğru bootstrap edilmeli. Önceki tag yoksa ilk kurulum davranışını açıkça belgele. Eski release geçmişi veya kullanıcı etiketleri üzerine otomatik yıkıcı işlem yapma.

## 8. GitHub Release CI/CD

`.github/workflows/release.yml` veya benzeri bir iş akışı:

- `v*` SemVer tag'inde çalışmalı.
- `workflow_dispatch` ile test/dry-run modu olabilir; varsayılan olarak gerçek public release yayımlamasın.
- Windows ve Linux asset'lerini paralel olarak üret.
- Her asset'i izole runner'da oluştur ve upload öncesi dosya varlığı/boyutu/sürümünü doğrula.
- `SHA256SUMS.txt` üret; her asset hash'ini tekrar hesaplayıp eşleştiğini test et.
- Release notları `CHANGELOG.md`/GitHub generated notes ile üretilsin; kullanıcıya boş veya yanıltıcı release notu yayımlama.
- `contents: write` iznini sadece release işi için ver. GitHub dokümanlarını ve action'ın mevcut sürümünü doğrula.
- Release API/asset upload hatası olursa hata kodunu ve GitHub request id'yi hassas veri içermeden raporla.
- Başarısız build'de yarım release'i published yapma. Draft release veya kontrollü cleanup yaklaşımı kullan.
- Her başarılı stable release'te doğru `latest` durumunu ayarla; prerelease `latest` olmamalı.
- Release işleminde repository owner/name'i tahmin etme; `GITHUB_REPOSITORY`/context'ten al.

Release için GitHub REST API kullanılacaksa `Accept: application/vnd.github+json`, güncel `X-GitHub-Api-Version` ve minimum token izinleri kullan. Tag/release/asset upload API uçlarını resmi dokümantasyondan doğrula. Token değerini URL'ye, loglara veya build artifact'lerine koyma.

## 9. GitHub API entegrasyonu: uygulama içi

EmuDesk'in **herkese açık sürüm bilgisi okuma** işlevleri için GitHub REST API entegrasyonu ekle:

- Yapılandırılabilir repo owner/name.
- En son stable release'i sorgulama.
- `prerelease`/draft sürümlerini normal stable kanalında yok sayma.
- Sürüm notlarını ve yayın tarihini gösterme.
- Platform/architecture için uygun release asset'ini bulma.
- `browser_download_url` bağlantısını kullanıcıya açma veya indirme başlatma.
- Güncelleme yoksa net durum gösterme.
- API hatası, internet olmaması ve rate limit için anlaşılır mesaj; cache, ETag/conditional request, backoff ve makul timeout.
- JSON alanlarını doğrula; eksik/yanlış alanları varsayılan güvenli şekilde işle.
- Asset adı beklenen platform/architecture pattern'i ile eşleşmiyorsa yanlış dosyayı indirme.
- İndirme sonrası SHA-256 checksum'ını `SHA256SUMS.txt` ile doğrula.
- Güncellemeyi kullanıcının onayıyla başlat; imza doğrulaması yoksa bunu açıkça belirt. API yanıtından gelen komutları çalıştırma veya otomatik yönetici yetkisi isteme.

**Kesin gizlilik sınırı:** Masaüstü uygulamasına asla `GITHUB_TOKEN`, PAT, GitHub App private key veya release yayınlama sırrı gömme. Public repo'nun herkese açık release bilgileri için kimlik doğrulama gerekmediğinde anonim istek kullan. Private repo gerekiyorsa token'ı uygulamaya koymak yerine ayrı, güvenli sunucu/proxy veya kullanıcının açıkça yetkilendirdiği OAuth akışı tasarla.

İşlem/release oluşturma yetkisi yalnızca GitHub Actions ya da ayrı güvenilir yönetim ortamında bulunmalı; masaüstü istemcisi public release okuma ve güncelleme kontrolü ile sınırlı kalmalı.

## 10. Sürüm ve API entegrasyonu testleri

Şu durumları test et:

- API normal yanıtı; draft/prerelease/stable ayrımı.
- Yeni sürüm, aynı sürüm ve uzak sürüm daha eski olduğunda davranış.
- HTTP 403/404/429, 5xx, timeout, bozuk JSON, eksik asset, yanlış hash ve ağ kesintisi.
- Release asset'leri yalnızca o release'in platformuna göre seçilir.
- API token'ı/secret hiçbir logda ve binary içinde görünmez.
- Tag/Cargo version mismatch release'i engeller.
- Release workflow yinelendiğinde duplicate tag/release oluşturmaz.
- Build matrix'lerinden biri hata verince eksik asset içeren stable release yayınlanmaz.
- Linux GUI binary temiz desteklenen Linux ortamında başlatılabiliyor mu? Test edilemiyorsa blocker olarak kaydet.
- Windows binary ve RDP adaptörü Windows runner'ında doğrulanabiliyor mu? Gerçek bağlantı testi yapılamadıysa bunu ayırarak raporla.

## 11. Dokümantasyon ve repo dosyaları

Gerekiyorsa oluştur/güncelle:

- `README.md` — özellikler, platform durumu, ekran görüntüsü mevcutsa, kurulum ve indirmeler.
- `CHANGELOG.md` — otomatik sürüm notları.
- `CONTRIBUTING.md` — branch/PR/Conventional Commit akışı.
- `SECURITY.md` — güvenlik açığı bildirme.
- `docs/architecture.md` — UI, RDP backend, ID ve koordinasyon ayrımı.
- `docs/platform-support.md` — Windows MSTSC ve Linux FreeRDP gereksinimleri.
- `docs/github-actions.md` — CI workflow'ları.
- `docs/github-release-setup.md` — Release Please, GitHub App/token izinleri, Actions permissions, tag/release akışı.
- `docs/github-api.md` — REST API kullanımı, public repo varsayımı, rate limit/cache ve gizli bilgi sınırları.
- `.env.example` — yalnızca örnek/değersiz placeholder; gerçek secret içermez.
- `.gitignore` — build dosyaları, credentials, gerçek `.env`, yerel config ve paketleme çıktıları.
- Lisans ve üçüncü taraf bildirimleri, gerçekten kullanılan bağımlılıklara uygun olmalı.

README'de desteklenmeyen özellikleri varmış gibi listeleme. GitHub Release indirme bağlantılarını ve badge'leri gerçek repo adresi doğrulandıktan sonra ekle.

## 12. GitHub üzerinde işlem sınırları

GitHub entegrasyon araçlarına veya git push yetkisine erişimin varsa:

- Önce doğru repo sahibi/adını ve aktif branch'i doğrula.
- Yeni branch aç; kullanıcı değişikliklerini ezme.
- Değişiklikleri küçük, incelemeye uygun commit'lere böl; Conventional Commit kullan.
- PR oluşturabiliyorsan özet, test sonuçları, riskler ve kalan işler ile aç.
- Kullanıcı açıkça istemedikçe mevcut tag'i silme/değiştirme, history rewrite, force-push, mevcut release silme veya public stable release yayımlama.
- Otomasyon yapılandırmasını ve release dry-run/testini doğrulamadan gerçek sürüm yayınlama.
- Erişimin yoksa repository'ye commit/push yapmış gibi iddia etme; oluşturulan dosyaları ve gerekli GitHub ayarlarını teslim et.

## 13. Son kabul kriterleri

- [ ] Windows x64 build başarılı.
- [ ] Linux x64 build başarılı.
- [ ] Her platformun RDP backend'i ayrı ve doğru algılanıyor.
- [ ] Direct IP/hostname ile yerel bağlantı, ID sunucusundan bağımsız çalışıyor.
- [ ] Format, Clippy ve testler geçiyor veya bilinen blocker'lar açıkça raporlanıyor.
- [ ] CI, PR ve ana branch push'larında çalışıyor.
- [ ] SemVer ve changelog otomasyonu yapılandırıldı.
- [ ] Tag tabanlı release workflow Windows/Linux paketlerini ve checksum'ları üretiyor.
- [ ] Release API token izinleri minimum ve secret'lar log/binary/repo dışında.
- [ ] Uygulama GitHub API üzerinden stable sürüm ve platform asset'lerini okuyabiliyor.
- [ ] Release işlemi duplicate-safe ve başarısız matrix build stable yayını engelliyor.
- [ ] Kurulum, yapılandırma, token izinleri ve release adımları belgeli.
- [ ] Gerçekten çalıştırılan testlerle yalnızca önerilen/çalıştırılamayan testler birbirinden ayrılmış.

## Uygulama sırası

1. Depo denetimi ve başlangıç testleri.
2. Windows/Linux derleme engellerini giderme ve platform adaptörleri.
3. Özgün arayüz ve doğrudan RDP bağlantısı.
4. Kalıcı ID/yerel mod; koordinasyon API'sini ayrı tutma.
5. GitHub API ile release listesi/güncelleme kontrolü.
6. Çoklu platform CI.
7. SemVer + Release Please + tag tabanlı release build/paketleme.
8. Checksum, güvenlik testleri, dokümantasyon ve dry-run.
9. Son rapor ve PR hazırlığı.

**Son rapor formatı:** İncelenen dosyalar; değişiklikler; Windows/Linux build sonucu; test sonuçları; eklenen workflow'lar; SemVer/Release Please ayarı; GitHub API özellikleri; gereken repository secrets/permissions; gerçek GitHub repo işlemlerinin yapılıp yapılmadığı; kalan blocker'lar. Her iddiayı gerçek komut çıktısı veya işlem sonucu ile destekle.
