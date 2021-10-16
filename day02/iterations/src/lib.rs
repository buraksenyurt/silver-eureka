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
}
