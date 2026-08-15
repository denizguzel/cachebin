# ClearDisk Arayüz Görseli: Ayrıntılı Tasarım Betimlemesi

## Genel Kompozisyon

Görsel, siyaha çok yakın koyu bir arka plan üzerinde yatay olarak sıralanmış altı adet macOS tarzı masaüstü uygulama ekranını gösterir. Ekranlar aynı ürünün farklı sekme ve akışlarını temsil eder. Her ekranın altında ortalanmış beyaz bir başlık ve daha küçük gri açıklama metni vardır.

Görsel boyutu yaklaşık `2000 x 580` pikseldir. Altı ekran soldan sağa şu sıradadır:

1. `Caches`
2. `Projects`
3. `Clean Projects`
4. `Clean Caches`
5. `Large Files`
6. `Overview`

Ekran kartları yaklaşık 320 piksel genişliğinde ve 460 piksel yüksekliğindedir. Köşeleri yaklaşık 18-20 piksel yuvarlatılmıştır. Uygulama panelleri koyu kömür grisi/siyah renktedir; arka plan panelden biraz daha siyahtır. Metinler çoğunlukla beyaz veya açık gridir. Vurgu renkleri mavi, yeşil, turuncu ve kırmızıdır.

## Ortak Uygulama Üst Bölümü

Her ana ClearDisk ekranında:

- Sol üstte kalın beyaz `ClearDisk` logosu bulunur.
- Sağ üstte dairesel ok/yenileme simgesi vardır.
- Bazı ekranlarda yenileme simgesinin solunda veri toplanmasını anlatan gri bir yükleniyor simgesi bulunur.
- Hemen altında, yaklaşık 8 piksel yuvarlatılmış mavi bir disk kullanım çubuğu vardır. Çubuğun solunda veya içinde `60%` yazısı görünür.
- Çubuğun altında solda `137.7 GB used`, sağda `90.5 GB free` metinleri yer alır.
- Sonraki satırda gri bir bilgi noktası ve `Forecast: collecting data... (8 snapshots)` metni bulunur.
- Bu satırın altında ince yatay ayırıcı çizgi vardır.
- Uyarı alanı kahverengi/koyu amber renkte, yaklaşık 34 piksel yüksekliğindedir. Solunda kilit simgesi ve şu metin vardır: `7 paths not readable – sizes may be incomplete`.

## 1. Caches Ekranı

İlk kartta aktif sekme `Developer` olduğundan, alt içerik geliştirici önbelleklerini gösterir.

### Özet Alanı

- Uyarı alanının altında büyük beyaz `18.9 GB` metni bulunur.
- Altında küçük gri `reclaimable space found` metni vardır.
- Yeşil ve camgöbeği renkli yatay dağılım çubuğu görünür. Yeşil bölüm yaklaşık yüzde 75, camgöbeği bölüm yaklaşık yüzde 25 genişliğindedir.
- Çubuğun altında yeşil nokta ile `14.3 GB safe`, camgöbeği nokta ile `4.6 GB projects` yazıları bulunur.
- Yan yana iki buton vardır:
  - Yeşil buton: `Clean Caches`, solunda parıltı/temizlik simgesi.
  - Koyu camgöbeği buton: `Clean Projects`, solunda klasör benzeri simge.

### Sekmeler ve Liste

- Dört sekme bulunur: `Developer`, `Projects`, `Overview`, `Large Files`.
- `Developer` sekmesi mavi arka planlı ve mavi metinlidir; diğerleri gri metinlidir.
- Listede her satırda solda kaynak türünü gösteren renkli simge, ad, açıklama, yaş bilgisi, boyut ve sağda çöp kutusu ile klasör simgesi bulunur.
- Görünen satırlar:
  - `Android Emulators`, açıklama `~/.android/avd`, yaş `63d ago`, boyut `7.0 GB`; sarı uyarı ve `Unused for 63 days – consider cleaning` notu.
  - `Gradle Cache`, açıklama `~/.gradle/caches`, yaş `3d ago`, boyut `4.5 GB`.
  - `npm Cache`, açıklama `Cached package tarballs from npmjs.org. Re-downloads on n...`, yaş `48d ago`, boyut `1.6 GB`.
- Kartın en altında `Total saved: 118 MB` yeşil metni ve sağda küçük gri `Quit` butonu bulunur.

Kartın altındaki görsel açıklama:

- Başlık: `Caches`
- Açıklama: `28 cache paths with` ve ikinci satırda `risk levels & sizes`

## 2. Projects Ekranı

