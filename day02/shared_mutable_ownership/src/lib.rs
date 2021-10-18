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

    fn change_cow(something: i32, data: &mut Cow<[i32]>) {
        data.to_mut().push(something);
    }

    #[test]
    fn refcell_test() {
        let numbers = vec![1, 2, 3, 4];
        let data_cell = RefCell::new(numbers);
        assert!(data_cell.borrow().eq(&vec![1, 2, 3, 4]));
        change_rc(5, &data_cell);
        assert!(data_cell.borrow().eq(&vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn cell_test() {
        let data_cell = Cell::from(vec![1, 2, 3, 4]);
        change_c(7, &data_cell);
        assert_eq!(data_cell.into_inner(), vec![1, 2, 3, 4, 7]);
    }

    use std::ptr::eq;

    #[test]
    fn cow_test() {
        let data = vec![1, 2, 3, 4];
        let mut data_cow = Cow::from(&data);
        assert!(eq(&data[..], &*data_cow)); // veri içerikleri eşittir
        change_cow(8, &mut data_cow); // data_cow diğer metota referans edilir ve orada içeriği değişir
        assert_eq!(data, vec![1, 2, 3, 4]);
        assert_eq!(data_cow, vec![1, 2, 3, 4, 8]); //data_cow içeriği change_cow içinde değiştirilmişti.
        assert!(!eq(&data[..], &*data_cow)); // change_cow çağrısı sonucu sadece data_cow değişir, data aynı şekilde kalır.
    }

    #[test]
    //#[should_panic] // Test sonucundaki hatayı göstermek için yorum dışı bıraktık.
    fn fail_with_cell_test() {
        // RefCell kullanımlarında borrow check (ödünç alma kontrolü) ihlali varsa bir panic oluşur.
        let some_ref = RefCell::new(vec![1, 2, 3, 4]);
        let _borrowed = some_ref.borrow(); // Sarmalanmış değer değiştirilemez şekilde ödünç veriliyor.
        change_rc(9, &some_ref); // Burada bir sorun yok. some_ref içeriği change_rc içinde değiştirilebilir çünkü aynı anda birden fazla değişmez referans ödünç alınabilir.
        change_rc(10, &some_ref); // Bu satırda ise panic oluşacak çünkü change_rc içinde borrow_mut çağrısı söz konusu ve değişmez referansı değiştirilebilir olarak almaya kalkıyoruz.
                                  // _ borrowed satırını kapatıp testi denersek bir sorun olmadığını ve panic oluşmadığını görebiliriz.
    }
}
