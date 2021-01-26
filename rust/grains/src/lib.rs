pub fn square(s: u32) -> u64 {
    assert!((1..=64).contains(&s), "Square must be between 1 and 64");
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    let mut total: u64 = 0;
    for n in 1..65 {
        total += square(n);
    }
    total
}
