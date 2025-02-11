use std::time::{Duration, Instant};

use criterion::{
  black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion, Throughput,
};
use linkedlist_vs_vec::{generate, remove_ith_list, remove_ith_vec};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

const SIZES: [usize; 3] = [100, 1_000, 10_000];

pub fn criterion_benchmark_middle(c: &mut Criterion) {
  let mut group = c.benchmark_group("Remove middle element");

  for size in SIZES {
    let _ = group.throughput(Throughput::Bytes(size as u64));
    let (list, vec) = generate(size);

    let _ = group.bench_with_input(BenchmarkId::new("list", size), &size, |b, _range| {
      b.iter_batched(
        || list.clone(),
        |list| remove_ith_list(black_box(list), size / 2),
        BatchSize::SmallInput,
      );
    });

    let _ = group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, _range| {
      b.iter_batched(
        || vec.clone(),
        |vec| remove_ith_vec(black_box(vec), size / 2),
        BatchSize::SmallInput,
      );
    });
  }
  group.finish();
}

pub fn criterion_benchmark_first(c: &mut Criterion) {
  let mut group = c.benchmark_group("Remove first element");

  for size in SIZES {
    let _ = group.throughput(Throughput::Bytes(size as u64));
    let (list, vec) = generate(size);

    let _ = group.bench_with_input(BenchmarkId::new("list", size), &size, |b, _range| {
      b.iter_batched(
        || list.clone(),
        |list| remove_ith_list(black_box(list), 0),
        BatchSize::SmallInput,
      );
    });

    let _ = group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, _range| {
      b.iter_batched(
        || vec.clone(),
        |vec| remove_ith_vec(black_box(vec), 0),
        BatchSize::SmallInput,
      );
    });
  }
  group.finish();
}

pub fn criterion_benchmark_q1(c: &mut Criterion) {
  let mut group = c.benchmark_group("Remove size/4th element");

  for size in SIZES {
    let _ = group.throughput(Throughput::Bytes(size as u64));
    let (list, vec) = generate(size);

    let _ = group.bench_with_input(BenchmarkId::new("list", size), &size, |b, _range| {
      b.iter_batched(
        || list.clone(),
        |list| remove_ith_list(black_box(list), size / 4),
        BatchSize::SmallInput,
      );
    });

    let _ = group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, _range| {
      b.iter_batched(
        || vec.clone(),
        |vec| remove_ith_vec(black_box(vec), size / 4),
        BatchSize::SmallInput,
      );
    });
  }
  group.finish();
}

pub fn criterion_benchmark_q3(c: &mut Criterion) {
  let mut group = c.benchmark_group("Remove 3*(size/4)th element");

  for size in SIZES {
    let _ = group.throughput(Throughput::Bytes(size as u64));
    let (list, vec) = generate(size);

    let _ = group.bench_with_input(BenchmarkId::new("list", size), &size, |b, _range| {
      b.iter_batched(
        || list.clone(),
        |list| remove_ith_list(black_box(list), (size / 4) * 3),
        BatchSize::SmallInput,
      );
    });

    let _ = group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, _range| {
      b.iter_batched(
        || vec.clone(),
        |vec| remove_ith_vec(black_box(vec), (size / 4) * 3),
        BatchSize::SmallInput,
      );
    });
  }
  group.finish();
}

pub fn criterion_benchmark_rand(c: &mut Criterion) {
  let mut group = c.benchmark_group("Remove random element");

  let mut rng = ChaCha8Rng::seed_from_u64(42);

  for size in SIZES {
    let _ = group.throughput(Throughput::Bytes(size as u64));
    let (list, vec) = generate(size);

    let _ = group.bench_with_input(BenchmarkId::new("list", size), &size, |b, _range| {
      b.iter_batched(
        || list.clone(),
        |list| remove_ith_list(black_box(list), rng.random_range(0..size)),
        BatchSize::SmallInput,
      );
    });

    let _ = group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, _range| {
      b.iter_batched(
        || vec.clone(),
        |vec| remove_ith_vec(black_box(vec), rng.random_range(0..size)),
        BatchSize::SmallInput,
      );
    });
  }
  group.finish();
}

pub fn criterion_benchmark_remove_half(c: &mut Criterion) {
  let mut group = c.benchmark_group("Remove half the elements");

  let mut rng = ChaCha8Rng::seed_from_u64(42);

  for size in SIZES {
    let _ = group.throughput(Throughput::Bytes(size as u64));
    let (list, vec) = generate(size);
    let num_to_remove = size / 2;

    let _ = group.bench_with_input(BenchmarkId::new("list", size), &size, |b, _range| {
      b.iter_custom(|iters| {
        let start = Instant::now();
        let mut cloning = Duration::new(0, 0);

        for _ in 0..iters {
          let clone_start = Instant::now();
          let mut list = list.clone();
          cloning += clone_start.elapsed();

          for i in 0..num_to_remove {
            list = remove_ith_list(black_box(list), rng.random_range(0..size - i));
          }
        }

        start.elapsed() - cloning
      });
    });

    let _ = group.bench_with_input(BenchmarkId::new("vec", size), &size, |b, _range| {
      b.iter_custom(|iters| {
        let start = Instant::now();
        let mut cloning = Duration::new(0, 0);

        for _ in 0..iters {
          let clone_start = Instant::now();
          let mut vec = vec.clone();
          cloning += clone_start.elapsed();

          for i in 0..num_to_remove {
            vec = remove_ith_vec(black_box(vec), rng.random_range(0..size - i));
          }
        }

        start.elapsed() - cloning
      });
    });
  }
  group.finish();
}

criterion_group!(
  benches,
  criterion_benchmark_first,
  criterion_benchmark_middle,
  criterion_benchmark_q1,
  criterion_benchmark_q3,
  criterion_benchmark_rand,
  criterion_benchmark_remove_half,
);
criterion_main!(benches);
