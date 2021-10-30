#[cfg(test)]
mod tests {

    use super::*;
    use std::thread::sleep; // Bir Thread'i belli süre uyutmakta yardımcı olacak
    use std::time::Duration; // Zaman aralığı vermede kullanılacak

    #[test]
    fn sum_of_two_works_test() {
        let result = sum(5, 6);
        assert_eq!(result, 11);
    }

    #[test]
    fn wait_a_few_seconds_test() {
        sleep(Duration::from_secs(5)); // Thread şu andan itibaren 5 saniye boyunca durdurulacak
        println!("Bir süre sonra..."); // Normalde terminalde yapılan cargo test işlemi sonrası println! fonksiyonları görünmez.
        assert_eq!(sum(1, 2), 3);
    }
}

pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}
