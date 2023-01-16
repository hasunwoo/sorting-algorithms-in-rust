use std::mem::ManuallyDrop;
use std::ptr;

pub fn sort<T: Ord>(s: &mut [T]) {
    if !s.is_empty() {
        let mut tmp = ManuallyDrop::new(Vec::new());
        mergesort_recursive(s, &mut tmp, 0, s.len() - 1);
        //code below will not execute when panic. memory leak will happen when panic happens.
        unsafe {
            //inner content of tmp should not be dropped.
            tmp.set_len(0);
            //drop tmp manually.
            ManuallyDrop::drop(&mut tmp);
        }
    }
}

fn mergesort_recursive<T: Ord>(
    s: &mut [T],
    tmp: &mut ManuallyDrop<Vec<T>>,
    start: usize,
    end: usize,
) {
    if end.saturating_sub(start) == 0 {
        return;
    }
    let mid = (start + end) / 2;
    mergesort_recursive(s, tmp, start, mid);
    mergesort_recursive(s, tmp, mid + 1, end);
    merge(s, tmp, start, mid, end);
}

#[inline(always)]
fn merge<T: Ord>(
    s: &mut [T],
    tmp: &mut ManuallyDrop<Vec<T>>,
    start: usize,
    mid: usize,
    end: usize,
) {
    assert!(start <= mid && mid < end);

    unsafe {
        //content of tmp array should not be dropped.
        tmp.set_len(0);
    }

    let mut i = start;
    let mut j = mid + 1;

    while i <= mid && j <= end {
        //It may panic while comparing.
        //It is safe to painc while comparing. Double drop should't happen since tmp vector is ManuallyDrop.
        if s[i] < s[j] {
            tmp.push(unsafe { ptr::read(&s[i] as *const T) });
            i += 1;
        } else {
            tmp.push(unsafe { ptr::read(&s[j] as *const T) });
            j += 1;
        }
    }

    while i <= mid {
        tmp.push(unsafe { ptr::read(&s[i] as *const T) });
        i += 1;
    }

    while j <= end {
        tmp.push(unsafe { ptr::read(&s[j] as *const T) });
        j += 1;
    }

    //check if length of tmp is equal to input array for safety.
    assert_eq!(tmp.len(), end - start + 1);

    //copy tmp array to original array.
    unsafe {
        ptr::copy_nonoverlapping(tmp.as_mut_ptr(), s.as_mut_ptr().add(start), tmp.len());
    }
}
