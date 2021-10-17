#![feature(test)]

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    extern crate test;
    use test::{black_box, Bencher};

    #[bench] // #4
    fn bench_test_for_reference_counting(runner: &mut Bencher) {
        let word: String = (0..100_000_000).map(|_| 'r').collect();
        let tenant = Rc::new(word); // word için yeni bir smart pointer oluşturulur. word değerini değil pointer lokasyonunu sahiplenir.
        runner.iter(|| {
            black_box(get_length_with_rc(tenant.clone())); // clone fonksiyonunun çağırıldığı bir black_box test çalışıtırılır.
        });
    }

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

    fn get_length(something: String) -> usize {
        something.len()
    }

    // Fonksiyonda gelen String için sahipliği Reference Counted olarak akıllı bir işaretçiye (smart pointer) devretmekteyiz.
    // Bu durumda bir String için sürekli deep copy yapmak yerine pointer'ın çoğaltılması söz konusudur. Bu performans açısından iyi sonuç verir.
    // #4 numaralı bench testinde kullanılmakta
    fn get_length_with_rc(something: Rc<String>) -> usize {
        something.len()
    }
}
