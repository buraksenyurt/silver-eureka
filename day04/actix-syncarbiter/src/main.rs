/*     
    Burada async-with-actic örneğini baz alıyoruz.
    Lakin aktörleri birden fazla instance olarak paralel çalıştırmak isteyeceğimiz senaryolar olabilir.
    SyncArbiter burada devreye giriyor.
*/
use actix::prelude::*;
use rand::Rng;
use std::thread;
use std::time::{Duration};

#[derive(Debug, Message)]
#[rtype(result = "()")]
struct TempData(pub u64, pub f64);

struct LogWriter;

impl Actor for LogWriter {
    type Context = SyncContext<Self>; // Arbiter için SyncContext kullanılmalı
}

impl Handler<TempData> for LogWriter {
    type Result = ();

    fn handle(&mut self, message: TempData, _: &mut Self::Context) {
        println!("Thread {:?}\nAnlık motor sıcaklığı bilgileri.\n\t{:?}",thread::current().id(),message);
        thread::sleep(Duration::from_millis(5000));
    }
}

fn main() -> std::io::Result<()> {
    let system = System::new();

    system.block_on(async {
        println!("Çıkmak için Ctlr+C...");
        // Çok kanallı bir aktör host başlatıyoruz. Bu nesneye hakem(arbiter) diyebiliriz.
        // Paralel olarak 4 tane sender çalışacak.
        // Bu nedenle her 5 saniyede 4 sıcaklık göstergesi ekrana yansıyacak.
        let sender = SyncArbiter::start(4, || LogWriter);

        for i in 0..20 {
            let event_id = i as u64;
            let data=read_engine_temperature(); 
            sender.do_send(TempData(event_id,data));
        }
    });

    system.run()
}

// Motor sıcaklığını ölçen bir sensörü taklit eden mock fonksiyon
fn read_engine_temperature() -> f64 {
    let mut rng = rand::thread_rng();
    let heat: f64 = rng.gen_range(-40.0..80.0);
    heat
}
