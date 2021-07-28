// cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn factorial(num: u32) -> u32 {
    match num {
        0 => 1,
        1 => 1,
        _ => factorial(num - 1) * num,
    }
}

fn bench_fact(c: &mut Criterion) {
    c.bench_function("fact 20", |b| b.iter(|| factorial(black_box(200))));
}

criterion_group!(benches, bench_fact);
criterion_main!(benches);
