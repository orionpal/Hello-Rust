pub fn iterate_for_loop(vec: &[i32]) -> i32 {
    let mut sum = 0;
    for &item in vec.iter() {
        sum += item;
    }
    sum
}

pub fn iterate_iter(vec: &[i32]) -> i32 {
    vec.iter().copied().sum()
}