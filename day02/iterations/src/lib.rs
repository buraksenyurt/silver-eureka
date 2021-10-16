#[cfg(test)]
mod tests {
    #[test]
    fn simple_iter_test() {
        let values = vec![1, 2, 3];
        let mut iterator = values.iter();
        assert_eq!(iterator.next(), Some(&1)); // next ile vecktörün sıradaki elemanı çekilirken pointer sonraki konuma ilerler.
        assert_eq!(iterator.next(), Some(&2));
        assert_eq!(iterator.next(), Some(&3));

        let mut total = 0;
        for v in values {
            total += v;
        }
        assert_eq!(total, 6);
    }

    #[test]
    fn collect_test() {
        let values = vec![10, 11, 5, 15, 0];
        let one_more = values.iter().map(|i| i + 1); // map iterasyon sırasında gelen her eleman için bir işlem uygular. Burada values'un her bir elemanını 1 artırıyor
        assert_eq!(one_more.collect::<Vec<i32>>(), vec![11, 12, 6, 16, 1]); // collect ile one_more sonuçlarını yeni bir vector nesnesinde toplayabiliriz
        let hex_version = values.iter().map(|i| format!("{:x}", i)); // Bu sefer her bir elemanın hexadecimal sayı sistemindeki karşılığı alınıyor
        assert_eq!(
            hex_version.collect::<Vec<String>>(), // Elde edilen hexadecimal değerler String türden bir vector nesnesine alınıyor
            vec![
                "a".to_string(),
                "b".to_string(),
                "5".to_string(),
                "f".to_string(),
                "0".to_string()
            ]
        );
    }

    #[test]
    fn zip_test() {
        let words = vec![
            "shall".to_string(),
            "we".to_string(),
            "began".to_string(),
            "!".to_string(),
        ];
        let word_counter = words.iter().map(get_letter_count);

        // words vektöründeki herbir kelime için get_letter_count çağrılır.
        // zip iki iterasyonda eş zamanlı ilerlenmesini sağlar.
        // Yani words üstündeki ilerlerken paralelde her word için word_counter iterasyonunda da hareket eder.
        // Sonuçlar collect ile (&String,usize) tuple'larından oluşan bir vector'e alınır
        let words_report: Vec<(&String, usize)> = words.iter().zip(word_counter).collect();
        assert_eq!(
            words_report,
            vec![
                (&"shall".to_string(), 5),
                (&"we".to_string(), 2),
                (&"began".to_string(), 5),
                (&"!".to_string(), 1)
            ]
        )
    }

    #[test]
    fn fold_test() {
        // Aggregation fonksiyonlarından olan fold ile iterasyondaki elemanların herbirine bir işlem uygulanır ve bunun sonucu elde edilir.
        let points = vec![3, 5, 7, 9, 11, 13];
        let count = points.iter().fold(0, |point, count| point + count); // İlk parametredeki 0, fold işlem sonucunun ilk değeridir
        assert_eq!(count, 48);
        assert_eq!(points.iter().fold(0, |point, count| point * count), 0); // Dolayısıyla tüm değerleri birbiriyle çarpsak bile başlangıç sonucu 0 olduğundan sonuç 0 çıkar
        assert_eq!(points.iter().fold(1, |point, count| point * count), 135135);
    }

    use super::*;

    #[test]
    fn fiter_with_struct_test() {
        let players = vec![
            Player {
                nickname: "barbarozsa".to_string(),
                level: 5,
            },
            Player {
                nickname: "rougue".to_string(),
                level: 8,
            },
            Player {
                nickname: "jorgensen".to_string(),
                level: 3,
            },
            Player {
                nickname: "klaussen".to_string(),
                level: 6,
            },
        ];
        // filter fonksiyonu ile iterasyondaki her bir eleman için sonucu bool olan bir başka fonksiyon(predicate) işletilir.
        let weaks = players.iter().filter(|p| p.level <= 5); // level değeri 5ten küçük ve eşit olanlar
        assert_eq!(weaks.count(), 2);

        let increased = players
            .iter()
            .filter(|p| p.level <= 5) // level değeri 5ten küçük ve eşit olanların
            .map(|p| p.nickname.to_string()) // isimlerini
            .collect::<Vec<String>>(); // topladık
        assert_eq!(
            increased,
            vec!["barbarozsa".to_string(), "jorgensen".to_string()]
        );
    }

    #[test]
    fn simple_filtering_test() {
        let numbers = vec![1, 5, 10, 15, 4, 3, 28, 15, 55, 9, 60];
        let count = numbers.iter().filter(|&n| n % 5 == 0).count(); // numbers ardışılında 5 ile bölünebilenlerin sayısı
        assert_eq!(count, 6);

        assert_eq!(numbers.iter().find(|&&n| n == 23), None); // sayılar arasında 23 var mı?
        assert_eq!(numbers.iter().find(|&&n| n == 15), Some(&15)); // sayılar arasında 15 var mı?

        let mut sub_numbers = numbers.iter().skip(3).take(5); // buda bir veri filtresi olarak düşünülebilir. numbers ardışılında ilk üç elemanı atlayıp kalan beş eleman alınır
        assert_eq!(sub_numbers.next(), Some(&15));
        assert_eq!(sub_numbers.next(), Some(&4));
        assert_eq!(sub_numbers.next(), Some(&3));
        assert_eq!(sub_numbers.next(), Some(&28));
        assert_eq!(sub_numbers.next(), Some(&15));
        assert_eq!(sub_numbers.next(), None);
    }

    // map fonksiyonundan çağrılan fonksiyon
    fn get_letter_count(word: &String) -> usize {
        word.len()
    }
}

pub struct Player {
    pub nickname: String,
    pub level: usize,
}
