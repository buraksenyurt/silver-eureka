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
}
