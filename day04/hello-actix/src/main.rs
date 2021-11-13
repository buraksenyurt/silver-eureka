/*
    actix, Tokio alt yapısını kullanır. Tokio güvenilir asenkron uygulamalar yazmak için bir çalışma zamanı ve ortam sağlar.
    https://github.com/tokio-rs/tokio


*/
use actix::prelude::*;

// Bir mesaj tanımladık. Örneğin bir istatistik bilgisi göndermek için kullanılacak olsun.
#[derive(Message)]
#[rtype(result = "bool")] // dönüş tipi
struct PublishStat<'a>(usize, &'a str); // string literal bir alanı olduğu için açıkça lifetime belirtmemiz gerekir.

// Aktörümüz
struct Consultant;

// Actor trait'inin sistem tarafından bilinmesi gerekir.
impl Actor for Consultant {
    type Context = Context<Self>;
}

// Aktörün mesaj geldiğinde yapacaklarını Handler yardımıyla tanımlarız
impl Handler<PublishStat<'_>> for Consultant {
    type Result = bool; // mesajdan dönen tür

    // mesajın işlendiği yer
    fn handle(&mut self, message: PublishStat, context: &mut Context<Self>) -> Self::Result {
        println!("{} - {}\n{:?}", message.1, message.0,context); // gelen bilgileri sembolik olarak ekrana yazdırdık.
        true // dönüş
    }
}

#[actix::main] // sistemin olay döngüsünü başlatmanın en kolay yolu bu niteliği main fonksiyonuna uygulamak.
async fn main() { // main fonksiyonunun asenkron tanımlandığına dikkat edelim.
    let address=Consultant.start();
    let response=address.send(PublishStat(12,"İlk periyotta attığı sayı.")).await; // mesajı gönderdiğimiz ve dönüş için beklediğimiz yer

    match response {
        Ok(result)=>println!("İstatistik işlenmek üzere sisteme kaydedildi mi? {}",result),
        _=>println!("İletişimde bir problem olmuş olmalı!")
    }
}