İkinci kart, aynı üst özeti kullanır ancak `Projects` sekmesi aktiftir.

- Sekme durumları: `Projects` mavi vurgulu; `Developer`, `Overview` ve `Large Files` gri.
- Liste proje ve build artifact kayıtlarını gösterir.
- İlk satırlar:
  - `rag-odev`, mor `node_modules` etiketi, teknoloji `Node.js`, yaş `35d`, boyut `165 MB`; altında `Stale – not modified for 35 days` uyarısı.
  - `mind-court-web`, mor `node_modules` etiketi, yol `~/Documents/opsus--sium/mind-court-web`, yaş `7d`, boyut `144 MB`.
  - `old-tect`, mor `node_modules` etiketi, yol `~/Documents/opsus-prizely-old/old-ext`, yaş `8d`, boyut `127 MB`.
  - Alt kısımda kısmen görünen `vscode` satırı vardır.
- Her satırın sağında kırmızı çöp kutusu ve mavi klasör simgesi bulunur.
- Alt durum çubuğu yine `Total saved: 118 MB` ve `Quit` içerir.

Kartın altındaki görsel açıklama:

- Başlık: `Projects`
- Açıklama: `Stale project artifacts` ve ikinci satırda `& build folders`

## 3. Clean Projects Ekranı

Üçüncü kart, proje artifact'larını seçip silmeye yönelik ayrı bir seçim ekranıdır.

### Üst Bölüm

- Sol üstte mavi sol ok ve `Back` metni vardır.
- Ortada beyaz `Clean Projects` başlığı bulunur.
- Sağ üstte mavi `Select All` metni vardır.
- Altında iki filtre sekmesi bulunur: `All` ve `Stale (>30d)`. `All` mavi seçili arka planlıdır.

### Proje Listesi

Her satırda solda dairesel seçim kutusu, mor veya turuncu proje simgesi, proje adı, renkli kategori etiketi, teknoloji metni ve sağda boyut bulunur. Seçilen satırların dairesi mavi dolgu ve beyaz onay işareti içerir.

Görünen kayıtlar:

- `windowkey`, `build`, `Swift PM`, `895 MB`, seçili değil.
- `the-saas-stack`, `node_modules`, `Node.js`, `556 MB`, seçili değil.
- `WindowKey`, `build`, `Swift PM`, `492 MB`, seçili.
- `ClearDisk`, `build`, `Swift PM`, `400 MB`, seçili değil.
- `app`, `build`, `Gradle (Kotlin)`, `368 MB`, seçili.
- `copy-of-immersive-gallery-os---stable-2`, `node_modules`, `Node.js`, `350 MB`, seçili değil.
- `app`, `build`, `Gradle (Kotlin)`, `347 MB`, seçili değil.
- `frontend`, `node_modules`, `Node.js`, `209 MB`, seçili.
- `app`, `build`, `Gradle (Kotlin)`, `177 MB`, seçili değil.
- `rag-odev`, `node_modules`, `Node.js`, `165 MB`, seçili.
- `mind-court-web`, `node_modules`, `Node.js`, `144 MB`, seçili.

Liste altındaki koyu şeritte `6 selected · 1.5 GB` yazar. En altta geniş turuncu buton bulunur: çöp kutusu simgesi ve `Remove Selected (1.5 GB)`.

Kartın altındaki görsel açıklama:

- Başlık: `Clean Projects`
- Açıklama: `Select & remove` ve ikinci satırda `project build artifacts`

## 4. Clean Caches Ekranı

Dördüncü kart, güvenli veya riskli önbellekleri temizleme ekranıdır.

- Üst çubukta sol üstte mavi sol ok ve `Back`, ortada `Clean Caches` başlığı vardır.
- İki segmentli filtre bulunur:
  - Sol segment koyu yeşil seçili durumdadır ve parlak yeşil `Safe Only` metni içerir.
  - Sağ segment `All (Including Risky)` metnini gri gösterir.
- İçerik üstünde solda `5 caches`, sağda yeşil `14.3 GB` bulunur.
- Beş önbellek satırı vardır. Her satırda renkli durum noktası, kalın ad, açıklama ve sağda boyut bulunur:
  - `Android Emulators`, `Android Virtual Devices and disk images. Must re-create in A...`, `7.0 GB`; sarı nokta.
  - `Gradle Cache`, `Downloaded JARs, build outputs, and wrapper dists. Re-do...`, `4.5 GB`; yeşil nokta.
  - `npm Cache`, `Cached package tarballs from npmjs.org. Re-downloads on n...`, `1.6 GB`; yeşil nokta.
  - `Homebrew Cache`, `Downloaded formula bottles and taps. Re-downloads on brew...`, `933 MB`; yeşil nokta.
  - `pip Cache`, `Downloaded Python wheels and sdists. Re-downloads on pip...`, `420 MB`; yeşil nokta.
