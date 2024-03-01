#[derive(Debug)]
pub struct FenwickTree {
    pub tree: Vec<i64>,
}

impl FenwickTree {
    pub fn with_len(n: usize) -> Self {
        Self {
            tree: vec![0; n + 1],
        }
    }

    pub fn len(&self) -> usize {
        self.tree.len() - 1
    }

    /// Indexing is 0-based, even if internally we use 1-based indexing
    pub fn add(&mut self, i: usize, delta: i64) {
        let mut i = i + 1; 
        assert!(i < self.tree.len());

        while i < self.tree.len() {
            self.tree[i] += delta;
            i = Self::next_sibling(i);
        }
    }

    /// Indexing is 0-based, even if internally we use 1-based indexing
    pub fn sum(&self, i: usize) -> i64 {
        let mut i = i + 1;  

        assert!(i < self.tree.len());
        let mut sum = 0;
        while i != 0 {
            sum += self.tree[i];
            i = Self::parent(i);
        }

        sum
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.sum(r) - if l == 0 { 0 } else { self.sum(l - 1) }
    }

    fn isolate_trailing_one(i: usize) -> usize {
        if i == 0 {
            0
        } else {
            1 << i.trailing_zeros()
        }
    }

    fn parent(i: usize) -> usize {
        i - Self::isolate_trailing_one(i)
    }

    fn next_sibling(i: usize) -> usize {
        i + Self::isolate_trailing_one(i)
    }
}

pub fn nested_segments_fenwick_tree (arr : &mut Vec<(u32, u32)>) {
    let max: u32= arr.iter().map(|e| e.1).max().unwrap(); 
    let mut tree = FenwickTree::with_len((max + 1) as usize);

    for i in 0..arr.len() {
        let (_, end) = arr[i];
        tree.add(end as usize, 1);
    }

    arr.sort_by(|a, b| a.0.cmp(&b.0));

    for item in arr {
        let (_, end) = item;
        let res = tree.sum((*end - 1) as usize);
        tree.add(*end as usize, -1);
        println!("{res}");
    }
}
