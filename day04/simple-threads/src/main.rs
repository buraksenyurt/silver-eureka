use std::thread;
use std::time::Duration;

fn main() {
    println!("main thread");
    // #1
    // do_something fonksiyonunda bir thread başlatılmakta
    let thread_1 = do_something();

    // O sırada main bir takım işlere başlıyor
    for _ in 0..10 {
        println!(".");
    }

    // bu noktaya gelindiğinde main thread, thread_1 sonlanıncaya kadar bekliyor
    // nitakim join çağrısı söz konusu.
    // Eğer join ile diğer thread'in sonlanmasını bekletmezsek main akışı hemen sonlanacaktır.
    println!(
        "Başlatılan Thread1'in sonlanması için bekleniyor. {:?}",
        thread_1.join()
    );
    // #1

    // #2
    // Bir diğer thread'in başlatılacağı kısım.
    let n = 1;
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let thread_2 = do_something_with(n, numbers); // parametreleri ile fonksiyon çağrısı
    println!("Artım değeri {}", n); // do_something_with fonksiyonuna parametre olarak gönderip orada kullandığımız n değişkenine main thread içinden erişmeye çalıştığımız yer. move kullanmadığımızda hata alınır.
    println!(
        "Thread2'nin tamamlanmasını bekliyoruz. {:?}",
        thread_2.join()
    );
    // #2

    println!("main thread bitiyor. Bye bye");
}

// #1
// Fonksiyon bir thread başlatmakta.
fn do_something() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Thread1 içinde bir takım işler yapılıyor...");
        thread::sleep(Duration::from_secs(5)); // Uzun bir işi simüle etmek için...
        println!("Thread1 içindeki işler tamamlandı.");
    })
}

// #2
// Yine içinde thread başlatılan ama metoda gelen parametreleri de kullanan bir fonksiyon söz konusu
fn do_something_with(increase: i32, values: Vec<i32>) -> thread::JoinHandle<Vec<i32>> {
    // #3
    thread::spawn(move || {
        println!("Thread2 içindeyiz.");
        //thread::spawn(|| {
        for i in values.iter() {
            // values vektöründe gelen sayıları yanyana basıyoruz
            print!(" {} ", i);
        }
        println!("");
        println!("do_something_with içerisinde {} sayısı var.", increase); //buradaki sahiplik problemini gidermek için move kullanılır
        values // sahipliği(ownersghip) geri döndürdüğümüz nokta
    })
}
