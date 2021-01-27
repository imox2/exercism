pub fn is_prime(n:u32) -> bool {
    let high_ceil = (n as f64).sqrt() as u32;
    let mut factors = 0;
    for num in 2..=high_ceil {
      if n % num == 0 {
          factors += 1;
          break;
      }  
    }
    factors == 0
}

pub fn nth(n: u32) -> u32 {
    let mut num = 2;
    let mut _nth = n;
    if n != 0 {
        while _nth != 0 {
            num += 1;
            if is_prime(num) {
                _nth -= 1;
            }
        }
    }
    num
}
