#[macro_use]
extern crate criterion;
use criterion::{black_box, Criterion};
use parallelism::{sum_parallel, sum_regular};
use rand::prelude::*;
use std::cell::RefCell;

// İki test için büyük bir sayı kümesi kullanacağız
// Heap üstünde referans edebilecek şekilde oluşturuyoruz. RefCell kullanmamızın sebebi bu.
// Test kümesi thread_local isimli macro tarafından yapılıyor.
thread_local!(static DATA: RefCell<Vec<i32>> = {
    // 100 milyon elemanlı rastgele int değerlerinden oluşan bir dizi alıp refcell ile refere ediyoruz.
    let numbers: Vec<i32> = (0..100_000_000).map(|_| random::<i32>()).collect(); 
    RefCell::new(numbers)
});

// normal iter kullanılan fonksiyon için benchmark yürütücü.
fn bench_regular(c: &mut Criterion) {
    c.bench_function("Regular", |b| {
        DATA.with(|n| b.iter(|| black_box(sum_regular(&n.borrow()))))
    });
}

// Bu da paralel olan sürümü için
fn bench_parallel(c: &mut Criterion) {
    c.bench_function("Parallel", |b| {
        DATA.with(|n| b.iter(|| black_box(sum_parallel(&n.borrow()))))
    });
}

criterion_group!(benches, bench_regular, bench_parallel);
criterion_main!(benches);
