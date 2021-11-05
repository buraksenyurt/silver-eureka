#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sum_regular_works_test() {
        let result = sum_regular(&[3, 4, 7, 9, 16]);
        assert_eq!(result, 160);
    }

    #[test]
    fn should_sum_parallel_works_test() {
        let result = sum_parallel(&[3, 4, 7, 9, 16]);
        assert_eq!(result, 160);
    }
}
// veriyi paralel işleme üzerine başarılı rust crate'lerinden birisi olan rayon kullanılmakta
use rayon::prelude::*;

// standart bir toplama fonksiyonu
// gelen slice içindeki elemanların karelerini alıp topluyor
pub fn sum_regular(input: &[i32]) -> i32 {
    input.iter().map(|&i| i * i).sum()
}

pub fn sum_parallel(input: &[i32]) -> i32 {
    input.par_iter().map(|&i| i * i).sum() // burada ise rayon ile gelen par_iter fonksiyonu çağırılmakta. Bu iterasyonun paralel ele alınması için kullanılıyor.
}
