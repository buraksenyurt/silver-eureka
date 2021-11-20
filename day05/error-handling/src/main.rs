use std::fs::File;

fn main() {
    /*
        Hata Yönetimi - 1

        Kurtarılabilir hata durumlarında da programcı panic! makrosunu kullanarak işleyişi durdurabilir.
        Aşağıdaki örnekte sistemde olmaya reports.txt isimli dosya okumak amacıyla açılmak isteniyor.
    */

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

    println!("Programın sonu"); // reports.txt yoksa panic! makrosu işletileceğinden program zaten bu satıra gelmeyecektir.
}
