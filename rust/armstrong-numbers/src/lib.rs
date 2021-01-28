pub fn is_armstrong_number(num: u32) -> bool {
    let mut cube_sum :u64 = 0;
    let mut num_copy: u64 = u64::from(num);
    let digits_length = num.to_string().len();
    while num_copy > 0 {
        let rem = num_copy % 10;
        cube_sum += rem.pow(digits_length as u32);
        num_copy = num_copy / 10;
    }
    cube_sum == u64::from(num)
}
