pub fn sort<T: Ord>(s: &mut [T]) {
    if s.len() == 0 {
        return;
    }
    for i in 0..s.len() - 1 {
        let mut min = i;
        for j in i + 1..s.len() {
            if s[j] < s[min] {
                min = j;
            }
        }
        s.swap(i, min);
    }
}
