pub fn wilbur_and_array (target: Vec<u32>) -> u32 {
    let mut ops = 0;
    let mut cur = 0;
    for i in 0..target.len() {
        ops += ((target[i] as i32 - cur as i32) as i32).abs();
        cur = target[i];
    }
    ops as u32
}
