use std::io::{Read, Write};
///
/// Key Value formatında uygulama ayarlarını tutar.
///
/// # Properties
///
/// pairs isimli Vector kullanılır. (String,String) ile çalışır.
///
pub struct Configuration {
    pairs: Vec<(String, String)>,
}

impl Configuration {
    // Configuration struct nesnesi örneklemek için kullanılan fonksiyon
    pub fn new(_pairs: Vec<(String, String)>) -> Configuration {
        Configuration { pairs: _pairs }
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
/// Belli bir key değerinin karşılığı olan value içeriğini döndüren bir davranış tanımlar.
///
pub trait ValueGetter {
    fn get(&self, s: &str) -> Option<String>;
}

///
/// Konfigurasyonu davranışları
///
pub trait ConfigurationBehaviors {
    fn read(&self, from: &mut impl Read) -> std::io::Result<Configuration>;
    fn write(&self, c: Configuration, to: &mut impl Write) -> std::io::Result<()>;
}

// trait implementasyonları
impl ConfigurationBehaviors for ConfigurationService {
    fn write(&self, c: Configuration, mut to: &mut impl Write) -> std::io::Result<()> {
        for pair in c.pairs {
            writeln!(&mut to, "{0}:{1}", pair.0, pair.1)?;
        }
        Ok(())
    }

    fn read(&self, from: &mut impl Read) -> std::io::Result<Configuration> {
        let mut buffer = String::new();
        from.read_to_string(&mut buffer)?; // Parametre ile taşınan String içeriği oku
        let pairs: Vec<(String, String)> = buffer
            .split_terminator("\n") // Satır bazında ayrıştır
            .map(|row| row.trim()) // boşlukları kaldır
            .filter(|row| {
                // : karakterini de hesaba katarak içeriği filtrele
                let pos = row.find(":").unwrap_or(0);
                pos > 0 && pos < row.len() - 1
            })
            .map(|row| {
                // süzülen içeriği Tuple'a dönüştür
                let parts = row.split(":").collect::<Vec<&str>>();
                (parts[0].to_string(), parts[1].to_string())
            })
            .collect(); // Map edilen içeriği Vector nesnesine al
        Ok(Configuration::new(pairs)) // Ayarları yeni Configuration nesnesi olarak döndür
    }
}

impl ValueGetter for Configuration {
    ///
    /// Key karşılığı olan Value bilgisini Configuration nesnesinden çeker
    ///
    fn get(&self, s: &str) -> Option<String> {
        self.pairs.iter().find_map(|tuple| {
            if &tuple.0 == s {
                Some(tuple.1.clone())
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn should_connenction_pair_write() {
        let sample = Configuration::new(vec![(
            "localhost".to_string(),
            "data source=localhost;instance=london;user id=sa;password=1234".to_string(),
        )]);

        let service = ConfigurationService::new();
        let mut target = vec![];
        assert!(service.write(sample, &mut target).is_ok());

        assert_eq!(
            String::from_utf8(target).unwrap(),
            "localhost:data source=localhost;instance=london;user id=sa;password=1234\n"
                .to_string()
        );
    }

    #[test]
    fn should_connection_pair_read() {
        let service = ConfigurationService::new();
        let some_pairs = &format!("{}\n{}", "username:scoth", "password:tiger").into_bytes();

        // Cursor, Read trait'ini implemente ettiği için ConfigurationService için implemente edilen read trait tarafından da kullanılabilir.
        let config = service
            .read(&mut Cursor::new(some_pairs))
            .expect("İçerik okunamadı");

        assert_eq!(
            config.pairs,
            vec![
                ("username".to_string(), "scoth".to_string()),
                ("password".to_string(), "tiger".to_string())
            ]
        );
    }
}
