use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;

pub fn sort<T: Ord>(s: &mut [T]) {
    if !s.is_empty() {
        //temp should be dropped safely even it panics on comparsion.
        let mut tmp: DropGuard<_, TempVecDropHandler<T>> =
            DropGuard::new(Vec::with_capacity(s.len()));
        mergesort_recursive(s, &mut tmp, 0, s.len() - 1);
    }
}

//drop handler to drop temp vector safely without double dropping its elements.
struct TempVecDropHandler<T> {
    _phantom: PhantomData<T>,
}

impl<T> DropHandler<Vec<T>> for TempVecDropHandler<T> {
    fn on_drop(this: &mut Vec<T>) {
        unsafe {
            //SAFETY: setting vector length to zero is safe. It avoids double drop of its elements.
            this.set_len(0);
        };
    }
}

fn mergesort_recursive<T: Ord, H: DropHandler<Vec<T>>>(
    s: &mut [T],
    tmp: &mut DropGuard<Vec<T>, H>,
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
fn merge<T: Ord, H: DropHandler<Vec<T>>>(
    s: &mut [T],
    tmp: &mut DropGuard<Vec<T>, H>,
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

//drop handler is baked into type system. It does not take space to store handler.
#[repr(transparent)]
struct DropGuard<T, H: DropHandler<T>> {
    inner: T,
    drop_handler: PhantomData<H>, // only used by type system to call handler
}

impl<T, H: DropHandler<T>> DropGuard<T, H> {
    fn new(inner: T) -> Self {
        Self {
            inner,
            drop_handler: PhantomData,
        }
    }
}

trait DropHandler<T> {
    fn on_drop(this: &mut T); //does not takes self or captures environment. can be used by type system at compile time.
}

impl<T, F: DropHandler<T>> Deref for DropGuard<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, F: DropHandler<T>> DerefMut for DropGuard<T, F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T, F: DropHandler<T>> Drop for DropGuard<T, F> {
    fn drop(&mut self) {
        F::on_drop(self); //call drop handler. this is handled by type system.
    }
}
