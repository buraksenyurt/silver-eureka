use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

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

    // case_3_use_unwrap_or_else();

    /*

        Hata Yönetimi - 4

        3ncü vakada kullanılan unwrap_or_else pratiği yerine çok daha kısa bir yol ile panic üretilmesi de sağlanabilir.
        Result türünün yararlı metotlarından birisi de unwrap'tır.
        Fonksiyon başarılı ise unwrap doğrudan Ok(T) cevabının sahip olduğu değeri döndürür, aksi durumda ise otomatik olarak üretilen hatayı panik olarak ortama yollar.
        Ki panik durumunda da sistemin işleyişi anında kesilecektir.

        Diğer yararlı bir fonksiyon da expect'tir. unwrap gibi yine fonksiyon çağrısı başarılı ise dönen değer anında elde edilir.
        Hata olması durumunda parametre olarak gelen mesaj da kullanılır.
    */
    // Sistemde bu dosya gerçekten yoksa unwrap işleyişi hata üretecek ve panik oluşacaktır.
    // let _reader = File::open("olmayanDosya.txt").unwrap();

    // let _reader = File::open("olmayanDosya.txt").expect("Dosya açılamadı. Dosyanın sistemde olup olmadığından emin olun.");

    /*

        Hata Yönetimi - 5 (Propogating Errors)

        Hata ihtimali taşıyan fonksiyonlardan Result<T,E> döndürülmesi halinde oluşan hatalar çağıran üst kodlara da taşınır.
        Örneğin aşağıdaki fonksiyon geriye Result<T,E> döndürmekte. İçerisinde oluşacak hatalar main fonksiyonu tarafından ele alınabilir.

    */
    // let _file_content =
    //     case_5_propogating_error("colors.txt").expect("Alt çağrıda bir hata oluştu.");

    /*

        Hata Yönetimi - 6

        Hataları bir üst tarafa fırlatırken kullanılabilecek oldukça kısa bir yol daha vardır; ? operatörü.
        ? operatörü eklenebilen fonksiyonlarda eğer işlem başarılı ise anında Ok() ile sonuç dönülür. Aksi durumda oluşan Err çağıran koda döndürülür.
        Bu nedenle case_5_propogating_error içeriğini çok daha yalın bir hali yazılabilir.

    */
    let content = case_6_question_mark_shortcut("none.txt");
    match content {
        Ok(c) => println!("{}", c),
        Err(e) => println!("Dosyadan okuma işleminde bir hata var. {:?}\n", e),
    };

    let content = case_6_question_mark_shortcut("names.txt").unwrap();
    println!("Names dosya içeriği\n\n{}", content);

    println!("\nProgramın sonu"); // Üst fonksiyondaki bilinçli panic hali gerçekleşirse(olmayan dosyanın açılması durumu) bu satıra zaten gelinmeyecektir.
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

#[allow(dead_code)]
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

// Fonksiyon okunan dosya içeriğini döndürmek için kullanılıyor.
#[allow(dead_code)]
fn case_5_propogating_error(file_name: &str) -> Result<String, io::Error> {
    let reader = File::open(file_name);

    let mut reader = match reader {
        Ok(file) => file,
        Err(error) => return Err(error), // Hata oluştuysa bunu geriye dönüyoruz
    };

    let mut content = String::new();

    match reader.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(error) => Err(error), // Dosya açıkken içeriğini okuyamadıysak oluşan hatayı da fırlatıyoruz. Burada açıkça return kullanılmamıştır keza fonksiyonun son işlevsel ifadesidir.
    }
}

fn case_6_question_mark_shortcut(file_name: &str) -> Result<String, io::Error> {
    let mut content = String::new();
    // Aşağıdaki çağrılara dikkat.
    // open ve read_to_string sonunda ? operatörü var.
    // Bu operatör ile case_5 fonksiyonundaki işlevselliğin aynısı sağlanmakta.
    File::open(file_name)?.read_to_string(&mut content)?;
    Ok(content)
}
