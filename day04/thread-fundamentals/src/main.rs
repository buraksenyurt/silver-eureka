use std::thread;
use std::time::Duration;

// İlk çalışmada ana thread içinden yeni bir thread başlatılıyor
fn main() {
    // bir thread başlatmak için spawn fonksiyonu kullanılabilir
    thread::spawn(|| {
        for i in 1..10 {
            println!("Spwaned\t\t{}", i);
            thread::sleep(Duration::from_millis(200)); //sembolik bir duraksatma (200 milisaniye)
        }
    });

    // ana thread içinde çalışan bir döngü
    for i in 1..5 {
        println!("Main\t\t{}", i);
        thread::sleep(Duration::from_millis(100)); // sembolik bir duraksatma (100 milisaniye)
    }
}
