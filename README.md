# EmuDesk

EmuDesk, Windows ve Linux üzerinde çalışan, açık kaynaklı ve koyu temalı bir uzak masaüstü (RDP) istemcisi/başlatıcısıdır. Kendi RDP görüntü akışını içermez; bağlantıları Windows'ta Microsoft'un yerleşik `mstsc.exe` istemcisiyle, Linux'ta FreeRDP (`xfreerdp3`/`xfreerdp`) motoruyla kurar.

EmuDesk, bağlantı profillerinizi (IP/hostname/ID, port, çözünürlük, tam ekran/pencere modu, çoklu monitör, kullanıcı adı) tek bir koyu arayüzde yönetmenize, kalıcı cihaz kimliğinizi taşımanıza ve ihtiyaç duyduğunuzda RDP bağlantısını tek tıkla başlatmanıza yardımcı olur.

## Özellikler

Yalnızca şu anda gerçekten uygulanan özellikler listelenmiştir:

- **Bağlantı profilleri** — IP/hostname/EmuDesk ID, port, çözünürlük, tam ekran/pencere modu, çoklu monitör ve kullanıcı adı alanlarıyla profilleri kaydedip yönetme.
- **RDP başlatıcı** — Profili, platforma uygun istemciye aktararak bağlantıyı başlatır:
  - Windows: `mstsc.exe` (Windows'un yerleşik RDP istemcisi).
  - Linux: FreeRDP `xfreerdp3` / `xfreerdp`.
- **Kalıcı cihaz kimliği** — Cihaza özgü, yeniden üretilebilir bir EmuDesk ID; randevu/koordinasyon adresidir, tek başına erişim hakkı değildir.
- **Koordinasyon sunucusu** — İsteğe bağlı `emudesk-server` ile cihaz kaydı, ID eşleştirme, online durum ve endpoint çözümleme; /health uç noktası ve kalıcı JSON kayıtları. Masaüstü kullanımı için zorunlu değildir.
- **Türkçe RDP yardım rehberi** — Uygulama içi Türkçe RDP kurulum/bağlantı rehberi.
- **GitHub güncelleme kontrolü** — Yayınlanan sürümleri kontrol eden modül.

> EmuDesk bir **launcher/yardımcı istemcidir**; RDP protokolünü kendisi uygulamaz ve görüntü/oturum akışını kendi üretmez. Görüntü akışı platformun yerleşik RDP istemcisi (Windows: `mstsc.exe`, Linux: FreeRDP) tarafından sağlanır.

## Desteklenen Platformlar

| Platform | RDP motoru | Durum |
|----------|-----------|-------|
| Windows | `mstsc.exe` (yerleşik) | Desteklenir |
| Linux | FreeRDP (`xfreerdp3` / `xfreerdp`) | Desteklenir |

## Gereksinimler

**Windows**

- Windows'un yerleşik RDP istemcisi (`mstsc.exe`). Genellikle hazır gelir.
- **Hedef makineye bağlanabilmek için hedef makinede Uzak Masaüstü (RDP) sunucusu etkin olmalıdır.** Windows Pro / Enterprise / Education sürümlerinde RDP sunucusu etkinleştirilebilir; Home sürümlerinde RDP sunucusu bulunmaz.

**Linux**

- FreeRDP istemcisi kurulu olmalıdır. Debian/Ubuntu tabanlı dağıtımlarda:

  ```bash
  sudo apt install freerdp3-x11
  ```

  Başka dağıtımlarda paket adı değişebilir (ör. Fedora: `freerdp`). EmuDesk `xfreerdp3` ve `xfreerdp` çalıştırılabilir dosyalarını arar.

## Yerel Kurulum

Proje bir Cargo (Rust) workspace'idir. Geliştirici ortamında masaüstü uygulamasını çalıştırmak için:

```bash
cargo run -p emudesk
```

Koordinasyon sunucusunu ayrıca çalıştırmak için:

```bash
cargo run -p emudesk-server
```

Sunucu, varsayılan olarak `PORT` ortam değişkeninden gelen portu (yoksa varsayılanını) kullanır.

## Koordinasyon Sunucusu (emudesk-server)

`emudesk-server`, HTTP + WebSocket tabanlı bir koordinasyon/randevu sunucusudur (axum). Şunları sağlar:

- Cihaz kaydı ve EmuDesk ID eşleştirme,
- Online durum takibi,
- Endpoint çözümleme,
- `/health` sağlık uç noktası,
- Kalıcı JSON kayıtları.

Masaüstü uygulaması bu sunucu olmadan da çalışır; sunucu yalnızca randevu/koordinasyon senaryoları içindir.

**Docker / Railway**

Bu workspace bir `Dockerfile`, `Procfile` ve `railway.json` içerir ve koordinasyon sunucusunu Docker/Railway'a dağıtılabilir şekilde hazırlar.

## EmuDesk ID Nedir?

EmuDesk ID, bir cihaza özgü olarak üretilen ve kalıcı olarak saklanan bir tanımlayıcıdır. Randevu/koordinasyon adresidir; **tek başına erişim hakkı değildir.** Uzak bir makineye bağlanmak için ayrıca geçerli kimlik bilgileri (kullanıcı adı/parola) ve erişilebilir bir ağ yolu gereklidir.

## Güvenlik Notları

- Parolalar asla komut satırına, loglara veya `.rdp` dosyalarına yazılmaz.
- `mstsc.exe` yamalanmaz veya yeniden dağıtılmaz; sistemin orijinal istemcisi kullanılır.
- EmuDesk ID tek başına erişim hakkı değildir; randevu adresidir.
- Masaüstüne GitHub token gömülmez.
- RDP 3389 portu gelişigüzel internete açılmaz; güvenli ağ/erişim kurallarına uyulmalıdır.

## Henüz Uygulanmayan / Doğrulanamayan

Aşağıdaki özellikler şu anda **uygulanmamıştır** ve bu sürümde "varmış" gibi sunulmaz:

- **İnternet üzerinden RDP relay / NAT geçişi** — EmuDesk ID'nin internet üzerinden uçtan uca relay olarak kullanılması henüz uygulanmadı/doğrulanmadı.
- **VPN / Gateway entegrasyonu** — Ayrı bir geliştirme aşaması olarak planlanmıştır; bu sürümde yoktur.

## Sürümler ve İndirme

Henüz public GitHub release bulunmamaktadır; sürüm 0.1.0'dır. Release'ler yayınlandığında indirme bağlantıları buraya eklenecektir:

- Windows: `# ...`
- Linux: `# ...`

## Lisans

MIT lisansı altında yayımlanır. Detaylar için [LICENSE](LICENSE) dosyasına bakın. Copyright (c) 2026 EmuDesk contributors.