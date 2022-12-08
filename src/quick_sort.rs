pub fn sort<T: Ord>(s: &mut [T]) {
    if s.len() > 0 {
        quicksort_recursive(s, 0, s.len() - 1);
    }
}

fn quicksort_recursive<T: Ord>(s: &mut [T], start: usize, end: usize) {
    if end.saturating_sub(start) <= 0 {
        return;
    }
    //offset start position because we partition against subslice of original one.
    let pivot = partition(&mut s[start..=end]) + start;
    quicksort_recursive(s, start, pivot.saturating_sub(1));
    quicksort_recursive(s, pivot + 1, end);
}

//this function will panic if s.len() < 1.
fn partition<T: Ord>(s: &mut [T]) -> usize {
    let mut i = 0; //next place to store value smaller then pivot
    for j in 0..s.len() - 1 {
        //move value to left region if smaller then pivot
        if &s[j] < s.last().unwrap() {
            s.swap(i, j);
            i += 1;
        }
    }
    //move pivot to middle of left and right region
    s.swap(i, s.len() - 1);
    i
}
