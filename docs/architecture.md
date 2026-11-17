# Mimari

Bu belge EmuDesk'in üst düzey mimarisini ve modül sınırlarını açıklar. Amaç, katkıcıların hangi bileşenin nerede yaşadığını ve sorumlulukların nasıl ayrıldığını net biçimde görmesini sağlamaktır.

## Genel Bakış

EmuDesk bir Cargo (Rust) workspace'idir ve üç member'dan oluşur:

```
emudesk-core    ── kalıcı cihaz kimliği (DeviceIdentity, IdentityStore) + yapılandırma
emudesk-server  ── HTTP + WebSocket koordinasyon/randevu sunucusu (axum)
emudesk         ── masaüstü GUI (egui/eframe) ve RDP başlatıcı
```

Masaüstü uygulaması RDP protokolünü kendisi uygulamaz. Platformun yerleşik RDP istemcisine (Windows: `mstsc.exe`, Linux: FreeRDP) bağlantıyı aktaran bir **launcher/adaptör** görevi görür.

## Modül Haritası

### UI (masaüstü — `emudesk`)

- **app / egui** — eframe ile koyu temalı arayüz: profil listesi, profil düzenleme formu, bağlantı başlatma, yardım rehberi ve güncelleme kontrolü ekranları.
- Profiller kullanıcı tarafında saklanır ve yönetilir.

### RDP Backend (platform adaptörü — `emudesk`)

RDP başlatma, platforma göre ayrışan bir adaptör katmanı üzerinden yapılır:

- **Windows adaptörü** — bağlantıyı `mstsc.exe` (Microsoft'un yerleşik RDP istemcisi) ile başlatır. Sistemin orijinal `mstsc.exe`'si kullanılır; yamalanmaz veya yeniden dağıtılmaz.
- **Linux adaptörü** — bağlantıyı FreeRDP ile başlatır; `xfreerdp3` ve `xfreerdp` çalıştırılabilir dosyalarından uygun olanı seçer.

Parolalar asla komut satırına, loglara veya `.rdp` dosyalarına yazılmaz; kimlik bilgileri istemci tarafında güvenli biçimde yönetilir.

### device_id (`emudesk-core`)

- **DeviceIdentity / IdentityStore** — cihaza özgü, kalıcı bir EmuDesk ID üretir ve saklar.
- EmuDesk ID bir **randevu adresidir**, tek başına erişim hakkı değildir.

### coordinator (`emudesk-server`)

- HTTP + WebSocket tabanlı koordinasyon/randevu sunucusu (axum).
- Cihaz kaydı, EmuDesk ID eşleştirme, online durum takibi ve endpoint çözümleme.
- `/health` sağlık uç noktası, `PORT` ortam değişkeni ve kalıcı JSON kayıtları.
- Masaüstü kullanımı için **zorunlu değildir**; yalnızca randevu/koordinasyon senaryoları içindir. Docker/Railway'a dağıtılabilir.

### github (güncelleme kontrolü — `emudesk`)

- GitHub REST API üzerinden public release'leri okur (bkz. [github-api.md](github-api.md)).
- Masaüstüne GitHub token gömülmez.

## Veri Akışı

```
[Profil] ──► [RDP Backend adaptörü] ──► mstsc.exe / xfreerdp
[device_id] ──► (isteğe bağlı) ──► [emudesk-server] randevu/koordinasyon
[github] ──► GitHub REST API ──► sürüm kontrolü
```

## Sorumluluk Ayrımı

| Bileşen | Sorumluluk | Bağımlı olduğu |
|---------|-----------|----------------|
| UI (app/egui) | profil yönetimi, kullanıcı etkileşimi | backend modülleri |
| RDP backend | platforma uygun istemciyi başlatma | sistem RDP istemcisi |
| device_id | kalıcı cihaz kimliği | yapılandırma |
| coordinator | randevu/koordinasyon (isteğe bağlı) | HTTP/WebSocket |
| github | release/asset okuma | GitHub REST API |

## Henüz Uygulanmayanlar

- İnternet üzerinden RDP relay / NAT geçişi — henüz uygulanmadı/doğrulanmadı.
- VPN / Gateway entegrasyonu — ayrı bir geliştirme aşaması olarak planlanmıştır.