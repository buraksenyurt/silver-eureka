use std::io;

///
/// Bir iş fonksiyonunun çalışma sonucunu raporlayan enum sabitidir
///
pub enum BusinessRetCode {
    Code { defination: String, short_code: u16 }, // struct türünden bir özellik
    Summary(String),                              //String tipi encapsulate ediyor
    FileError(io::Error),                         // Error tipini encapsulate ediyor
    Unknown,
}

impl BusinessRetCode {
    // Enum sabiti için bir fonksiyon tanımı söz konusu
    pub fn write(&self, mut to: &mut impl io::Write) -> io::Result<()> {
        // kendi değerine bakıyoruz
        let value = match self {
            BusinessRetCode::Code {
                defination: _,
                short_code: _,
            } => "Code",
            BusinessRetCode::Summary(_) => "Summary",
            BusinessRetCode::FileError(_) => "FileError",
            BusinessRetCode::Unknown => "Unknown",
        };
        // Write metodunu uygulayan bir tip içinde kullandırıyoruz
        write!(&mut to, "{}", value)?;
        Ok(())
    }
}

///
/// Belli bir sayısal değere göre uygun BusinessRetCode döndüren fonksiyon
///
pub fn check(state: i32) -> BusinessRetCode {
    if state == 1 {
        BusinessRetCode::Code {
            defination: "İşlem başarılı".to_string(),
            short_code: 32,
        }
    } else if state == 0 {
        BusinessRetCode::Summary(
            "İşlem sırasında hata oluştu. Lütfen sorumluya bilgi verin.".to_string(),
        )
    } else if state == 99 {
        BusinessRetCode::FileError(io::Error::from(io::ErrorKind::Other))
    } else {
        BusinessRetCode::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::{check, BusinessRetCode};

    #[test]
    fn should_check_state_for_1() {
        let state = 1;
        let ret_code = check(state);
        match ret_code {
            BusinessRetCode::Code {
                defination: _,
                short_code: _,
            } => assert_eq!(state, 1),
            BusinessRetCode::FileError(_) => assert_eq!(state, 99),
            BusinessRetCode::Summary(_) => assert_eq!(state, 0),
            BusinessRetCode::Unknown => assert!(state < 0),
        }
    }

    #[test]
    fn should_check_state_for_0() {
        let state = 0;
        let ret_code = check(state);
        match ret_code {
            BusinessRetCode::Code {
                defination: _,
                short_code: _,
            } => assert_eq!(state, 1),
            BusinessRetCode::FileError(_) => assert_eq!(state, 99),
            BusinessRetCode::Summary(_) => assert_eq!(state, 0),
            BusinessRetCode::Unknown => assert!(state < 0),
        }
    }

    #[test]
    fn should_check_state_for_99() {
        let state = 99;
        let ret_code = check(state);
        match ret_code {
            BusinessRetCode::Code {
                defination: _,
                short_code: _,
            } => assert_eq!(state, 1),
            BusinessRetCode::FileError(_) => assert_eq!(state, 99),
            BusinessRetCode::Summary(_) => assert_eq!(state, 0),
            BusinessRetCode::Unknown => assert!(state < 0),
        }
    }

    #[test]
    fn should_check_state_for_any() {
        let state = -1;
        let ret_code = check(state);
        match ret_code {
            BusinessRetCode::Code {
                defination: _,
                short_code: _,
            } => assert_eq!(state, 1),
            BusinessRetCode::FileError(_) => assert_eq!(state, 99),
            BusinessRetCode::Summary(_) => assert_eq!(state, 0),
            BusinessRetCode::Unknown => assert!(state < 0),
        }
    }
}
