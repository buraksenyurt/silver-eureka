// programda kullanacağımız diğer modülleri bildiriyoruz.
// bu kısımların kullanılabilmesi için program klasöründeki cargo.toml dosyasında gerekli dependency bildirimlerinin yapılması gerekir.
use business::*;
use utility::can_play;

fn main() {
    let mut fergy = Player::new("fergy".to_string(), 79);
    println!("Merhaba,{}\nGüncel skorun {}.", fergy.nickname, fergy.level);
    let mut can_fergy_play = can_play(fergy.level);
    println!("Oyuna girebilme durumun {:?}", can_fergy_play);
    fergy.increase_level(3);
    can_fergy_play = can_play(fergy.level);
    println!("Oyuna girebilme durumun {:?}", can_fergy_play);
}
