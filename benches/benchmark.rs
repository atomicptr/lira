use lira::prelude::*;

use criterion::{Criterion, criterion_group, criterion_main};

fn render_table_grid(size: usize) -> String {
    table()
        .child(tbody().children(1..=size, |row| {
            tr().map_when(row % 2 == 0, |n| n.class("even"))
                .children(1..=size, |col| td().text(format!("{}x{}", row, col)))
        }))
        .into()
}

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lira Benchmarks");

    group.measurement_time(std::time::Duration::from_secs(5));
    group.warm_up_time(std::time::Duration::from_secs(1));
    group.sample_size(200);
    group.noise_threshold(0.02);
    group.confidence_level(0.95);
    group.significance_level(0.05);

    group.bench_function("render table grid: 100x100", |b| {
        b.iter(|| render_table_grid(100))
    });
    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
