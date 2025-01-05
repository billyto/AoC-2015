use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01::{parse_input, solve_part1, solve_part2};

fn benchmark_part1(c: &mut Criterion) {
    let input = include_str!("../../inputs/day01.txt");
    let parsed = parse_input(input).unwrap();
    c.bench_function("day 01 - part 1", |b| {
        b.iter(|| solve_part1(black_box(&parsed)))
    });
}

fn benchmark_part2(c: &mut Criterion) {
    let input = include_str!("../../inputs/day01.txt");
    let parsed = parse_input(input).unwrap();
    c.bench_function("day 01 - part 2", |b| {
        b.iter(|| solve_part2(black_box(&parsed)))
    });
}

// You can also create benchmark groups
fn benchmark_full_solution(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 01 - full solution");
    let input = include_str!("../../inputs/day01.txt");

    group.bench_function("parse + part1", |b| {
        b.iter(|| {
            let parsed = parse_input(black_box(input)).unwrap();
            solve_part1(&parsed)
        })
    });

    group.bench_function("parse + part2", |b| {
        b.iter(|| {
            let parsed = parse_input(black_box(input)).unwrap();
            solve_part2(&parsed)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_part1,
    benchmark_part2,
    benchmark_full_solution
);
criterion_main!(benches);
