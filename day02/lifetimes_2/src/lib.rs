#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Örnekte varyans(değişiklik) hesaplaması ile ilgili enstrümanlar kullanılıyor.
// Üzerinde çalışılacak sayı dizisini tutan bir struct veri modeli söz konusu.
// Önemli olan Struct'a generic lifetime parametresi geçirilmesi ve içerdeki sayı dizisinin de bu yaşam süresine göre değerlendirilmesi gerektiğinin derleyiciye söylenmiş olması.
pub struct Statistician<'a> {
    numbers: &'a [f64],
}

// new ile yeni bir Statistician nesnesi örneklemeyi kolaylaştıran fonksiyonu yazıyoruz.
// parametre olarak gelen sayı dizisinin lifetimes bilgisi ile birlikte geldiğine dikkat edelim.
impl<'a> Statistician<'a> {
    pub fn new(numbers: &'a [f64]) -> Option<Statistician> {
        if numbers.len() < 5 { // Beştan az sayı varsa None değerini döneceğiz
            None
        } else {
            Some(Statistician { numbers: numbers })
        }
    }
}
