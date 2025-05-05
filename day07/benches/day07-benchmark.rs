use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day07::{parse_input, solve_part1, solve_part2};

fn benchmark_part1(c: &mut Criterion) {
    let input_path = format!("../inputs/{}.txt", env!("CARGO_PKG_NAME"));
    let parsed = parse_input(input_path).unwrap();
    c.bench_function("day 07 - part 1", |b| {
        b.iter(|| solve_part1(black_box(&parsed)))
    });
}

fn benchmark_part2(c: &mut Criterion) {
    let input_path = format!("../inputs/{}.txt", env!("CARGO_PKG_NAME"));
    let parsed = parse_input(input_path).unwrap();
    c.bench_function("day 07 - part 2", |b| {
        b.iter(|| solve_part2(black_box(&parsed)))
    });
}

fn benchmark_full_solution(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 07 - full solution");
    //let input = include_str!("../../inputs/day01.txt");
    let input_path = format!("../inputs/{}.txt", env!("CARGO_PKG_NAME"));
    let input_path = input_path.as_str();

    group.bench_function("parse + part1", |b| {
        b.iter(|| {
            let parsed = parse_input(black_box(input_path.to_string())).unwrap();
            solve_part1(&parsed)
        })
    });

    group.bench_function("parse + part2", |b| {
        b.iter(|| {
            let parsed = parse_input(black_box(input_path.to_string())).unwrap();
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
