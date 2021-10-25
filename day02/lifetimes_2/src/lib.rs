#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_statistician_numbers_in_range_test() {
        assert!(Statistician::new(&vec![]).is_none());
        assert!(Statistician::new(&vec![1.0, 2.0, 3.0, 4.0]).is_none());
        assert!(Statistician::new(&vec![1.0, 2.0, 1.0, 3.0, 2.0, 5.0]).is_some());
    }

    #[test]
    fn should_median_works_test() {
        let values = &vec![1.0, 2.0, 1.0, 3.0, 2.0, 5.0];
        let fisher = Statistician::new(values).unwrap();
        assert_eq!(fisher.median(), 2.0);
    }

    #[test]
    fn should_variance_works_test() {
        let values = &vec![1.0, 2.0, 1.0, 3.0, 2.0, 5.0];
        let fisher = Statistician::new(values).unwrap();
        assert_eq!(fisher.variance(), 1.888888888888889);
    }

    #[test]
    fn should_standart_deviation_works_test() {
        let values = &vec![1.0, 2.0, 1.0, 3.0, 2.0, 5.0];
        let fisher = Statistician::new(values).unwrap();
        assert_eq!(fisher.std_deviation(), 1.3743685418725535);
    }

    #[test]
    fn should_mean_works_test() {
        let values = &vec![1.0, 2.0, 1.0, 3.0, 2.0, 5.0];
        let fisher = Statistician::new(values).unwrap();
        assert_eq!(fisher.mean(), 2.3333333333333335);
    }

    #[test]
    fn should_all_same_values_no_variance_test() {
        let values = &vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let fisher = Statistician::new(values).unwrap();
        assert_eq!(fisher.variance(), 0.0);
        assert_eq!(fisher.std_deviation(), 0.0);
        assert_eq!(fisher.mean(), 1.0);
        assert_eq!(fisher.median(), 1.0);
    }
}

// Örnekte standart sapma ve ortalamaya göre varyans(değişiklik) hesaplaması ile ilgili enstrümanlar kullanılıyor.
// Üzerinde çalışılacak sayı dizisini tutan bir struct veri modeli söz konusu.
// Önemli olan Struct'a generic lifetime parametresi geçirilmesi ve içerdeki sayı dizisinin de bu yaşam süresine göre değerlendirilmesi gerektiğinin derleyiciye söylenmiş olması.
pub struct Statistician<'a> {
    numbers: &'a [f64],
}

// new ile yeni bir Statistician nesnesi örneklemeyi kolaylaştıran fonksiyonu yazıyoruz.
// parametre olarak gelen sayı dizisinin lifetimes bilgisi ile birlikte geldiğine dikkat edelim.
impl<'a> Statistician<'a> {
    pub fn new(numbers: &'a [f64]) -> Option<Statistician> {
        if numbers.len() < 5 {
            // Beştan az sayı varsa None değerini döneceğiz
            None
        } else {
            Some(Statistician { numbers: numbers })
        }
    }
    // orta değer bulma fonksiyonu
    pub fn mean(&self) -> f64 {
        let sum: f64 = self.numbers.iter().sum();
        sum / self.numbers.len() as f64
    }
    // varyans hesaplama fonksiyonu
    pub fn variance(&self) -> f64 {
        let mean = self.mean();
        let total_sum_of_squares: f64 = self.numbers.iter().map(|n| (n - mean).powi(2)).sum();
        return total_sum_of_squares / self.numbers.len() as f64;
    }
    // standart sapma fonksiyonu
    pub fn std_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    // medyan değerini bulan fonksiyon
    pub fn median(&self) -> f64 {
        let mut _numbers = self.numbers.to_vec(); // veri yapısındaki numbers'ın bir klonu oluşturulup vektör dizisi haline getiriliyor
        _numbers.sort_by(|x, y| x.partial_cmp(y).unwrap()); //
        let m = _numbers.len() / 2;
        if _numbers.len() % 2 == 0 {
            _numbers[m]
        } else {
            (_numbers[m] + _numbers[m - 1]) / 2.0
        }
    }
}
