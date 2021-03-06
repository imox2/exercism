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

pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut divident = n;
    let mut result = 1;
    for i in 2..=n {
        if is_prime(i as u32) {
            // if prime
            while divident != 0 && divident % i == 0 {
                divident = divident / i;
                factors.push(i);
                result = result * i;
                if result == n {
                    break;
                }
            }
        }
        if result == n {
            break;
        }
    }
    factors
}
