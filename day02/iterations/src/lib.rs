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

    // map fonksiyonundan çağrılan fonksiyon
    fn get_letter_count(word: &String) -> usize {
        word.len()
    }
}
