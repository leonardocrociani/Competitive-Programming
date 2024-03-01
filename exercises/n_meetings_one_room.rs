pub fn n_meetings_one_room(start: &Vec<u32>, end: &Vec<u32>) -> u32 {
    let mut pairs: Vec<(u32, u32)> = start.iter().zip(end.iter()).map(|(&s, &e)| (s, e)).collect();
    
    pairs.sort_by(|a, b| a.1.cmp(&b.1));

    let mut counter = 1;
    let mut last_end = pairs[0].1;

    for pair in pairs {
        if pair.0 > last_end { 
            counter += 1;
            last_end = pair.1;
        }
    }

    counter
}
