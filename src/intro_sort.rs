pub fn sort<T: Ord>(s: &mut [T]) {
    if !s.is_empty() {
        let depth_limit = 2 * ((s.len() as f32).log2() as usize);
        intro_sort(s, 0, s.len() - 1, depth_limit);
    }
}

#[inline(always)]
fn intro_sort<T: Ord>(s: &mut [T], start: usize, end: usize, depth_limit: usize) {
    let size = end.checked_sub(start).map(|s| s + 1);
    match size {
        //has negative size or single item
        None | Some(1) => {}
        //size less than 16
        Some(size) if size < 16 => {
            insertion_sort(&mut s[start..=end]);
        }
        //size grather than 16
        Some(_) => {
            if depth_limit == 0 {
                heap_sort::sort(&mut s[start..=end]);
                return;
            }
            let pivot = partition(&mut s[start..=end]) + start;
            intro_sort(
                s,
                start,
                pivot.saturating_sub(1),
                depth_limit.saturating_sub(1),
            );
            intro_sort(s, pivot + 1, end, depth_limit.saturating_sub(1));
        }
    }
}

//this function will panic if s.len() < 1.
#[inline(always)]
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

#[inline(always)]
fn insertion_sort<T: Ord>(s: &mut [T]) {
    for i in 1..s.len() {
        let mut j = i;
        while j > 0 && s[j - 1] > s[j] {
            s.swap(j - 1, j);
            j -= 1;
        }
    }
}

mod heap_sort {
    #[inline(always)]
    pub fn sort<T: Ord>(s: &mut [T]) {
        if s.len() <= 1 {
            return;
        }
        build_max_heap(s);
        for i in (1..=s.len() - 1).rev() {
            s.swap(0, i);
            max_heapify_down(&mut s[0..i], 0);
        }
    }

    #[inline(always)]
    fn get_children<T>(heap: &[T], node: usize) -> (Option<usize>, Option<usize>) {
        let left_child = 2 * node + 1;
        let right_child = 2 * node + 2;
        (
            (left_child < heap.len()).then_some(left_child),
            (right_child < heap.len()).then_some(right_child),
        )
    }

    #[inline(always)]
    fn max_heapify_down<T: Ord>(heap: &mut [T], node: usize) {
        let mut current_node = node;
        loop {
            let max_child = match get_children(heap, current_node) {
                (Some(left_child), None) => left_child,
                (Some(left_child), Some(right_child)) => {
                    if heap[left_child] <= heap[right_child] {
                        right_child
                    } else {
                        left_child
                    }
                }
                _ => break,
            };
            if heap[current_node] >= heap[max_child] {
                break;
            }
            heap.swap(current_node, max_child);
            current_node = max_child;
        }
    }

    #[inline(always)]
    fn build_max_heap<T: Ord>(heap: &mut [T]) {
        assert!(heap.len() >= 2);
        for node in (0..=heap.len() / 2 - 1).rev() {
            max_heapify_down(heap, node);
        }
    }
}
