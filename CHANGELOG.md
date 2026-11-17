# Changelog

Tüm önemli değişiklikler bu dosyada tutulur. Biçimlendirme [Keep a Changelog](https://keepachangelog.com/tr/1.0.0/) ve [SemVer](https://semver.org/) ilkelerini izler.

## [0.1.0] - İlk sürüm

İlk yayın. EmuDesk'in başlangıç özellik seti.

### Eklenen

- **Masaüstü GUI (`emudesk`)** — koyu temalı egui/eframe arayüzü.
  - Bağlantı profilleri: IP/hostname/EmuDesk ID, port, çözünürlük, tam ekran/pencere modu, çoklu monitör, kullanıcı adı.
  - Profil kaydetme, düzenleme ve başlatma.
- **RDP başlatıcı** — platforma uygun istemciye aktarım:
  - Windows: `mstsc.exe`.
  - Linux: FreeRDP (`xfreerdp3` / `xfreerdp`).
- **Kalıcı cihaz kimliği** — `emudesk-core` içinde DeviceIdentity / IdentityStore; cihaza özgü ve kalıcı EmuDesk ID.
- **Yapılandırma** — `emudesk-core` içinde kalıcı yapılandırma desteği.
- **Türkçe RDP yardım rehberi** — uygulama içi rehber.
- **GitHub güncelleme kontrolü** — yayınlanan sürümleri kontrol eden modül.
- **Koordinasyon sunucusu (`emudesk-server`)** — HTTP + WebSocket (axum): cihaz kaydı, ID eşleştirme, online durum, endpoint çözümleme; `/health` uç noktası, `PORT` ortam değişkeni, kalıcı JSON kayıtları; Docker/Railway dağıtım desteği.

### Güvenlik

- Parolalar komut satırına, loglara veya `.rdp` dosyalarına yazılmaz.
- `mstsc.exe` yamalanmaz veya yeniden dağıtılmaz.
- EmuDesk ID, tek başına erişim hakkı değildir; randevu adresidir.
- Masaüstüne GitHub token gömülmez.

### Bilinen Sınırlamalar

- İnternet üzerinden RDP relay / NAT geçişi henüz uygulanmamıştır.
- VPN / Gateway entegrasyonu ayrı bir geliştirme aşaması olarak planlanmıştır, bu sürümde yoktur.
- Henüz public GitHub release bulunmamaktadır.