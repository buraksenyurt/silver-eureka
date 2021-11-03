use std::sync::{Arc, Mutex}; //Arc(Atomic Reference Counter).
use std::thread;
use std::time::Duration;

/*
    Örnek senaryoda aynı vector içerisinde veri ekleyen 4 farklı thread çalıştırılmakta. Burada aynı vektöre birden fazla thread'in erişebilmesi ve üzerinde kitlenmeler
    olmadan değişiklik yapabilmesi(örneğin veri eklemek) Mutex ve Arc yapılarından yararlanılır.
    İş parçacıkları arasında belleği paylaşımlı olarak kullanabilmek için thread safe reference counting pointer olarak geçen Arc kullanılır.
    Arc içinde referans edilen verilerin korunması için Mutex gerekli kilit koyma işlemlerini üstlenir.
*/

fn main() {
    let shared_data = Arc::new(Mutex::new(vec![])); //Bir vector için Mutex ve thread safe atomic referans nesnesi oluşturduk
    let threads: Vec<thread::JoinHandle<()>> =
        (0..4).map(|_| collector(shared_data.clone())).collect(); // 4 defa collector fonksiyonunu çağırıyoruz ki o da hemen bir JoinHandle dönüyor
    let _: Vec<()> = threads.into_iter().map(|t| t.join().unwrap()).collect(); // Ana uygulama thread'ini diğer thread işleri tamamlanıncaya kadar duraksatıyoruz.
    println!("Result: {:?}", shared_data); // shared_data tarafından refere edilen vektor içeriğini ekrana basıyoruz
}

// içinde iş parçacığı başlatan fonksiyonumuz.
// parametre olarak Arc nesnesini almakta.
fn collector(shared_data: Arc<Mutex<Vec<Number>>>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        // iş parçacığında sonsuz döngü söz konusu lakin
        // data ile ifade edilen vector'ün eleman sayısı 10'a ulaşınca terk ediliyor.
        println!("Bulunduğumuz Yer?\t{:?}", thread::current().id());

        // local iş parçacığını burada kitliyoruz
        let mut data = shared_data.lock().unwrap();

        if data.len() > 0 {
            // Pattern Matching özelliğinde yararlanarak
            // vetkröün bir önceki elemanına bakıyoruz.
            // Odd ise Even, Even ise Odd ekliyoruz.
            match data[data.len() - 1] {
                Number::Odd => {
                    println!("\t{:?}\tEven ekliyor.", thread::current().id());
                    data.push(Number::Even);
                }
                Number::Even => {
                    println!("\t{:?}\tOdd ekliyor.", thread::current().id());
                    data.push(Number::Odd);
                }
            }
        } else {
            // Eğer vektör içinde hiç eleman yoksa Odd ile başlıyoruz
            data.push(Number::Odd)
        }
        // Sonsuz döngü vektör uzunluğu 10a gelince sonlanıyor
        //
        if data.len() > 10 {
            break;
        }

        thread::sleep(Duration::from_secs(1));
    })
}
#[derive(Debug)]
enum Number {
    Odd,  //tex
    Even, //Çift
}
