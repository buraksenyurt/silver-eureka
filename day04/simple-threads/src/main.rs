use std::thread;
use std::time::duration;

fn main() {
    let thread_1 = do_something();

    for _ in 0..10 {
        println!(".");
    }

    println!(
        "Başlatılan thread'in sonlanması için bekleniyor. {:?}",
        thread_1.join()
    );
}

// Fonksiyon bir thread başlatmakta.
fn do_something() -> thread::JoinHandle<()> {
    thread::spawn(|| {
        println!("Bir takım işler...");
        thread::sleep(Duration::from_sec(5)); // Uzun bir işi simüle etmek için...
        prinln!("İşler tamamlandı.");
    })
}
