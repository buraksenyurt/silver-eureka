//! Tamamen öğrenme amacıyla açılmış önemsiz bir rust modülüdür.
//! Sembolik olarak Event isimli bir veri yapısı ile çalışan Queue modelinde bir koleksiyon ve ilgili fonksiyonlarını barındırır.
//!
//! # Example
//!
//! ```
//! use std::time::SystemTime;
//! use event_queue::Event;
//!
//! let t = SystemTime::now();
//! let e = Event::new("Product_Added".to_string(), t);
//! ```

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_event() {
        let t = SystemTime::now();
        let e = Event::new("Product_Added".to_string(), t);
        assert_eq!(e.to_str(), format!("Product_Added [{:?}]", t));
    }
}

use std::time::SystemTime;

/// Bir olayı sembolize eden veri yapısıdır.
/// 
/// # Arguments
/// 
/// * `name` - Olayın adı.
/// * `time` - Olayın zamanı.
pub struct Event {
    pub name: String,
    pub time: SystemTime,
}

/// Event veri türü ile ilgili fonksiyonlar.
impl Event {
    /// Yeni bir event değişkeni oluşturur.
    pub fn new(n: String, t: SystemTime) -> Event {
        Event { name: n, time: t }
    }

    // Event içeriğini String formatta dönen fonksiyondur
    pub fn to_str(&self) -> String {
        format!("{} [{:?}]", self.name, self.time)
    }
}
