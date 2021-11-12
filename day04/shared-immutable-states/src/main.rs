//use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

/*
Single Source of Truth. Verinin tek bir kaynakta olması ilkesi.

Hatırlanacağı üzere kanallar yardımıyla thread'ler arası haberleşmemiz mümkündür.
*/

fn main() {
    start();
}

fn start() {
    // sender ve receiver aldığımız bir kanal nesnesi tanımlanır.
    let (s, r) = channel::<usize>();

    // bir thread açıp sender'ın bir klonunu başka bir fonksiyona göndermekteyiz.
    let t1 = thread::spawn(move || {
        println!("Thread {:?}", thread::current().id());
        let sender_clone = s.clone();
        do_something(sender_clone);
    });

    let r = Arc::new(r); // Receiver için Mutex ve thread safe bir atomic referans nesnesi tanımlandı.

    // İkinci bir thread açtık ve bu kez receiver'ın klonunu sender'ın klonunu da gönderdiğimiz aynı fonksiyona yolladık.
    // Bu kullanım şekliyle derleme hatası alırız. `std::sync::mpsc::Receiver<usize>` cannot be shared between threads safely.
    let t2 = thread::spawn(move || {
        println!("Thread {:?}", thread::current().id());
        do_something(r.clone());
    });

    t1.join();
    t2.join();
}

fn do_something<T>(input: T)
where
    T: std::fmt::Debug,
{
    println!("{:?}", input);
}
