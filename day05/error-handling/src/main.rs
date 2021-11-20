use std::fs::File;
use std::io::ErrorKind;

fn main() {
    /*
        Hata Yönetimi - 1

        Kurtarılabilir hata durumlarında da programcı panic! makrosunu kullanarak işleyişi durdurabilir.
        Aşağıdaki örnekte sistemde olmaya reports.txt isimli dosya okumak amacıyla açılmak isteniyor.
    */
    //case_1_explicitly_panic_from_result();

    /*
        Hata Yönetimi - 2

        Result ile dönen hatalar türlerine göre kategorilere de ayrılabilirler.
        Bu türe bakarak pattern matching kısmında hataları daha spesifik olarak ele alabiliriz.
        Söz gelimi case_2' de yine sistemde olmayan reports.txt açılmaya çalışır ve NotFound türünden hata yakalanıp,
        bu kez Create çağrısı ile dosya oluşturulur. Hatta dosya oluşturmak için kullanılan create çağrısı da Result döndüğünden,
        bu işlemin sonucu da pattern matching ile kontrol altında tutulur.
    */
    //case_2_error_kinds();

    /*

        Hata Yönetimi - 3

        İlk iki hata yönetimi pratiğinde görüldüğü üzere pattern matching dalları çoğaldığında kod okunurluğu da yazımı da zorlaşıyor.
        Aslında Result türünün unwrap_or_else fonksiyonu ve closure kullanarak iş daha da basitleştirilebilir.

    */

    case_3_use_unwrap_or_else();

    println!("Programın sonu"); // Üst fonksiyondaki bilinçli panic hali gerçekleşirse(olmayan dosyanın açılması durumu) bu satıra zaten gelinmeyecektir.
}

#[allow(dead_code)]
fn case_1_explicitly_panic_from_result() {
    // File modülündeki open metodu parametre olarak verilen dosyayı read-only modda açmaya çalışır.
    let reader = File::open("reports.txt");
    // match pattern ile Result<T,E> sonuçlarını dallarına göre değerlendirebiliriz.
    let _reader = match reader {
        Ok(file) => file, // Dosya okunabildiyse bu kısım çalışacaktır.
        Err(error) => {
            // Hata olması halinde çalışan kısım da burasıdır.
            panic!("Dosya açılamadı. Hata {:?}", error) // Programcı işleyişi tamamen durdurma kararını vermiş ve panic! makrosunu kullanmıştır.
        }
    };
}

#[allow(dead_code)]
fn case_2_error_kinds() {
    // Yine reports isimli text dosyası açmak istiyoruz
    let reader = File::open("reports.txt");

    // pattern matching uyguladığımız kısım
    let _reader = match reader {
        Ok(file) => file, // Dosya açılabildiyse sorun yok.
        Err(error) => match error.kind() {
            // Dosya bir hata sebebiyle açılamadıysa ErrorKind ile türüne bakıyoruz.
            // Hatta NotFound türünden bir hata söz konusu ise create fonksiyonu ile dosya oluşturmaya çalışıyoruz.
            ErrorKind::NotFound => match File::create("reports.txt") {
                // ve create fonksiyonu da bir Result döndüğünden işlemin başarılı olup olmadığına başka bir matching ile bakıyoruz.
                Ok(created) => {
                    println!("Reports isimli bir dosya oluşturuldu");
                    created
                } // Evet sorun yok, dosya oluşturulabildi.
                Err(e) => panic!(
                    "Dosya oluşturulamadı. Diskte yazma yetkisi olmayabilir.\n {:?}",
                    e
                ), // ya da bir hata sebebiyle dosya oluşturulamadı.
            },
            other_err => panic!("Farklı türden bir hata söz konusu {:?}", other_err), // NotFound haricinden bir hata söz konusu ise burası çalışacak.
        },
    };
}

fn case_3_use_unwrap_or_else() {
    // Bu kez pattern matching yerine unwrap_or_else ve closure blokları kullanılıyor.
    // unwrap_or_else eğer eklendiği fonksiyon başarılı ise hemen sonucu döner. Aksi durum bir hata anlamına gelir parametre olarak açılan kod bloğu çalıştırılır.
    let _reader = File::open("reports.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            // create fonksiyonu da Result<T,E> döndürdüğünden unwrap_or_else kullanılabilir.
            // dosya oluşturulabildiyse hemen sonuç döner. Aksi durumda |e| ile başlayan kod bloğu çalıştırılır.
            File::create("reports.txt").unwrap_or_else(|e| {
                panic!("Dosya oluşturulması sırasında hata. {:?}", e); // sistemin işleyişini kesiyoruz
            })
        } else {
            panic!("Dosya açılması sırasında hata. {:?}", e); // buraya gelindiyse de sistem işleyişi anında kesilir.
        }
    });
}
