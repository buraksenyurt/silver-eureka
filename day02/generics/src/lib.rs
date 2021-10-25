use std::fmt::Display;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_complex_numbers_works_test() {
        let number_1 = Complex { x: 1, y: -5 };
        assert_eq!(number_1.real_root(), &1);
        assert_eq!(number_1.virtual_root(), &-5);
        assert_eq!(number_1.to_str(), "1-5i");

        let number_2 = Complex { x: -1.5, y: -5.89 };
        assert_eq!(number_2.real_root(), &-1.5);
        assert_eq!(number_2.virtual_root(), &-5.89);
        assert_eq!(number_2.to_str(), "-1.5-5.89i");
    }
}

// Kendi generic veri yapılarımızı tasarlayabilir.
// Örneğin tamsayı veya noktalı sayılar ile çalışan kompleks sayı için iki ayrı veri türü yazmak yerine
// Aşağıdaki gibi generic olan bir versiyon kullanabiliriz
// x ve y T olarak hangi türü kullanırsak onla çalışır
pub struct Complex<T> {
    x: T,
    y: T,
}

// Üstteki veri modeline gelen generic türü fonksiyonlarında da kullanabiliriz
impl<T> Complex<T>
where
    T: Display, // T türünün Display Trait'ini uygulaması gerekir ki to_str fonksiyonundaki format! makrosunda kullanabilelim.
{
    pub fn real_root(&self) -> &T {
        &self.x
    }
    pub fn virtual_root(&self) -> &T {
        &self.y
    }
    pub fn to_str(&self) -> String {
        format!("{}{}i", self.x, self.y)
    }
}
