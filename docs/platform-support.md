# Platform Desteği

Bu belge, EmuDesk'in RDP bağlantısı kurmak için kullandığı platform motorlarının gereksinimlerini ve kurulum adımlarını açıklar.

## Desteklenen Platformlar

| Platform | RDP motoru | Kurulum |
|----------|-----------|---------|
| Windows | `mstsc.exe` (yerleşik) | Genellikle hazırdır; RDP sunucusu etkinleştirmesi gerekir |
| Linux | FreeRDP (`xfreerdp3` / `xfreerdp`) | Paket yöneticisiyle kurulum gerekir |

---

## Windows — MSTSC

### İstemci

Windows, RDP istemcisi olarak yerleşik `mstsc.exe`'yi içerir. Genellikle ek kurulum gerekmez. EmuDesk, sistemin orijinal `mstsc.exe`'sini kullanır; yamalamaz veya yeniden dağıtmaz.

### RDP Sunucusunu Etkinleştirme

Uzak bir Windows makineye bağlanabilmek için **hedef makinede** Uzak Masaüstü (RDP) sunucusunun etkin olması gerekir:

1. **Ayarlar** → **Sistem** → **Uzak Masaüstü** yolunu izleyin.
2. **Uzak Masaüstü** anahtarını açın.
3. Gerekirse onay uyarısını kabul edin.

> **Not:** RDP sunucusu yalnızca Windows **Pro / Enterprise / Education** sürümlerinde etkinleştirilebilir. **Home** sürümlerinde RDP sunucusu bulunmaz.

Ayrıca uzak makineye ağ üzerinden erişilebilir olması gerekir; 3389 portu gerektiğinde güvenli biçimde açılmalı ve gelişigüzel internete açılmamalıdır.

---

## Linux — FreeRDP

### İstemci Kurulumu

EmuDesk, Linux'ta FreeRDP istemcisini kullanır. `xfreerdp3` veya `xfreerdp` çalıştırılabilir dosyalarından uygun olanı tespit eder ve kullanır.

**Debian / Ubuntu** tabanlı dağıtımlar:

```bash
sudo apt install freerdp3-x11
```

**Diğer dağıtımlar** (paket adı değişebilir):

- Fedora / RHEL tabanlı: `sudo dnf install freerdp`
- openSUSE: `sudo zypper install freerdp`
- Arch Linux: `sudo pacman -S freerdp`

Kurulumdan sonra FreeRDP'nin doğru kurulduğunu şu komutla doğrulayabilirsiniz:

```bash
xfreerdp3 /version
# veya
xfreerdp /version
```

### xfreerdp3 / xfreerdp Tespiti

EmuDesk, aşağıdaki sırayla uygun bir FreeRDP istemcisi arar:

1. `xfreerdp3`
2. `xfreerdp`

Bunlardan herhangi biri bulunursa bağlantı onunla başlatılır. Hiçbiri bulunamazsa, kullanıcıya FreeRDP kurulumu önerilir.

---

## Güvenlik Notları

- Parolalar asla komut satırına, loglara veya `.rdp` dosyalarına yazılmaz.
- RDP 3389 portu gelişigüzel internete açılmaz; güvenli erişim kurallarına uyulur.