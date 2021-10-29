use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Fibonacci sayısını bulan bir fonksiyon için benchmark ölçümlemesi yapacağız
// Sayıların bir kısmını hatırlayalım. F(n) = F(n-1) + F(n-2) ; F(0)=0, F(1)=1
// 0	1	1	2	3	5	8	13	21	34	55	89	144	233	377	610	987	1597	2584	4181	6765
fn fibonacci(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Fibonacci sayısı bulma için performans testi", |b| {
        b.iter(|| fibonacci(black_box(13)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
