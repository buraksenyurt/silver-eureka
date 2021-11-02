use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // #0
    // başlangıçta data isimli bir vektörümüz var.
    // İçinde n sayıda vector içerdiğini varsayalım.
    // Amacımız her bir vektör için birer iş parçacığı başlatmak.
    // Bu iş parçacığında sembolik olarak vektor içindeki bir eleman üstünde matematik işlem yapalım.
    let data_set = vec![
        vec![1, 1, 1],
        vec![2, 2, 2],
        vec![6, 6, 6, 6, 6],
        vec![3, 3, 3],
        vec![4, 4, 4, 4],
    ]; // Örnek veri setimiz

    let start = Instant::now(); // Toplam işlem süresini bulacağız

    // map fonksiyonundan da yararlanarak her bir vektör için ayrı bir iş parçacığı başlatıyoruz.
    let threads: Vec<thread::JoinHandle<Vec<i32>>> = data_set
        .clone() // üzerinde çalışacağımı veriyi klonladık. Bellekteki bu ek işi berataraf etmek için Reference Counter bir çözüm olabilir mi?
        .into_iter() // bir iterasyon içerisine aldık
        .map(|part| {
            thread::spawn(move || {
                println!("Thread ID {:?}", thread::current().id()); // İzlemeyi kolaylaştırmak için açılan Thread ID'sini aldık
                                                                  // thread içerisinde vektör verisini de taşımamız lazım. Bu nedenle move operatörünü kullandık. thread'i yine spawn ile başlattık.
                                                                  // sembolik olarak bir duraksatma da yapıyoruz.
                thread::sleep(Duration::from_secs(2));
                // part, gelen vektörü temsil eder. Onu da iterasyona alıp map fonskiyonunu çağırıyoruz. Her birinin değerini 1 artırıyoruz.
                part.into_iter().map(|c| c * 10).collect()
            })
        })
        .collect(); // Açılan JoinHandle<Vec<i32>> sonuçlarını topluyoruz.

    // Açılan paralel thread'lerin tamamının işinin bitmesini bekletmek için başka bir iterasyon çalıştırıyoruz.
    let results: Vec<i32> = threads
        .into_iter()
        .flat_map(|t| t.join().unwrap())
        .collect::<Vec<i32>>();

    let data_set: Vec<i32> = data_set.into_iter().flat_map(|e| e).collect();

    println!(
        "Toplam Süre\t{:?}\nVeri\t\t{:?}\nSonuç\t\t{:?}",
        start.elapsed(), // Geçen zamanı sn cinsinden aldık
        data_set,
        results
    );
}
