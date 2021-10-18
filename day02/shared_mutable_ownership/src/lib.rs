#![feature(test)]

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use std::borrow::Cow;
    use std::cell::{Cell, RefCell};

    #[bench] // #1 Klasik yöntemle vector içeriğini değiştirmek
    fn regular_mutation_test(runner: &mut Bencher) {
        // mutable tipte bir vector verisine çok sayıda 1 ekleniyor.
        let mut data = vec![];
        runner.iter(|| {
            for _ in 0..1_000_000 {
                data.push(1);
            }
        });
    }
}