- Altında geniş yeşil buton bulunur: çöp kutusu simgesi ve `Clean Safe Caches (14.3 GB)`.

Kartın altındaki görsel açıklama:

- Başlık: `Clean Caches`
- Açıklama: `Safe or risky` ve ikinci satırda `cache cleaning`

## 5. Large Files Ekranı

Beşinci kart, üst özet alanı ve sekmeleri korur; `Large Files` sekmesi mavi vurguludur.

- Büyük dosya listesinde her satırda turuncu disk/drive simgesi, dosya adı, dosya yolu, boyut ve sağda mavi klasör simgesi bulunur.
- Görünen dosyalar:
  - `FreeCAD_1.0.2-con...S-arm64-py311.dmg`, yol `~/Downloads/FreeCAD...cOS-arm64-py311.dmg`, `634 MB`.
  - `Beekeeper-Studio-5.5.3-arm64.dmg`, yol `~/Downloads/Beekeeper...tudio-5.5.3-arm64.dmg`, `234 MB`.
  - `VSCode-darwin-universal.dmg`, yol `~/Downloads/VSCode...universal.dmg`, `233 MB`.
  - `googlechrome.dmg`, yol `~/Downloads/googlechrome.dmg`, `232 MB`.
  - `Bambu_Studio_ma...251118194119.dmg`, boyut `201 MB`.
- Alt durum çubuğunda `Total saved: 118 MB` ve `Quit` bulunur.

Kartın altındaki görsel açıklama:

- Başlık: `Large Files`
- Açıklama: `Find biggest files` ve ikinci satırda `across your disk`

## 6. Overview Ekranı

Altıncı kart, aynı ClearDisk başlığı ve disk kullanım özetiyle başlar; `Overview` sekmesi aktiftir.

- Sekme çubuğunda `Overview` mavi arka plan ve mavi metinle seçilidir.
- İçerik, disk kategorilerini yatay çubuklarla gösterir. Her satırda solda kategoriye ait renkli kare/ikon, ortada kategori adı ve çubuk, sağda boyut bulunur:
  - `Applications`, mavi ikon, mavi çubuk, `10.6 GB`.
  - `Documents`, camgöbeği ikon, camgöbeği çubuk, `7.7 GB`.
  - `Caches`, kırmızı ikon, kırmızı çubuk, `6.0 GB`.
  - `Downloads`, yeşil ikon, yeşil çubuk, `4.5 GB`.
  - `Desktop`, mor ikon, mor çubuk, `3.9 GB`.
  - `Photos`, camgöbeği ikon, çok kısa camgöbeği çubuk, `12 MB`.
- Alt durum çubuğunda `Total saved: 118 MB` ve `Quit` bulunur.

Kartın altındaki görsel açıklama:

- Başlık: `Overview`
- Açıklama: `Disk usage dashboard` ve ikinci satırda `& storage forecast`

## Renk, Tipografi ve Stil Kuralları

- Ana arka plan: neredeyse siyah.
- Panel: koyu gri/kömür tonları; kartlar hafif transparan veya çok koyu yüzey hissi verir.
- Ana metin: beyaz veya açık gri.
- İkincil metin: orta gri.
- Mavi: disk kullanım çubuğu, seçili sekmeler, klasör ikonları ve bağlantılar.
- Yeşil: güvenli temizleme, güvenli alan, başarı bilgileri ve kaydedilen alan.
- Turuncu: seçili proje artifact'larını silme ve uyarı/yaş vurguları.
- Kırmızı: silme ikonları ve cache kategorisi.
- Yazı tipi modern bir sistem sans-serifidir; başlıklar orta/kalın, açıklamalar küçük ve gri, sayısal özetler büyük ve kalındır.
- Panel içeriği yoğun ama düzenlidir. Ayırıcı çizgiler ince, butonlar kısa yükseklikte ve köşeleri yaklaşık 7-8 piksel yuvarlatılmıştır.
- Tüm kartlar aynı genişlik, üst boşluk, özet düzeni ve alt durum çubuğu yapısını paylaşır. Bu ortak yapı ürünün tutarlı bir masaüstü disk temizleme uygulaması olarak algılanmasını sağlar.
