/*     
    Örnekte bir oyun motorunun sıcaklıklarını yakalayan sensör bilgilerini dinleyen bir aktör söz konusu.
    actix'in bir başka uyarlaması olarak düşünebiliriz. 
*/
use actix::prelude::*;
use rand::Rng;
use std::thread;
use std::time::Duration;

// Mesaj olarak tanımladığımız veri yapısı.
// derive niteliğinde Message kullanarak mesaj olduğunu ifade ettik.
#[derive(Debug, Message)]
#[rtype(result = "()")] // Mesajdan geriye birşey döndürmeyeceğimzi için bu şekilde tanımladık.
struct TempData(pub u64, pub f64);

// Aktörümüz LogWriter.
struct LogWriter;

impl Actor for LogWriter {
    type Context = Context<Self>;
}

// Aktörümüz dinleyici olark TempData mesajını implemente etmeli.
// Sistemin bu mesajı bilmesi içinse generic Handler trait'ini TempData mesajı için işlemeli.
impl Handler<TempData> for LogWriter {
    type Result = ();

    fn handle(&mut self, message: TempData, _: &mut Self::Context) {
        println!("Anlık motor sıcaklığı bilgileri.\n\t{:?}", message);
        thread::sleep(Duration::from_millis(1000));
    }
}

// system.run çağrısının sonucu result türündendir. main'den sisteme bir çıktı vermiş oluyoruz.
fn main() -> std::io::Result<()> {
    // Yeni bir event loop sistemi tanımladık
    let system = System::new();

    // Aktörün işlemlerini icra ettiği yer. Bu işlemler sırasında sistemi bekletiyoruz
    system.block_on(async {
        // Aktör nesnesini oluşturuyoruz.
        LogWriter::create(|ctx| {
            // Mesaj göndermek için adresi aldık
            let address = ctx.address();
            println!("Context bilgisi : {:?}",ctx);
            // sembolik bir döngü
            for i in 0..10 {
                let event_id = i as u64;
                let data=read_engine_temperature(); // sensörde motor sıcaklığını okuduk ki bu iş 1 saniye sürüyor. Gecikmeli bir işlemi simüle ettik.
                address.do_send(TempData(event_id,data)); // aktör adresini kullanarak mesajı yolluyoruz. Artık aktörün handler sisteminde bu mesaj değerlendirilecek.
            }
            LogWriter{}
        });
        System::current().stop(); // işlemlerin sonlandığı yer olarak düşünebiliriz. Sistemi durduruyoruz. Eğer koymazsak sistem bir programı kapatana kadar açık kalır. Sürekli dinelemeli durumlarda bunu tercih edebiliriz.
    });

    system.run() 

    // match result {
    //     Ok(r)=>println!("{:?}",r),
    //     _ =>println!("Bir hata oluştu")
    // }
}

// Motor sıcaklığını ölçen bir sensörü taklit eden mock fonksiyon
fn read_engine_temperature() -> f64 {
    let mut rng = rand::thread_rng();
    let heat: f64 = rng.gen_range(-40.0..80.0);
    heat
}
