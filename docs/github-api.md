# GitHub Güncelleme Kontrolü

Bu belge, EmuDesk'in güncelleme kontrolü modülünün GitHub REST API'yi nasıl kullandığını açıklar.

## Genel Bakış

Güncelleme kontrolü, GitHub üzerinde **public** bir depodan yayınlanan release'leri okuyarak mevcut sürümle karşılaştırır. Public repo varsayılır; kimlik doğrulama (token) gerekmez.

## Releases / Latest

En son yayınlanan release'yi okumak için GitHub REST API'nin `releases/latest` uç noktası kullanılır:

```
GET /repos/{owner}/{repo}/releases/latest
```

Yanıt, sürüm etiketi (`tag_name`), sürüm adı ve bağlı **asset**'lerin listesini içerir.

## Asset Seçimi

Bir release'ye bağlı indirilebilir asset'ler `assets` listesinde yer alır. EmuDesk, indirilecek uygun asset'i ad desenine göre seçer:

- **Windows:** adında `windows-x86_64` geçen asset.
- **Linux:** adında `linux-x86_64` geçen asset.

Seçim, asset adındaki bu platform desenlerine göre yapılır; uygun asset bulunamazsa ilgili platform için güncelleme sunulmaz.

## SHA-256 Doğrulama

İndirilen asset'in bütünlüğü **SHA-256** ile doğrulanır:

1. İndirilen dosyanın SHA-256 özeti hesaplanır.
2. Beklenen özet (release notlarında veya ayrı bir checksum dosyasında yayınlanan) ile karşılaştırılır.
3. Özetler eşleşmezse dosya güvenilmeyen olarak değerlendirilir ve kurulmaz.

## Rate Limit ve Cache

- GitHub REST API, kimlik doğrulamasız istekler için **rate limit** uygular.
- EmuDesk, bu limite takılmamak ve gereksiz istekten kaçınmak için sürüm kontrolü sonucunu yerel olarak **cache**'ler ve istekleri makul aralıklarla tekrarlar.
- Cache, kullanıcı yeniden kontrol etmek istediğinde geçersiz kılınabilir.

## Token Gizliliği

- Güncelleme kontrolü **public repo** okuması yaptığından kimlik doğrulama token'ı gerektirmez.
- Bu ilkeye uygun olarak masaüstüne **asla GitHub token gömülmez**. Herhangi bir senaryoda token'lar komut satırına, loglara veya sürüm/kurulum dosyalarına yazılmaz.

## Not

Henüz public GitHub release bulunmamaktadır; sürüm 0.1.0'dır. Release yayınlandığında indirme bağlantıları README'de yer alacaktır.