use std::f64;
use std::io::{self};

fn main() {
    println!("Ne güzel bir gün öyle değil mi?");
    println!("Adım. {}, {}. {} yaşındayım.", "Burak", "Selim", 45);
    println!(
        "Argümanların pozisyonlanmasına örnektir. {0} {1} {1} {0}",
        3, 2
    );
    println!(
        "Argümanlarda isim kullanımına örnektir. {age} {nickname}",
        age = 45,
        nickname = "red"
    );
    println!(
        "Sayı formatlamaya örnektir. Pi = {0:.10} veya {0:.0} şeklinde yazılabilir",
        f64::consts::PI
    );

    print!("Senin adın");
    println!(" ne?");
    let mut _name = String::new();
    if let Ok(l) = io::stdin().read_line(&mut _name) {
        println!("Merhaba {} (Adın {} byte uzunluğunda)", _name, l);
    } else {
        eprintln!("Bi sorun var sanki");
    }
}
