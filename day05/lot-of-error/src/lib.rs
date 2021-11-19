#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_errors_work_test() {
        let pos_err = format!("{}", ErrorWrapper::Device(PosConnectionError(13)));
        assert_eq!(
            "13 numaralı cihaz erişilebilir durumda değil. [İletişim Problemi.]",
            pos_err
        );

        let opr_err = format!(
            "{}",
            ErrorWrapper::Operation(MissingOperationError("SendPhoto"))
        );
        assert_eq!(
            "Çağırılmak istenen 'SendPhoto' operasyonu bulunamadı. [Olmayan Servis Operasyonu.]",
            opr_err
        );

        let io_err = format!(
            "{}",
            ErrorWrapper::FileSystem(io::Error::from(io::ErrorKind::InvalidData))
        );
        assert_eq!("invalid data [invalid data]", io_err);
    }
}

use std::error::Error; // Error trait'i için gerekir
use std::fmt; // Display trait'i için gerekir
use std::io;

/*
    User-Defined Errors

    Hata tanımlamaları da nihayetinde bir struct ile ifade edilen veri yapılarıdır.
    Error kelimesi ile bitmeleri bir isimlendirme standardıdır.
    Aşağıda üç örnek tanım görülmekte.
*/
#[derive(Debug)]
struct PosConnectionError(usize);
#[derive(Debug)]
struct MissingOperationError<'a>(&'a str); // string referans olduğunda lifetime parametresi kullandık
#[derive(Debug)]
struct UnexpectedCommError();

// Birden fazla hatanın ele alınacağı durumlarda bunları temsil eden bir enum sabiti kullanmak yararlıdır
#[derive(Debug)]
enum ErrorWrapper<'a> {
    Device(PosConnectionError),
    Operation(MissingOperationError<'a>),
    Unknown(UnexpectedCommError),
    FileSystem(io::Error),
}

/*
    Standart olarak hatalar bir description fonksiyonu sunarlar.
    Kendi tanımladığımız hatalara Error trait'ini uygulayarak bu davranışı kazandırabiliriz.
*/
impl<'a> Error for ErrorWrapper<'a> {
    // description fonksiyonu hata ile ilgili kısa bilgi döner.
    fn description(&self) -> &str {
        // match ile ErrorWrapper enum sabitindeki olası tüm durumlar ele alınır.
        match *self {
            ErrorWrapper::Device(_) => "İletişim Problemi.",
            ErrorWrapper::Operation(_) => "Olmayan Servis Operasyonu.",
            ErrorWrapper::FileSystem(ref e) => e.description(), // Io, dosya girdi çıktı hatası ve rust'ın önceden tanımlı yapılarından. Dolayısıyla onun description fonksiyon sonucunu döndürüyor.
            _ => "Bilinmeyen Hata.", // Yukarıdaki durumlar haricindeki bir durum oluşması halinde bu kısım çalışır.
        }
    }
}

/*
    Hatalara ait detayların sistemin Display davranışını çağıran noktalarda kullanılabilmesi için,
    ErrorWrapper tipine bu Trait'in de uygulanması ve fmt fonksiyonunun yazılması gerekir.
*/
impl<'a> fmt::Display for ErrorWrapper<'a> {
    /*
        fmt fonksiyonunun ikinci parametresi metin içeriği formatlamak isteyen Formatter tipindendir.
        write! makrosu yardımıyla parametre olarak gelen Formatter üstüne gerekli hata bilgileri yazdırılır.
    */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ErrorWrapper::Device(ref e) => write!(
                f,
                "{} numaralı cihaz erişilebilir durumda değil. [{}]",
                e.0,                // PosConnectionErro'un sayısal değeri
                self.description()  // Kısa tanımı.
            ),
            ErrorWrapper::FileSystem(ref e) => write!(f, "{} [{}]", e, self.description()),
            ErrorWrapper::Operation(ref e) => write!(
                f,
                "Çağırılmak istenen '{}' operasyonu bulunamadı. [{}]",
                e.0,
                self.description()
            ),
            ErrorWrapper::Unknown(_) => write!(
                f,
                "İletişim sırasında bilinmeyen bir hata oluştu. [{}]",
                self.description()
            ),
        }
    }
}
