#[cfg(test)]
mod tests {
    use super::Player;

    #[test]
    fn unwrap_tests() {
        let value = Some(32); // Option değerlerinden birisidir. Some(T)
        assert_eq!(value.unwrap(), 32); // unwrap fonksiyonunun üretim ortamlarında kullanıması önerilmez nitekim hata durumlarında kontrolsüz panic oluşmasına sebep olur
        assert_eq!(None.unwrap_or(5), 5);
        assert_eq!(None.unwrap_or_else(|| 1 + 1), 2);
    }

    #[test]
    fn take_and_replace_tests() {
        let ply1 = Player {
            nickname: String::from("niklas"),
            level: 100,
        };
        let mut niklas: Option<Player> = Some(ply1);
        let p1 = niklas.take(); // take ile niklas'ın kendisi p1'e alınırken niklas değişkenine None değeri atanır
        assert!(niklas.is_none());
        assert_eq!(p1.unwrap().level, 100);

        let mut number = Some(23);
        let origin = number.replace(24); // number'ın değeri Some(24) olarak değişirken origin'e Some(23) atanır
        assert_eq!(origin, Some(23));
        assert_eq!(number, Some(24));
    }

    #[test]
    fn and_or_tests() {
        let n1 = Some(3);
        let n2 = Some(5);
        let n3 = Some(7);

        // and ve or fonksiyonlarının ilginç kullanımları
        assert_eq!(n1.and(n2).and(n3), Some(7));
        assert_eq!(n3.and(Option::<i32>::None), None);
        assert_eq!(n1.or(None), Some(3));
        assert_eq!(n2.or(n3), Some(5));
        assert_eq!(None.or(n3), Some(7));
    }

    #[test]
    fn and_then_tests() {
        let nbr = Some(3);
        let n2 = nbr.and_then(|n| Some(n + 1)).and_then(|n| Some(n + 1));
        let mut iter = n2.iter(); // Option değerleri üstünde ileri hareket edilebilir
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), None);
    }
}

pub struct Player {
    nickname: String,
    level: i32,
}

impl Player {
    pub fn writeline(p: &Self) {
        println!("{}{}", p.nickname, p.level);
    }
}
