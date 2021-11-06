#[macro_use]
extern crate criterion;
use criterion::{black_box, Criterion};
use parallelism::{qsort_parallel, qsort_regular, sum_parallel, sum_regular};
use rand::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

// İki test için büyük bir sayı kümesi kullanacağız
// Heap üstünde referans edebilecek şekilde oluşturuyoruz. RefCell kullanmamızın sebebi bu.
// Test kümesi thread_local isimli macro tarafından yapılıyor.
thread_local!(static DATA: RefCell<Vec<i32>> = {
    // 100 milyon elemanlı rastgele int değerlerinden oluşan bir dizi alıp refcell ile refere ediyoruz.
    let numbers: Vec<i32> = (0..100_000).map(|_| random::<i32>()).collect();
    RefCell::new(numbers)
});

// NUMBERS kullanımı üstünde çalışmalıyız. Kümeyi büyütünce main thread stack'i şişirip patlatıyor. 
thread_local!(static NUMBERS: RefCell<Vec<i32>> = {
    // 100 milyon elemanlı rastgele int değerlerinden oluşan bir dizi alıp refcell ile refere ediyoruz.
    let numbers: Vec<i32> = (0..10000).map(|_| random::<i32>()).collect();
    RefCell::new(numbers)
});

// normal iter kullanılan fonksiyon için benchmark yürütücü.
fn bench_regular(c: &mut Criterion) {
    c.bench_function("Sum(Regular)", |b| {
        DATA.with(|n| b.iter(|| black_box(sum_regular(&n.borrow()))))
    });
}

// Bu da paralel olan sürümü için
fn bench_parallel(c: &mut Criterion) {
    c.bench_function("Sum(Parallel)", |b| {
        DATA.with(|n| b.iter(|| black_box(sum_parallel(&n.borrow()))))
    });
}

// Standart quick sort için benchmark
fn bench_regular_quicksort(c: &mut Criterion) {
    c.bench_function("QuickSort(Regular)", |b| {
        NUMBERS.with(|n| b.iter(|| black_box(qsort_regular(&mut n.borrow_mut()))))
    });
}

// Join ile paralel çalışan quick sort için benchmark
fn bench_parallel_quicksort(c: &mut Criterion) {
    c.bench_function("QuickSort(Parallel)", |b| {
        NUMBERS.with(|n| b.iter(|| black_box(qsort_parallel(&mut n.borrow_mut()))))
    });
}

criterion_group!(
    benches,
    bench_regular,
    bench_parallel,
    bench_regular_quicksort,
    bench_parallel_quicksort
);
criterion_main!(benches);
