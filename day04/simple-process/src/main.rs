use std::process::{Command, Stdio};

/*
    Bu uygulamanın amacı weather isimli binary'yi parametreler ile çağırıp sonuçlarını almaktan ibaret.
*/
fn main() {
    // Process başlatmak için Command crate'inin new operasyonu ile başlayan metot zincirini kullanabiliriz.
    // exe'nin yolunu belirttik ama OS'a ait ve path tanımlarında yer alan binary'ler de kullanılabilir.
    let weather = Command::new("C:\\Users\\burak\\source\\repos\\silver-eureka\\day04\\weather\\target\\debug\\weather.exe")
        .args(["istanbul", "3"]) // diğer uygulamanın dışardan aldığı parametreleri veriyoruz.
        .stdout(Stdio::piped()) // weather uygulamasının verdiği çıktıyı almak için bir kanal bildirimi
        .output()
        .expect("Process başlatılamadı"); // process başlatılamadığı durumda oluşacak panic mesajımız

    let result = String::from_utf8_lossy(&weather.stdout); // diğer uygulama çıktısını UTF8 formatında alıyoruzaldık
    println!("{}", result); // ve bu uygulamanın ekranına bastık    
}
