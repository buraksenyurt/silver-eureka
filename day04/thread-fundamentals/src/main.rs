use std::thread;
use std::time::Duration;

// İlk çalışmada ana thread içinden yeni bir thread başlatılıyor
fn main() {
    //case_1();
    let t1 = case_2(); // case_2 fonksiyonu JoinHandle döndürür.
    t1.join().unwrap(); // 14ncü satırdaki join çağrısını ana thread'in işleteceği for döngüsü önüne alırsak.
    // ana thread içinde çalışan bir döngü
    for i in 1..5 {
        println!("Main\t\t{}", i);
        thread::sleep(Duration::from_millis(100)); // sembolik bir duraksatma (100 milisaniye)
    }
    //t1.join().unwrap(); // Ana thread'i t1'in işaret ettiği thread işleyişini bitirene kadar bekletecektir.
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