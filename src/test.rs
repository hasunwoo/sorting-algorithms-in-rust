use crate::make_sorting_test;

make_sorting_test!(bubble_sort, super::bubble_sort::sort);
make_sorting_test!(insertion_sort, super::insertion_sort::sort);
make_sorting_test!(selection_sort, super::selection_sort::sort);
make_sorting_test!(quick_sort, super::quick_sort::sort);
make_sorting_test!(merge_sort, super::merge_sort::sort);
make_sorting_test!(safe_merge_sort, super::safe_merge_sort::sort);

make_sorting_test!(std_sort_unstable, |s| s.sort_unstable());
make_sorting_test!(std_sort, |s| s.sort());

fn test_random_sorting<T: Ord + Clone>(
    sample: &[T],
    sorted_sample: &[T],
    sort_fn: impl FnOnce(&mut [T]),
) -> bool {
    let mut sample: Vec<T> = sample.to_vec();
    sort_fn(&mut sample);
    &sample[..] == sorted_sample
}

#[macro_export]
macro_rules! make_sorting_test {
    ($name:ident, $f:expr) => {
        #[test]
        fn $name() {
            use rand::{Rng, distributions::Standard};
            let mut rng = rand::thread_rng();
            let sample_size = 20;
            for n in 0..sample_size {
                let sample: Vec<isize> = (&mut rng).sample_iter(Standard).take(n).collect();
                let sorted_sample = {
                    let mut tmp = sample.clone();
                    tmp.sort_unstable();
                    tmp
                };
                assert!(test_random_sorting(&sample, &sorted_sample, $f), "error while testing array with size of {n}");
            }
        }
    }
}