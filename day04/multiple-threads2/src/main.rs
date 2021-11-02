// mutliple-language örneğinin fonksiyon kullanılan halidir.
// Kodların anlaşılması güçleşmesin diye.

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let data_set = vec![
        vec![1, 1, 1],
        vec![2, 2, 2],
        vec![6, 6, 6, 6, 6],
        vec![3, 3, 3],
        vec![4, 4, 4, 4],
    ];

    let start = Instant::now();

    // üzerinde işlem yaptıracağımız veri setini klonlayarak calculate fonksiyonuna gönderdik
    let data_results: Vec<i32> = calculate(data_set.clone())
        .into_iter()
        .flat_map(|t| t.join().unwrap())
        .collect();

    println!(
        "Toplam Süre\t{:?}\nVeri\t\t{:?}\nSonuç\t\t{:?}",
        start.elapsed(),
        data_set,
        data_results
    );
}

// Geriye JoinHandle türünden vektör dönen fonksiyon
fn calculate(data: Vec<Vec<i32>>) -> Vec<thread::JoinHandle<Vec<i32>>> {
    data.into_iter() // gelen veri setinde ileri yönlü haraket için iterasyon başlar
        .map(|slice| {
            // her bir vektör slice ile ifade edilir
            thread::spawn(move || {
                //bir thread başlatılır
                println!("Thread ID : {:?}", thread::current().id()); // başlatılan thread no ekrana basılır
                thread::sleep(Duration::from_secs(2)); // geçici bir duraksatma yapılır
                slice.into_iter().map(|n| n * 10).collect() // slice ile ifade edilen vektör üstünde toplu işlem yapılıp sonuçlar JoinHandle referansına çekilir
            })
        })
        .collect()
}
