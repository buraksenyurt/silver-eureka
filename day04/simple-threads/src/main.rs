use std::thread;
use std::time::Duration;

fn main() {
    // do_something fonksiyonunda bir thread başlatılmakta
    let thread_1 = do_something();

    // o sırada main bir takım işlere başlıyor
    for _ in 0..10 {
        println!(".");
    }

    // bu noktaya gelindiğinde main thread, thread_1 sonlanıncaya kadar bekliyor
    // nitakim join çağrısı söz konusu.
    // Eğer join ile diğer thread'in sonlanmasını bekletmezsek main akışı hemen sonlanacaktır.
    println!(
        "Başlatılan thread'in sonlanması için bekleniyor. {:?}",
        thread_1.join()
    );
}

// Fonksiyon bir thread başlatmakta.
fn do_something() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Bir takım işler...");
        thread::sleep(Duration::from_secs(5)); // Uzun bir işi simüle etmek için...
        println!("İşler tamamlandı.");
    })
}
