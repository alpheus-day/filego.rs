use std::{env, fs, hint::black_box, path::PathBuf, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use filego::{
    check::{Check, CheckResult},
    merge::Merge,
    split::{Split, SplitResult},
};

const CHUNK_SIZE: usize = 1 * 1024 * 1024;
const FILE_NAME: &str = "test.jpg";

struct Configs {
    in_file: PathBuf,
    cache_dir: PathBuf,
    out_dir: PathBuf,
}

fn get_configs() -> Configs {
    let root: PathBuf = env::current_dir().unwrap();

    let out_dir: PathBuf = root.join(".media").join("output");

    Configs {
        in_file: root.join("assets").join(FILE_NAME),
        cache_dir: root.join(".media").join("cache"),
        out_dir: out_dir.clone(),
    }
}

fn pre_split() -> SplitResult {
    let configs: Configs = get_configs();

    Split::new()
        .in_file(&configs.in_file)
        .out_dir(&configs.cache_dir.join("-1"))
        .chunk_size(CHUNK_SIZE)
        .run()
        .unwrap()
}

fn bench_split(c: &mut Criterion) {
    let mut group = c.benchmark_group("Split");
    group.warm_up_time(Duration::from_secs(3));
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(10));

    let configs: Configs = get_configs();

    if configs.out_dir.exists() {
        fs::remove_dir_all(&configs.out_dir).unwrap();
    }

    group.bench_function("FileGo", |b| {
        let mut i: usize = 0;

        b.iter(|| {
            let out_dir: PathBuf = configs.cache_dir.join(i.to_string());

            let result: SplitResult = Split::new()
                .in_file(&configs.in_file)
                .out_dir(out_dir)
                .chunk_size(CHUNK_SIZE)
                .run()
                .unwrap();

            black_box(result);

            i += 1;
        });
    });

    group.finish();
}

fn bench_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("Check");
    group.warm_up_time(Duration::from_secs(3));

    let split: SplitResult = pre_split();
    let configs: Configs = get_configs();

    group.bench_function("FileGo", |b| {
        b.iter(|| {
            let result: CheckResult = Check::new()
                .in_dir(&configs.cache_dir.join("0"))
                .file_size(split.file_size)
                .total_chunks(split.total_chunks)
                .run()
                .unwrap();

            black_box(result);
        });
    });

    group.finish();
}

fn bench_merge(c: &mut Criterion) {
    let mut group = c.benchmark_group("Merge");
    group.warm_up_time(Duration::from_secs(3));
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(10));

    let configs: Configs = get_configs();

    if configs.out_dir.exists() {
        fs::remove_dir_all(&configs.out_dir).unwrap();
    }

    group.bench_function("FileGo", |b| {
        let mut i: usize = 0;

        b.iter(|| {
            let out_file: PathBuf = configs.out_dir.join(format!("{}.jpg", i));

            let result: bool = Merge::new()
                .in_dir(&configs.cache_dir.join("0"))
                .out_file(out_file)
                .run()
                .unwrap();

            black_box(result);

            i += 1;
        });
    });

    group.finish();
}

criterion_group!(benches, bench_split, bench_check, bench_merge,);
criterion_main!(benches);
