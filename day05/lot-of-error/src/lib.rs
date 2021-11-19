#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::io;

/*
    User-Defined Errors

    Hata tanımlamaları da nihayetinde bir struct ile ifade edilen veri yapılarıdır.
    Error kelimesi ile bitmeleri bir isimlendirme standardıdır.
    Aşağıda üç örnek tanım görülmekte.
*/
pub struct PosConnectionError(usize);
pub struct FunctionCallError(usize);
pub sturct UnexpectedCommError{}

// Birden fazla hatanın ele alınacağı durumlarda bunları temsil eden bir enum sabiti kullanmak yararlıdır
pub enum ErrorWrapper{
    Device(PosConnectionError),
    Operation(MissingOperationError),
    Unknown(UnexpectedCommError),
    Io(io::Error)
}

/*
    Standart olarak hatalar bir description fonksiyonu sunarlar.
    Kendi tanımladığımız hatalara Error trait'ini uygulayarak bu davranışı kazandırabiliriz.
*/
impl Error for ErrorWrapper{
    // description fonksiyonu hata ile ilgili bilgi döner.
    fn description(&self)-> &str{
        // match ile ErrorWrapper enum sabitindeki olası tüm durumlar ele alınır.
        match *self{
            ErrorWrapper::Device(_)=>"Cihazla iletişim kurulamıyor.",
            ErrorWrapper::Operation(_)=>"İstenen servis operasyonu bulunamadı.",
            ErrorWrapper::Io(ref e)=>e.description(), // Io, dosya girdi çıktı hatası ve rust'ın önceden tanımlı yapılarından. Dolayısıyla onun description fonksiyon sonucunu döndürüyor.
            _ => "Beklenmeye bir hata söz konusu." // Yukarıdaki durumlar haricindeki bir durum oluşması halinde bu kısım çalışır.
        }
    }
}