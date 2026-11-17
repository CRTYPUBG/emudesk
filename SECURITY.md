# Güvenlik Politikası

EmuDesk, güvenlik açıklarının sorumlu biçimde bildirilmesini ve özel olarak ele alınmasını ciddiye alır.

## Güvenlik Açığı Bildirimi

Bir güvenlik açığı bulduysanız, **public issue açmayın** ve bunu herkese açık platformlarda tartışmayın. Bunun yerine, açığı **özel (private) bildirim yolu** ile iletin.

Bildiriminizi hazırlarken lütfen şunları ekleyin:

- Etkilenen sürüm ve bileşen (masaüstü, koordinasyon sunucusu, `emudesk-core`).
- Açığı yeniden üretmenin adım adım açıklaması.
- İlgili log parçaları (gizli bilgileri/kimlik bilgilerini çıkardıktan sonra).
- Mümkünse önerilen düzeltme veya azaltma (mitigasyon).

## Sorumlu Açıklama (Responsible Disclosure)

- Bildiriminizi aldıktan sonra açığı doğrulamaya ve etkiyi değerlendirmeye çalışırız.
- Düzeltme hazırlanırken ve koordine edilirken açığı **gizli** tutmanızı rica ederiz.
- Uygun bir iyileştirme süresi sonrasında düzeltme ve güvenlik açıklaması yayınlanır.

## Güvenlik İlkeleri

Bu proje aşağıdaki ilkelere bağlıdır; katkılar ve bildirimler bu ilkelerle tutarlı olmalıdır:

- Parolalar asla komut satırına, loglara veya `.rdp` dosyalarına yazılmaz.
- `mstsc.exe` yamalanmaz veya yeniden dağıtılmaz; sistemin orijinal istemcisi kullanılır.
- EmuDesk ID tek başına erişim hakkı değildir; yalnızca randevu adresidir.
- Masaüstüne GitHub token gömülmez.
- RDP 3389 portu gelişigüzel internete açılmaz; güvenli erişim kurallarına uyulur.