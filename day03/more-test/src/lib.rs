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
    fn wait_3_seconds_test() {
        sleep(Duration::from_secs(3)); // Thread şu andan itibaren 3 saniye boyunca durdurulacak
        println!("3 saniye kadar sonra..."); // Normalde terminalde yapılan cargo test işlemi sonrası println! fonksiyonları görünmez.
        assert_eq!(sum(1, 2), 3);
    }

    #[test]
    fn wait_5_seconds_test() {
        sleep(Duration::from_secs(5)); // Thread şu andan itibaren 5 saniye boyunca durdurulacak
        println!("5 saniye kadar sonra...");
        assert_eq!(sum(1, 2), 3);
    }

    #[test]
    #[ignore] // ilgili test çalıştırılmaz
    fn out_of_test() {
        let conn = get_connection();
        assert_eq!(conn, "data source=localhost;database=Nortwhind");
    }
}

pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn get_connection() -> String {
    "data source=localhost;database=Nortwhind".to_string()
}
