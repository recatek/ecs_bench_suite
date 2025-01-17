use criterion::*;
use ecs_bench_suite::*;

fn bench_simple_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_insert");
    group.bench_function("gecs", |b| {
        let mut bench = gecs::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("naive", |b| {
        let mut bench = naive::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion", |b| {
        let mut bench = legion::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("planck_ecs", |b| {
        let mut bench = planck_ecs::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("shipyard", |b| {
        let mut bench = shipyard::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("specs", |b| {
        let mut bench = specs::simple_insert::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_simple_iter(c: &mut Criterion) {
    let mut group = c.benchmark_group("simple_iter");
    group.bench_function("gecs", |b| {
        let mut bench = gecs::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("naive", |b| {
        let mut bench = naive::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion", |b| {
        let mut bench = legion::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion (packed)", |b| {
        let mut bench = legion_packed::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (cd OFF)", |b| {
        let mut bench = bevy::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (cd ON)", |b| {
        let mut bench = bevy::simple_iter_cd::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("planck_ecs", |b| {
        let mut bench = planck_ecs::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("shipyard", |b| {
        let mut bench = shipyard::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("specs", |b| {
        let mut bench = specs::simple_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_frag_iter_bc(c: &mut Criterion) {
    let mut group = c.benchmark_group("fragmented_iter");
    group.bench_function("gecs", |b| {
        let mut bench = gecs::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("naive", |b| {
        let mut bench = naive::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion", |b| {
        let mut bench = legion::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (cd OFF)", |b| {
        let mut bench = bevy::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (cd ON)", |b| {
        let mut bench = bevy::frag_iter_cd::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("planck_ecs", |b| {
        let mut bench = planck_ecs::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("shipyard", |b| {
        let mut bench = shipyard::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("specs", |b| {
        let mut bench = specs::frag_iter::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_schedule(c: &mut Criterion) {
    let mut group = c.benchmark_group("schedule");
    group.bench_function("gecs (manual)", |b| {
        let mut bench = gecs::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("naive", |b| {
        let mut bench = naive::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion", |b| {
        let mut bench = legion::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion (packed)", |b| {
        let mut bench = legion_packed::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (manual, cd OFF)", |b| {
        let mut bench = bevy::schedule_manual::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (parallel, cd OFF)", |b| {
        let mut bench = bevy::schedule_parallel::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (single, cd OFF)", |b| {
        let mut bench = bevy::schedule_single::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (single, cd ON)", |b| {
        let mut bench = bevy::schedule_single_cd::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs (manual)", |b| {
        let mut bench = hecs::schedule_manual::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("planck_ecs", |b| {
        let mut bench = planck_ecs::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("shipyard", |b| {
        let mut bench = shipyard::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("specs", |b| {
        let mut bench = specs::schedule::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_heavy_compute(c: &mut Criterion) {
    let mut group = c.benchmark_group("heavy_compute");
    group.bench_function("gecs (single)", |b| {
        let mut bench = gecs::heavy_compute::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("naive (single)", |b| {
        let mut bench = naive::heavy_compute::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (single, cd OFF)", |b| {
        let mut bench = bevy::heavy_compute_single::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (single, cd ON)", |b| {
        let mut bench = bevy::heavy_compute_single_cd::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs (single)", |b| {
        let mut bench = hecs::heavy_compute_single::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion (parallel)", |b| {
        let mut bench = legion::heavy_compute::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("legion (parallel, packed)", |b| {
        let mut bench = legion_packed::heavy_compute::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy (parallel, cd OFF)", |b| {
        let mut bench = bevy::heavy_compute_parallel::Benchmark::new();
        b.iter(move || bench.run());
    });

    group.bench_function("hecs (parallel)", |b| {
        let mut bench = hecs::heavy_compute_parallel::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("shipyard", |b| {
        let mut bench = shipyard::heavy_compute::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("specs (parallel)", |b| {
        let mut bench = specs::heavy_compute::Benchmark::new();
        b.iter(move || bench.run());
    });
}

fn bench_add_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_remove_component");
    group.bench_function("legion", |b| {
        let mut bench = legion::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("hecs", |b| {
        let mut bench = hecs::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("planck_ecs", |b| {
        let mut bench = planck_ecs::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("shipyard", |b| {
        let mut bench = shipyard::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("specs", |b| {
        let mut bench = specs::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("bevy", |b| {
        let mut bench = bevy::add_remove::Benchmark::new();
        b.iter(move || bench.run());
    });
}

criterion_group!(
    name = benchmarks;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(10));
    targets =
        bench_schedule,
        bench_simple_insert,
        bench_simple_iter,
        bench_frag_iter_bc,
        bench_heavy_compute,
        bench_add_remove,
);
criterion_main!(benchmarks);
