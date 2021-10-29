#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_player_can_serialize_to_json_test() {
        let duke_nukem = Player::new("duke nukem".to_string(), 88, "Other Earth".to_string());
        // Player değişkenini serileştirmek için serde_json crate içindeki bir fonksiyondan yararlandık
        let serialized = Player::serialize(&duke_nukem);
        assert_eq!(
            serialized,
            "{\"nickname\":\"duke nukem\",\"level\":88,\"region\":\"Other Earth\"}"
        );
    }

    #[test]
    fn should_player_can_deserialize_from_json_test() {
        let data = r#"
        {
            "nickname": "tomb rider",
            "level": 94,
            "region": "Red Squad"
        }"#;

        let p = Player::deserialize(data.to_string());
        assert_eq!(p.nickname, "tomb rider");
        assert_eq!(p.level, 94);
        assert_eq!(p.region, "Red Squad");
    }

    #[test]
    fn should_valid_dates_test() {
        let some_date: &str = "12.04.1976";
        assert_eq!(is_valid_date(some_date), Some("12.04.1976"));
        let some_date: &str = "12.04.196";
        assert_eq!(is_valid_date(some_date), None);
    }
}

static DATE_EXPRESSION: &str = r"(?x)((?P<day>\d{2}).(?P<month>\d{2}).(?P<year>\d{4}))";
// yyyy-mm-dd için basit bir regex formatını sabit olarak tanımladık
// Gelen string değerinin yukarıdaki regex ifadesine uyup uymadığını kontrol eden fonksiyondur
pub fn is_valid_date(s: &str) -> Option<&str> {
    let date_regex = Regex::new(DATE_EXPRESSION).unwrap(); // Regex crate'inden bir nesne örneklenir

    // is_some'ın true dönüp dönmeme haline göre Option ile sonucu dönmekteyiz
    match date_regex.captures(&s).is_some() {
        true => Some(s),
        _ => None,
    }
}

use regex::Regex; // regex işlemleri için kullanılan yardımcı crate
use serde::Deserialize; // regex ters serileştirme işlemleri için kullanılan crate
use serde::Serialize; // serileştirme işlemleri için kullanılan yardımcı crate

// Serileşebilir kobay bir struct. Hem struct -> json hem de json -> struct için sırasıyla Serialize ve Deserialize trait'lerine sahip olunması gerekir
#[derive(Serialize, Deserialize)]
pub struct Player {
    pub nickname: String,
    pub level: i32,
    pub region: String,
}

impl Player {
    pub fn new(nick: String, level: i32, region: String) -> Player {
        Player {
            nickname: nick,
            level: level,
            region: region,
        }
    }

    // Player nesnesinin kendisini serde_json yardımıyla json formatına serileştirne fonksiyon
    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    // JSON string kullanarak tekrar Player nesnesi döndüren fonksiyon
    pub fn deserialize(s: String) -> Player {
        let p: Player = serde_json::from_str(&s).unwrap();
        p
    }
}
