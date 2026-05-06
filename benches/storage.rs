use std::hint::black_box;

use chrono::{Local, TimeZone};
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use funpou::{memo::Memo, storage};
use tempfile::NamedTempFile;

fn memo_at(sequence: usize) -> Memo {
    let created_at = Local
        .with_ymd_and_hms(2026, 3, 20, 14, 5, (sequence % 60) as u32)
        .unwrap();

    Memo {
        id: created_at.format("%Y%m%d%H%M%S").to_string(),
        body: format!("benchmark memo {sequence:04}"),
        created_at,
    }
}

fn bench_append_memo(c: &mut Criterion) {
    let memo = memo_at(0);

    c.bench_function("append_memo", |b| {
        b.iter_batched(
            || NamedTempFile::new().unwrap(),
            |file| storage::append_memo(file.path(), black_box(&memo)).unwrap(),
            BatchSize::SmallInput,
        );
    });
}

fn bench_read_all(c: &mut Criterion) {
    let file = NamedTempFile::new().unwrap();
    for sequence in 0..1_000 {
        storage::append_memo(file.path(), &memo_at(sequence)).unwrap();
    }

    c.bench_function("read_all_1000_memos", |b| {
        b.iter(|| {
            let memos = storage::read_all(file.path()).unwrap();
            black_box(memos);
        });
    });
}

fn bench_format_display(c: &mut Criterion) {
    let memo = memo_at(0);

    c.bench_function("format_display", |b| {
        b.iter(|| {
            black_box(memo.format_display(black_box("%Y-%m-%d %H:%M")));
        });
    });
}

criterion_group!(
    storage_benches,
    bench_append_memo,
    bench_read_all,
    bench_format_display
);
criterion_main!(storage_benches);
