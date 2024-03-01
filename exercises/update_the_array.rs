struct FenwickTree {
    tree: Vec<i64>
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

pub fn update_the_array ( n : usize,  updates : &Vec<(usize, usize, u32)>, queries : &Vec<u32>) {
    let mut array:Vec<u32> = vec![0; n];
    let mut tree = FenwickTree::with_len(n);
    
    for i in 0..n {
        tree.add(i, 0);
    }

    for update in updates {
        let (mut l, r, val) = update;

        tree.add(l, *val as i64);
        if r + 1 < n {
            tree.add(r+ 1, -(*val as i64));
        }
    }

    for query in queries {
        println!("{:?}" , tree.sum(*query as usize))
    }
}