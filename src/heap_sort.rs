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
