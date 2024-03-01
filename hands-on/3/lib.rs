use std::cmp::{Ordering::*, max};

pub fn plan_holiday (cities:Vec<Vec<u32>>, D:usize) -> u32 {
    
    let n = cities.len();

    let mut opt = vec![
        vec![0; D + 1];
        n + 1
    ];

    for i in 1..=n {

        for d in 1..=D {

            let mut total_attractions = 0;

            for k in 0..D {

                if d as i32 - k as i32 - 1 >= 0 {
        
                    total_attractions += cities[i-1][k];
                    opt[i][d] = max(opt[i][d], total_attractions + opt[i-1][d-k-1]);	
        
                }
            }

            // better pick the previous opt?
            opt[i][d] = max(opt[i][d], opt[i-1][d]);
        }
    }
    
    opt[n][D]
}

// first sort, then do LIS on difficulty (n log n, with what we've seen in class)
pub fn design_course (mut topics: Vec<(i32, i32)>) -> usize {

    let n = topics.len();
    let mut _max = 0;

    topics.sort_by(|a, b| {
        match a.0.cmp(&b.0) {
            Equal => b.1.cmp(&a.1),
            ord => ord
        }
    });

    let mut opt = vec![];
    opt.push(topics[0]);

    for i in 1..n {

        if topics[i].1 > opt[opt.len() - 1].1 {
            opt.push(topics[i]);
        }

        else { 
            let mut l = 0;
            let mut r = opt.len() - 1;

            while l < r {
                let mid = l + (r - l) / 2;


                if opt[mid].1 < topics[i].1 {
                    l = mid + 1;
                }
                else {
                    r = mid;
                }
            }

            opt[l] = topics[i]; 
        }

    }

    opt.len()
}
