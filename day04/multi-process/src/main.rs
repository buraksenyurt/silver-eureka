use std::io::Write;
use std::process::{Command, Stdio};

/*
    Bu örnekte linux'un cat komutu kullanılmakta.
    cat için ayrı bir alt process açılıyor ve bilgi yazabilmek için bir stdin hattı oluşturuluyor.
    Bu hatta yazılan verinin de yine cat için açılan process üstünden okunması sağlanıyor.
    Yani program içinde açtığımız bir alt process'e gönderdiğimi veriyi yine bu process üstünden okuyoruz.
*/

fn main() {
    // değiştirilebilir şekilde bir command nesnesi örneklenir
    // linux cat komutu kullanılmakta
    let mut cat = Command::new("cat")
        .stdin(Stdio::piped()) // bilgi girdisi için çağırılan fonksiyon. piped ile ana ve alt process'leri bir hat üstünden birbirlerine bağladık
        .stdout(Stdio::piped()) // bilgi çıktısı için de bir hat tesis ediliyor
        .spawn() // process ayrı iş parçacığı olarak tohumlandı
        .expect("Process başlatılamadı.");

    // stdin nesnesini kullanarak veri yazacağız.
    // mutable olması için as_mut kullanılıyor.
    let input_channel = cat.stdin.as_mut().expect("stdin kullanılamıyor.");

    // Hat yardımıyla bir takım bilgiler yazıyoruz.
    input_channel
        .write_all(
            b"Some kind of information about us.\nStrangers in the night...\nLa la la la laaa...",
        ) //b ile string içeriğin binary olarak çevrilmesi sağlanıyor.
        .expect("Child Process'e yazma işlemi başarısız.");

    // Bu satırda ise hat üstünden yazılan bilgiyi okuyoruz.
    let result = String::from_utf8(
        cat.wait_with_output()
            .expect("Ters giden bir şeyler var.")
            .stdout
            .as_slice()
            .iter()
            .cloned()
            .collect(),
    )
    .unwrap();

    println!("{}", result);
}
