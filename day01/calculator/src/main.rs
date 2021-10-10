mod basic; // basic modül bildirimi. Önce bu bildirim yapılır, cargo run sonrası uyarı mesajına göre basic klasörü açılır ve içerisine mod.rs dosyası eklenir.

// internal crate
mod console {
    use rust_matlib::*; // harici(external) crate kullanacağımızı belirtiyoruz. crate bağımlılığı toml dosyasında tanımlı.

    pub fn create_location() {
        let random_loc = get_location();
        println!(
            "{0}:{1} koordinatlarına bir işaret kondu",
            random_loc.x, random_loc.y
        );
    }
}
use crate::basic::sum;
use console::create_location; // dahili(internal) crate kullanacağımızı belirttik.
fn main() {
    create_location();
    println!("2.3+3.2={}", sum(2.3, 3.2));
}
