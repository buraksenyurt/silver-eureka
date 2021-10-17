#![feature(test)]

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::{black_box, Bencher};

    #[bench] // #3
    fn bench_test_for_clone(runner: &mut Bencher) {
        // clone çağrımı maliyetlidir çünkü string'in birçok kopyası oluşturulur.
        // Bu durumu analiz etmek için rust'ın nightly build'unu kullanıp benchmark modülünden yararlanıyoruz.
        // Terminalden cargo bench şeklinde çalıştırılır.
        
        // Test için büyük bir string oluşturulur
        let word: String = (0..100_000_000).map(|_| 'r').collect(); 
        runner.iter(|| {
            black_box(get_length(word.clone())); // clone fonksiyonunun çağırıldığı bir black_box test çalışıtırılır.
        });
    }

    #[test] // #2
    fn fix_out_of_scope_test() {
        // out_of_scope testindeki hatayı çözmek için clone metodundan yararlanılır
        // Bu durumda word değişkeni bu scope'ta yaşamaya devam edebilir
        let word = "rustician".to_owned();
        assert_eq!(get_length(word.clone()), 9);
        assert_eq!(word.len(), 9);
    }

    #[test] // #1
    fn out_of_scope_test() {
        let word = "rustician".to_owned();
        assert_eq!(get_length(word), 9); // Dikkat!
                                         // assert_eq!(word.len(), 9); // word değişkeni get_length fonksiyonuna aktarıldıktan sonra scope dışında kalır ve bu nedenle bu satırda derleyici hata verir. Açıp hatayı görün
    }
}

pub fn get_length(something: String) -> usize {
    something.len()
}
