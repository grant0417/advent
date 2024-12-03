use criterion::{black_box, criterion_group, criterion_main, Criterion};

use advent::day2::{part1, part2};

const INPUT: &str = include_str!("../data/2024/day2.txt");

fn bench_part1(c: &mut Criterion) {
    c.bench_function("part1", |b| b.iter(|| black_box(part1(black_box(INPUT)))));
}

fn bench_part2(c: &mut Criterion) {
    c.bench_function("part2", |b| b.iter(|| black_box(part2(black_box(INPUT)))));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
