use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{distributions, Rng};
use sorting_algorithms::{
    bubble_sort, heap_sort, insertion_sort, merge_sort, quick_sort, safe_merge_sort, selection_sort,
};

macro_rules! sorting_benchmark_group {
    (group_name = $name:expr, criterion = $c:expr, rng = $rng:expr, data_size = $data_size:expr, batch_size = $batch_size:expr, algorithms = $(($algo_name:expr, $algo_fn:expr)),+) => {{
        let mut group = $c.benchmark_group($name);
        let data: Vec<isize> = (&mut $rng).sample_iter(distributions::Standard).take($data_size).collect();
        $(group.bench_function($algo_name, |b| {
            b.iter_batched(
                || data.clone(),
                |mut data| $algo_fn(black_box(&mut data)),
                $batch_size,
            );
        });)+
        group.finish();
    }};
}

fn bench_sorting(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    sorting_benchmark_group!(
        group_name = "small_inputs",
        criterion = c,
        rng = rng,
        data_size = 999,
        batch_size = BatchSize::SmallInput,
        algorithms = ("bubble_sort", bubble_sort::sort),
        ("selection_sort", selection_sort::sort),
        ("insertion_sort", insertion_sort::sort),
        ("quick_sort", quick_sort::sort),
        ("merge_sort", merge_sort::sort),
        ("safe_merge_sort", safe_merge_sort::sort),
        ("heap_sort", heap_sort::sort)
    );
    sorting_benchmark_group!(
        group_name = "medium_inputs",
        criterion = c,
        rng = rng,
        data_size = 99999,
        batch_size = BatchSize::SmallInput,
        algorithms = ("quick_sort", quick_sort::sort),
        ("merge_sort", merge_sort::sort),
        ("safe_merge_sort", safe_merge_sort::sort),
        ("heap_sort", heap_sort::sort)
    );

    sorting_benchmark_group!(
        group_name = "large_inputs",
        criterion = c,
        rng = rng,
        data_size = 9999999,
        batch_size = BatchSize::SmallInput,
        algorithms = ("quick_sort", quick_sort::sort),
        ("merge_sort", merge_sort::sort),
        ("safe_merge_sort", safe_merge_sort::sort),
        ("heap_sort", heap_sort::sort)
    );
}

criterion_group!(benches, bench_sorting);
criterion_main!(benches);
