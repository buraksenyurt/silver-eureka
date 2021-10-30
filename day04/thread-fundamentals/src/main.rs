use std::thread;
use std::time::Duration;

// İlk çalışmada ana thread içinden yeni bir thread başlatılıyor.
// İkinci çalışmada join ile ana thread'den uzun yaşayan thread'lerin işleyişini bitirmesi bekleniyor.
// Üçüncü çalışmada move operatörü ile değişken sahipliğinin bir thread'e alınmasına bakılıyor.
fn main() {
    // //case_1();
    // let t1 = case_2(); // case_2 fonksiyonu JoinHandle döndürür.
    // t1.join().unwrap(); // 14ncü satırdaki join çağrısını ana thread'in işleteceği for döngüsü önüne alırsak.
    //                     // ana thread içinde çalışan bir döngü
    // for i in 1..5 {
    //     println!("Main\t\t{}", i);
    //     thread::sleep(Duration::from_millis(100)); // sembolik bir duraksatma (100 milisaniye)
    // }
    // //t1.join().unwrap(); // Ana thread'i t1'in işaret ettiği thread işleyişini bitirene kadar bekletecektir.

    // case 3: data değişkeninin içeriğini t2'nin ele aldığı diğer thread'de kullanmak istiyoruz.
    // Normalde spawn ile başlatılan blok içerisinde data'nın iter fonksiyonu ile elemanlarında dolaşmak mümkün olmalı.
    // Ancak Rust spawn ile başlatılan thread'in ne kadar süreceğini bilemez ve bu nedenle data referansının halen daha geçerli olup olmadığını anlayamaz.
    // Bilinçli olarak data'nın sahipliğinin closure tarafından alınabilmesine izin vereceğimizi belirtmeliyiz.(move kullanımı)

    let data = vec![0, 1, 1, 2, 3, 5, 8, 13, 21];

    //let t2 = thread::spawn(|| {
    let t2 = thread::spawn(move || {
        println!("Açılan thread hesaplamalara başlıyor.");
        let mut total = 0;
        for v in data.iter() {
            total += v;
            thread::sleep(Duration::from_millis(200));
        }
        println!("\nToplam\t\t{}", total);
    });

    // println!("{}", data.len()); // Bu satırda 'value used here after move' hatası alınır.
    // Yani move kullanımı ile data'nın sahipliğini t2 thread'inin kapsamına taşırken, ana thread'in artık data vektörünü kullanmayacağını garanti etmiş oluyoruz.

    t2.join().unwrap();
}

fn case_2() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Spwaned\t\t{}", i);
            thread::sleep(Duration::from_millis(200)); //sembolik bir duraksatma (200 milisaniye)
        }
    })
}

// fn case_1() {
//     // bir thread başlatmak için spawn fonksiyonu kullanılabilir
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("Spwaned\t\t{}", i);
//             thread::sleep(Duration::from_millis(200)); //sembolik bir duraksatma (200 milisaniye)
//         }
//     });

//     // ana thread içinde çalışan bir döngü
//     for i in 1..5 {
//         println!("Main\t\t{}", i);
//         thread::sleep(Duration::from_millis(100)); // sembolik bir duraksatma (100 milisaniye)
//     }
// }
