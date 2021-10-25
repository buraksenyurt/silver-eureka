use std::fmt::Display;
mod data;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_complex_numbers_works_test() {
        let number_1 = Complex { x: 1, y: -5 }; // tam sayı türü ile kullanıyoruz
        assert_eq!(number_1.real_root(), &1);
        assert_eq!(number_1.virtual_root(), &-5);
        assert_eq!(number_1.to_str(), "1+(-5)i");

        let number_2 = Complex { x: -1.5, y: -5.89 }; // float sayı türü ile kullanıyoruz
        assert_eq!(number_2.real_root(), &-1.5);
        assert_eq!(number_2.virtual_root(), &-5.89);
        assert_eq!(number_2.to_str(), "-1.5+(-5.89)i");

        let number_2 = Complex { x: -1.5, y: -5.89 }; // float sayı türü ile kullanıyoruz
        assert_eq!(number_2.real_root(), &-1.5);
        assert_eq!(number_2.virtual_root(), &-5.89);
        assert_eq!(number_2.to_str(), "-1.5+(-5.89)i");

        let number_3 = Complex2 {
            x: 9_i8,
            y: 923.9394_f64,
        }; // <T,K> durumuna uygun. T için 8 bir işaretli tam sayı, y için 64 bit float türü.
        assert_eq!(number_3.real_root(), &9_i8);
        assert_eq!(number_3.virtual_root(), &923.9394_f64);
        assert_eq!(number_3.to_str(), "9+(923.9394)i");
    }
}

// #1
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
    // gerçel kökü döner
    pub fn real_root(&self) -> &T {
        &self.x
    }
    // sanal kökü döner
    pub fn virtual_root(&self) -> &T {
        &self.y
    }
    // x+(-)yi gibi kompleks sayı formunda String olarak veri döner
    pub fn to_str(&self) -> String {
        format!("{}+({})i", self.x, self.y)
    }
}

// #2
// Birden fazla generic tipi de kullanabiliriz
// Örneğin Complex2 türünde x T türünden y farklı olarak K türünden olabilir
pub struct Complex2<T, K> {
    x: T,
    y: K,
}

// Üstteki kullanıma göre implementasyon şekli değişir. Hem T, hem K için ele alınmalıdır
impl<T, K> Complex2<T, K>
where
    T: Display,
    K: Display,
{
    pub fn real_root(&self) -> &T {
        &self.x
    }
    pub fn virtual_root(&self) -> &K {
        &self.y
    }
    pub fn to_str(&self) -> String {
        format!("{}+({})i", self.x, self.y)
    }
}