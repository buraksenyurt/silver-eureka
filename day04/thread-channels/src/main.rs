// Örnekte kanallar yardımıyla thread'ler arası mesaj bazlı haberleşme konusu ele alınmakta.
// Burada Multi Producer Single Consumer olarak da ifade edebileceğimiz mpsc modülü kullanılmakta.
// FIFO (First In First Out) ilkesine göre çalışan bir yapı.
use std::sync::mpsc::{Sender,Receiver};
use std::sync::mpsc;
usd std::thread;
use rand::prelude::*;
use std::time::Duration;

fn main() {
    
}

enum Symbol{
    Smile(usize),
    Hearth(usize),
}
