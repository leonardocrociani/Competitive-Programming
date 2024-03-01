use std::{
    cmp::{max, min},
    collections::HashSet,
};

#[derive(Debug)]
pub struct SegmentTree {
    pub key: i32,
    pub range: (usize, usize),
    pub left: Option<Box<SegmentTree>>,
    pub right: Option<Box<SegmentTree>>,
}

impl SegmentTree {
    pub fn from(arr: &Vec<i32>) -> SegmentTree {
        let mut tmp = SegmentTree {
            key: -1,
            range: (0, 0),
            left: None,
            right: None,
        };
        tmp.from_helper(&arr, 0, arr.len() - 1)
    }

    fn from_helper(&mut self, arr: &Vec<i32>, l: usize, r: usize) -> SegmentTree {
        if l > r {
            panic!("Invalid range {l}, {r}");
        }
        if l == r {
            SegmentTree {
                key: arr[l],
                range: (l, r),
                left: None,
                right: None,
            }
        } else {
            let mid = l + (r - l) / 2;

            let left = self.from_helper(arr, l, mid);
            let right = self.from_helper(arr, mid + 1, r);

            let node = SegmentTree {
                key: max(left.key, right.key),
                range: (l, r),
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };

            node
        }
    }

    pub fn print_tree(&self) {
        println!("{} {:?}", self.key, self.range);
        self.helper_print(&self.left);
        self.helper_print(&self.right);
    }

    pub fn helper_print(&self, curr: &Option<Box<SegmentTree>>) {
        match curr {
            Some(val) => {
                println!("{} {:?}", val.as_ref().key, val.as_ref().range);
                self.helper_print(&val.as_ref().left);
                self.helper_print(&val.as_ref().right);
            }
            None => (),
        }
    }
}

impl SegmentTree {
    pub fn Update(&mut self, i: usize, j: usize, val: i32) {
        self._update(i - 1, j - 1, val);
    }
    fn _update(&mut self, i: usize, j: usize, val: i32) -> Option<i32> {
        let range = self.range;

        if range.0 == range.1 {
            // leaf -> update and return
            self.key = min(self.key, val);
            return Some(self.key);
        }

        // no overlap
        if range.1 < i || j < range.0 {
            return Some(self.key);
        }

        // overlap -> update
        let nvs = if let Some(sx) = &mut self.left {
            sx._update(i, j, val)
        } else {
            None
        };
        let nvd = if let Some(dx) = &mut self.right {
            dx._update(i, j, val)
        } else {
            None
        };
        self.key = max(nvs, nvd).unwrap();
        return max(nvs, nvd);
    }

    pub fn Max(&self, i: usize, j: usize) -> i32 {
        return self._max(i - 1, j - 1).unwrap();
    }

    fn _max(&self, i: usize, j: usize) -> Option<i32> {
        let range = self.range;

        if i > range.1 || j < range.0 {
            return None;
        }

        if i <= range.0 && range.1 <= j {
            return Some(self.key);
        }

        let ml = if let Some(sx) = &self.left {
            sx._max(i, j)
        } else {
            None
        };
        let mr = if let Some(dx) = &self.right {
            dx._max(i, j)
        } else {
            None
        };

        return max(ml, mr);
    }
}

// --- problema 2 ---

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Bound {
    Begin,
    End,
}

#[derive(Debug)]
pub struct OverlapST {
    pub key: HashSet<usize>,
    pub range: (usize, usize),
    pub left: Option<Box<OverlapST>>,
    pub right: Option<Box<OverlapST>>,
}

impl OverlapST {
    pub fn from_segments(segments: &mut Vec<(usize, usize)>) -> OverlapST {
        // compute overlaps in O(n log n)

        let mut pairs: Vec<_> = segments
            .iter()
            .flat_map(|&(b, e)| [(b, Bound::Begin), (e, Bound::End)])
            .collect();

        pairs.sort_unstable();

        let max = pairs[pairs.len() - 1].0;

        let mut last_el: i32 = -1;
        let mut counter = 0;
        let mut overlaps: Vec<usize> = vec![0; max + 1];

        for i in 0..pairs.len() {
            let pair = &pairs[i];

            if last_el != -1 && (last_el != pair.0 as i32) {
                overlaps[last_el as usize] = counter + 1;
            }

            match pair.1 {
                Bound::Begin => counter += 1,
                Bound::End => counter -= 1,
            }

            last_el = pair.0 as i32;
        }

        overlaps[last_el as usize] = counter + 1;

        let mut tmp = OverlapST {
            key: HashSet::new(),
            range: (0, 0),
            left: None,
            right: None,
        };

        tmp.from_helper(&overlaps, 0, overlaps.len() - 1)
    }

    fn from_helper(&mut self, arr: &Vec<usize>, l: usize, r: usize) -> OverlapST {
        if l > r {
            panic!("Invalid range {l}, {r}");
        }

        if l == r {
            // leaf
            let mut set = HashSet::new();
            set.insert(arr[l]);
            
            OverlapST {
                key: set,
                range: (l, r),
                left: None,
                right: None,
            }

        } else {
            let mid = l + (r - l) / 2;

            let left = self.from_helper(arr, l, mid);
            let right = self.from_helper(arr, mid + 1, r);

            let node = OverlapST {
                key: OverlapST::merge_set(&left.key, &right.key),
                range: (l, r),
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };

            node
        }
    }

    fn merge_set (s1: &HashSet<usize>, s2: &HashSet<usize>) -> HashSet<usize> {
        let mut s3 = s1.clone();
        for element in s2 {
            s3.insert(*element);
        }
        println!("Merged {:?} and {:?} into -> {:?}", s1, s2, s3);
        s3
    }
}

impl OverlapST {
    pub fn IsThere (&self, i:usize, j:usize, k: u32) -> bool {

        let range = self.range;

        if i > range.1 || j < range.0 {
            return false;
        }

        if i <= range.0 && range.1 <= j {
            return self.key.contains(&(k as usize));
        }

        let left = if let Some(sx) = &self.left { sx.IsThere(i, j, k) } else { false };
        let right = if let Some (dx) = &self.right { dx.IsThere(i, j, k) } else {false};

        return left || right;
    }
}

