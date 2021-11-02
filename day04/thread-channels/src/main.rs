// Örnekte kanallar yardımıyla thread'ler arası mesaj bazlı haberleşme konusu ele alınmakta.
// Burada Multi Producer Single Consumer olarak da ifade edebileceğimiz mpsc modülü kullanılmakta.
// FIFO (First In First Out) ilkesine göre çalışan bir yapı.
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

    let smile_sender = s.clone(); // bir sender kopyası aldık

    // thread başlatıyoruz
    // içinde sonsuz bir döngü söz konusu
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        // send metodu ile Symbol tipinden bir enum sabitini kanala yolluyoruz.
        smile_sender
            .send(Symbol::Smile(random::<usize>() % 10))
            .unwrap();
    });

    // yeni bir sender kopyası aldık
    let hearth_sender = s.clone();
    // içinde sonsuz döngü barındıran yeni bir thread açıyoruz
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        // hearth_sender bu kez Heart türünden bir enum sabitini kanala gönderiyor
        hearth_sender
            .send(Symbol::Hearth(random::<usize>() % 10))
            .unwrap();
    });
}

enum Symbol {
    Smile(usize),
    Hearth(usize),
}
