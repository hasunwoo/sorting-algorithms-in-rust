use std::mem;

//it would be nice to add speclized version of sort function(T: Copy, Clone).
//but it is not supported in current version of rust
pub fn sort<T: Ord + Default>(s: &mut [T]) {
    if s.len() > 0 {
        let mut tmp: Vec<T> = Vec::with_capacity(s.len());
        mergesort_recursive(s, &mut tmp, 0, s.len() - 1);
    }
}

fn mergesort_recursive<T: Ord + Default>(s: &mut [T], tmp: &mut Vec<T>, start: usize, end: usize) {
    if end.saturating_sub(start) <= 0 {
        return;
    }
    let mid = (start + end) / 2;
    mergesort_recursive(s, tmp, start, mid);
    mergesort_recursive(s, tmp, mid + 1, end);
    safe_merge(s, tmp, start, mid, end);
}

fn safe_merge<T: Ord + Default>(
    s: &mut [T],
    tmp: &mut Vec<T>,
    start: usize,
    mid: usize,
    end: usize,
) {
    assert!(start <= mid && mid + 1 <= end);
    tmp.clear();
    let mut i = start;
    let mut j = mid + 1;
    while i <= mid && j <= end {
        if s[i] < s[j] {
            tmp.push(mem::take(&mut s[i]));
            i += 1;
        } else {
            tmp.push(mem::take(&mut s[j]));
            j += 1;
        }
    }
    while i <= mid {
        tmp.push(mem::take(&mut s[i]));
        i += 1;
    }
    while j <= end {
        tmp.push(mem::take(&mut s[j]));
        j += 1;
    }
    for i in 0..tmp.len() {
        s[end - i] = tmp.pop().unwrap();
    }
}
