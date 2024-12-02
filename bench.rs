use criterion::{criterion_group, criterion_main, BatchSize, Criterion};

use advent::day1::{gen_input, part1, part2};

fn bench_part1(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("part1", |b| {
        b.iter_batched(|| gen_input(&mut rng), |s| part1(&s), BatchSize::LargeInput)
    });
}

fn bench_part2(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    c.bench_function("part2", |b| {
        b.iter_batched(|| gen_input(&mut rng), |s| part2(&s), BatchSize::LargeInput)
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
