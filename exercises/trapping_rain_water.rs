pub fn trapping_rain_water(height: Vec<i32>) -> i32 {
    let mut trapped = 0;
    let mut l = 0;
    let mut r = height.len() - 1;
    let mut lmax = 0;
    let mut rmax = 0;

    while l < r {
        if height[l] < height[r] {
            if height[l] > lmax {
                // there is a block
                lmax = height[l];
            } else {
                trapped += lmax - height[l]; // i can trap
            }

            l += 1;
        } else {
            if height[r] > rmax {
                rmax = height[r];
            } else {
                trapped += rmax - height[r];
            }

            r -= 1;
        }
    }

    trapped
}

