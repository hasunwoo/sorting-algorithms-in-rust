pub fn sort<T: Ord>(s: &mut [T]) {
    for i in 0..s.len() {
        for j in 0..s.len() - i - 1 {
            if s[j] > s[j + 1] {
                s.swap(j, j + 1);
            }
        }
    }
}
