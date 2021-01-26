pub fn square(s: u32) -> u64 {
    let base: u64 = 2; // an explicit type is required
    let result: u64;
    if s > 0 && s < 65 {
        if s == 1 {
            result = u64::from(s);
        } else {
            result = base.pow(s - 1);
        }
    } else {
        panic!("Square must be between 1 and 64");
    }

    result
}

pub fn total() -> u64 {
    let mut total: u64 = 0;
    let base: u64 = 2; // an explicit type is required
    for n in 1..65 {
        total += base.pow(n - 1);
    }
    total
}
