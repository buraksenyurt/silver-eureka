/* reçetede IoT cihazlarındaki sensör verilerinin toplanması ve yazılması ile ilgili bir örnek icra ediliyor */
use rand::Rng;

// main metodundan sisteme cevap dönebiliriz. Bu sadece Result tipi ile mümkün olabilir.
fn main() -> std::io::Result<()> {
    // sistemi çalıştırıp event loop'u başlatıyoruz.
    System::run(|| {
        println!("Sistemden çıkmak için Ctrl+C...");
        
    })
}

// Motor sıcaklığını ölçen bir sensörü taklit eden mock fonksiyon
fn read_engine_temperature() -> f64 {
    let mut rng = rand::thread_rng();
    let heat: f64 = rng.gen_range(-40.0..80.0);
    heat
}

// Mesaj olarak tanımladığımız veri yapısı.
// derive niteliğinde Message kullanarak mesaj olduğunu ifade ettik.
#[derive(Debug, Message)]
struct TempData(pub u64, pub f64);
