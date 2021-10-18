#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use std::borrow::Cow;
    use std::cell::{Cell, RefCell};

    #[bench] // #1 Klasik yöntemle vector içeriğini ddeğiştirme testi ölçümleri için
    fn mutation_with_regular_bench(runner: &mut Bencher) {
        // mutable tipte bir vector verisine çok sayıda 1 ekleniyor.
        let mut data = vec![];
        runner.iter(|| {
            for _ in 0..1_000_000 {
                data.push(1);
            }
        });
    }

    #[bench] // #2 RefCell ile vector içeriğini değiştirme testi ölçümleri için
    fn mutation_with_refcell_bench(runner: &mut Bencher) {
        // vector için bir RefCell referansı oluşturuyoruz
        let data_ref = RefCell::new(vec![]);
        runner.iter(|| {
            for _ in 0..1_000_000 {
                data_ref.borrow_mut().push(1); //borrow_mut ile RecCell'in sahiplendiği vecktörü ödünç alıyor ve 1 ekliyoruz
            }
        });
    }

    #[bench] // #3 Cell ile vector içeriğini değiştirme testi ölçümleri için
    fn mutation_with_cell_bench(runner: &mut Bencher) {
        // vector için bir Cell nesnesi oluşturuyoruz. Cell değiştirilebilir bellek lokasyonunu taşıyor.
        let data_ref = Cell::new(vec![]);
        runner.iter(|| {
            for _ in 0..1_000_000 {
                let mut data = data_ref.take(); // mutable formata çevir
                data.push(1); // mutable örneğe veri ekle
                data_ref.set(data); // referansı tekrardan güncelle
            }
        });
    }

    #[bench] // #4 Clone-on-Write(Cow) kullanımı ile test ölçümleri için
    fn mutation_with_cow_bench(runner: &mut Bencher) {
        // Cow tipi yazma üzerine klonlama işlevi sağlayan bir akıllı işaretçidir.
        // Ödünç alınan verileri taşıyıp değiştirilemez(immutable) erişim sağlayabilir.
        // Eğer bir mutasyon veya sahiplik gerekiyorsa verileri klonlar(lazy clonning).
        let mut data_ref = Cow::from(vec![]);
        runner.iter(|| {
            for _ in 0..1_000_000 {
                data_ref.to_mut().push(1);
            }
        });
    }

    // Aşağıdaki fonksiyonlar sembolik olarak gelen vector sonuna bir değer eklemektedir.
    // Bunu vector'lerin olduğu yerde de yapabilirdik ancak örneklerin amacı değiştirilebilir veri sahipliğinin farklı yollarla ele alınması.
    // Bu amaçla change_rc fonksiyonunda RefCell,
    // change_c fonksiyonunda Cell
    // change_cow fonksiyonunda Cow kullanımları yer alıyor.
    fn change_rc(something: i32, data: &RefCell<Vec<i32>>) {
        data.borrow_mut().push(something);
    }

    fn change_c(something: i32, data: &Cell<Vec<i32>>) {
        let mut values = data.take();
        values.push(something);
        data.set(values);
    }

    #[test]
    fn refcell_test() {
        let data_cell = RefCell::new(vec![1, 2, 3, 4]);
        change_rc(5, &data_cell);
        assert!(data_cell.borrow().eq(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn cell_test() {
        let data_cell = Cell::from(vec![1, 2, 3, 4]);
        change_c(7, &data_cell);
        assert_eq!(data_cell.into_inner(), vec![1, 2, 3, 4, 7]);
    }
}
