use criterion::{black_box, criterion_group, criterion_main, Criterion, PlotConfiguration};
use runner::jobs;

fn criterion_benchmark(c: &mut Criterion) {
    let all_solutions = jobs();

    c.bench_function("Total: All Days", |b| {
        b.iter(|| {
            for (solution, _, input) in all_solutions {
                solution(black_box(input));
            }
        })
    });

    let mut group = c.benchmark_group("Individual");
    group
        .plot_config(PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic));

    for (solution, name, input) in all_solutions {
        group.bench_function(*name, |b| b.iter(|| solution(black_box(input))));
    }

    group.finish();
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_benchmark
);
criterion_main!(benches);
