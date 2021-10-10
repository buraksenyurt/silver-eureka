use std::io::{Read, Write};
///
/// Key Value formatında uygulama ayarlarını tutar.
///
/// # Properties
///
/// values isimli Vector kullanılır. (String,String) ile çalışır.
///
pub struct Configuration {
    values: Vec<(String, String)>,
}

impl Configuration {
    // Configuration struct nesnesi örneklemek için kullanılan fonksiyon
    pub fn new(_values: Vec<(String, String)>) -> Configuration {
        Configuration { values: _values }
    }
}

///
/// Konfigurasyon yönetim servisidir.
///
pub struct ConfigurationService {}

impl ConfigurationService {
    // ConfigurationService struct nesnesi örneklemek için kullanılan fonksiyon
    pub fn new() -> ConfigurationService {
        ConfigurationService {}
    }
}

///
/// Belli bir key değerinin karşılığı olan value içeriğini döndüren fonksiyonun temsilidir
///
pub trait ValueGetter {
    fn get(&self, s: &str) -> Option<String>;
}

pub trait ConfigurationReader {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Configuration>;
}

pub trait ConfigurationWriter {
    fn write(&self, c: Configuration, to: &mut impl Write) -> std::io::Result<()>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
