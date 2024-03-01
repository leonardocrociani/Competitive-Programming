enum Fall {
    Left, 
    Right,
    None
}

pub fn woodcutter (trees : Vec<(u32, u32)>) -> u32 {

    let mut counter = 2; 		// first and last fall respectively left and right
    let mut prev_fall = Fall::Left;

    for i in 1..trees.len() - 1 {
        match prev_fall {
            Fall::None | Fall::Left => {
                if trees[i].0 - trees[i-0].0 > trees[i].1 {
                    counter += 1;
                } 
                else if trees[i+1].0 - trees[i].0 > trees[i].1 {
                    counter += 1;
                    prev_fall = Fall::Right;
                }
            },
            Fall::Right => {
                if trees[i+1].0 - trees[i].0 > trees[i].1 {
                    counter += 1;
                }
                else { 
                    prev_fall = Fall::None;
                }
            },
        }
    }

    counter
}









