// use std::collections::HashMap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // let mut rain_sound_options = HashMap::new();
    let mut sum = 0;
    for n in 1..limit {
        for factor in factors {
            if *factor !=0 && n % factor == 0 {
                sum += n;
                break;
            }
        }
    }
    sum
}
