use std::env;
use std::io::{self, Write};

/*
    Bu kobay uygulama ekrandan aldığı iki parametre ile çalışacak.
    İlk parametre ile gelen şehrin ikinci parametre ile gelen gün sayısı kadarlık hava tahminini verecek. Güya ;)
    Amacımız aslında bu uygulamayı başka bir Rust uygulamasında process açarak başlatmak, parametre göndermek ve sonucunu yine o uygulamada okumak yönünde.
*/
fn main() {
    // env isimli crate üstünden args fonksiyonunu çağırarak terminalden girilen parametreleri alıyoruz
    let args: Vec<String> = env::args().collect();
    let city = &args[1];
    let days = &args[2].parse::<i32>().unwrap();

    let result = format!("{} şehri için {} günlük hava tahmini...", city, days);
    // stdout ile sisteme sonuçları uygulama çıktısı olarak vermekteyiz.
    io::stdout().write(result.as_ref());
    io::stdout().flush();
}
