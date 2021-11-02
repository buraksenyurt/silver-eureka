// Örnekte kanallar yardımıyla thread'ler arası mesaj bazlı haberleşme konusu ele alınmakta.
// Burada Multi Producer Single Consumer olarak da ifade edebileceğimiz mpsc modülü kullanılmakta.
// FIFO (First In First Out) ilkesine göre çalışan bir yapıdır.
// Çok sayıda mesaj göndericisi ve tek bir alıcı olan senaryolarda kullanılır.
// İlk kullanımda Simple Streaming söz konusu.
use rand::prelude::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    // Önce basit bir streaming kanalı oluşturulur
    // Bir gönderici ve bir alıcı mevcuttur.
    let (s, r): (Sender<Symbol>, Receiver<Symbol>) = mpsc::channel();

    let pingpong_sender = s.clone(); // bir sender kopyası aldık

    // thread başlatıyoruz
    // içinde sonsuz bir döngü söz konusu
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        // send metodu ile Symbol tipinden bir enum sabitini kanala yolluyoruz.
        pingpong_sender
            .send(Symbol::PingPong(random::<usize>() % 10)) //rastgele sayının 10lu modunu enum sabitine parametre olarak veriyoruz. Kaç tane sembol üreteceğimizi belirtecek.
            .unwrap();
    });

    // yeni bir sender kopyası aldık
    let bowling_sender = s.clone();
    // içinde sonsuz döngü barındıran yeni bir thread açıyoruz
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        // bowling_sender bu kez Bowling türünden bir enum sabitini kanala gönderiyor
        bowling_sender
            .send(Symbol::Bowling(random::<usize>() % 10))
            .unwrap();
    });

    let rugby_sender = s.clone();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        rugby_sender
            .send(Symbol::Rugby(random::<usize>() % 10))
            .unwrap();
    });

    // burada receiver ile kanala gelen mesajları almaya başlıyoruz.
    // recv() çağrısı ana thread'i blokluyor.
    // Bir mesaj geldiği sürece bu while döngüsü çalışmaya devam edecek.
    while let Ok(incoming_item) = r.recv() {
        println!(
            "{}",
            // println! makrosunda pattern matching kullanıyoruz
            // gelen öğe Symbol enum sabitinin hangi değerine uyuyorsa
            // yine sender tarafında üretilen rastgele sayı kadar bir karakteri ekrana bastırıyoruz.
            match incoming_item {
                Symbol::PingPong(c) => "🏓".repeat(c + 1),
                Symbol::Bowling(c) => "🎳".repeat(c + 1),
                Symbol::Rugby(c) => "🏉".repeat(c + 1),
            }
        );
    }
}

enum Symbol {
    PingPong(usize),
    Bowling(usize),
    Rugby(usize),
}
