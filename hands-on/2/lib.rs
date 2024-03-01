// --- PROBLEMA 1 ---

use std::{cmp::{max, min}};

#[derive(Debug)]
pub struct SegmentTree {
    pub key: i32,
    pub lazy: i32,
    pub range: (usize, usize),
    pub left: Option<Box<SegmentTree>>,
    pub right: Option<Box<SegmentTree>>,
}

// CREAZIONE ALBERO
impl SegmentTree {
    pub fn from(arr: Vec<i32>) -> SegmentTree {
        let mut tmp = SegmentTree {
            key: -1,
            lazy: -1,
            range: (0, 0),
            left: None,
            right: None,
        };
        tmp.from_helper(&arr, 0, arr.len() - 1)
    }

    fn from_helper(&mut self, arr: &Vec<i32>, l: usize, r: usize) -> SegmentTree {
        if l == r {
            SegmentTree {
                key: arr[l],
                lazy: arr[l],
                range: (l, r),
                left: None,
                right: None,
            }
        } else {
            let mid = l + (r - l) / 2;

            let left = self.from_helper(arr, l, mid);
            let right = self.from_helper(arr, mid + 1, r);

            let _max = max(left.key, right.key);

            let node = SegmentTree {
                key: _max,
                lazy: _max,
                range: (l, r),
                left: Some(Box::new(left)),
                right: Some(Box::new(right)),
            };

            node
        }
    }

    pub fn print_tree(&self) {
        println!("{}|{} {:?}", self.key, self.lazy, self.range);
        self.helper_print(&self.left);
        self.helper_print(&self.right);
    }

    pub fn helper_print(&self, curr: &Option<Box<SegmentTree>>) {
        match curr {
            Some(val) => {
                println!(
                    "{}|{} {:?}",
                    val.as_ref().key,
                    val.as_ref().lazy,
                    val.as_ref().range
                );
                self.helper_print(&val.as_ref().left);
                self.helper_print(&val.as_ref().right);
            }
            None => (),
        }
    }
}

// IMPLEMENTAZIONE DELLE OPERAZIONI
impl SegmentTree {
    pub fn Update(&mut self, i: usize, j: usize, val: i32) {
        self.update(i - 1, j - 1, val);
    }
    fn update(&mut self, i: usize, j: usize, val: i32) -> Option<i32> {

        // lazy update
        if self.key != self.lazy {
            self.key = min(self.key, self.lazy);
            self.lazy = self.key;
        }

        let range = self.range;

        // no overlap
        if range.1 < i || j < range.0 {
            return Some(self.key);
        }

        // total overlap -> lazy propagation
        if i <= range.0 && range.1 <= j {
            self.key = min(self.key, val);
            self.lazy = self.key;

            if let Some(sx) = &mut self.left {
                sx.lazy = min(val, sx.lazy)
            };
            if let Some(dx) = &mut self.right {
                dx.lazy = min(val, dx.lazy)
            };

            return Some(self.key);
        }

        // partial overlap -> update
        let nvs = if let Some(sx) = &mut self.left {
            sx.update(i, j, val)
        } else {
            None
        };
        let nvd = if let Some(dx) = &mut self.right {
            dx.update(i, j, val)
        } else {
            None
        };

        self.key = max(nvs, nvd).unwrap();
        self.lazy = self.key;
        
        return Some(self.key);
    }

    pub fn Max(&mut self, i: usize, j: usize) -> i32 {
        return self.max(i - 1, j - 1).unwrap();
    }

    fn max(&mut self, i: usize, j: usize) -> Option<i32> {
        if self.key != self.lazy {
            self.key = min(self.key, self.lazy);
            self.lazy = self.key;
        }

        let range = self.range;

        if i > range.1 || j < range.0 {
            return None;
        }

        if i <= range.0 && range.1 <= j {
            return Some(self.key);
        }

        let ml = if let Some(sx) = &mut self.left {
            sx.max(i, j)
        } else {
            None
        };
        let mr = if let Some(dx) = &mut self.right {
            dx.max(i, j)
        } else {
            None
        };

        return max(ml, mr);
    }
}

// --- PROBLEMA 2 ---

#[derive(Debug, Clone)]
pub struct S2TNode {
    key: i32,
    lazy: i32,
}

#[derive(Debug)]
pub struct S2Tree {
    pub nodes: Vec<S2TNode>,
    pub length: usize
}

// CREAZIONE ALBERO
impl S2Tree {

    pub fn from_segments(segments: Vec<(usize, usize)>) -> S2Tree {
        let n = segments.len();
        let dim = (2 * n - 1).next_power_of_two();

        let mut tree = S2Tree {
            nodes: vec![S2TNode { key: 0, lazy: 0 }; dim - 1],
            length : n
        };

        for segment in segments {
            tree.update(segment, 0, n - 1, 0);
        }

        tree
    }

    fn update(&mut self, seg: (usize, usize), l: usize, r: usize, i: usize) {

        let is_leaf = l == r;
        let sx_idx = 2 * i + 1;
        let dx_idx = 2 * i + 2;

        // lazy update
        if self.nodes[i].lazy != 0 {
            self.nodes[i].key += self.nodes[i].lazy;

            if !is_leaf {
                self.nodes[sx_idx].lazy += self.nodes[i].lazy;
                self.nodes[dx_idx].lazy += self.nodes[i].lazy;
            }

            self.nodes[i].lazy = 0;
        }

        if seg.1 < l || seg.0 > r {
            return;
        }

        // lazy propagation
        if seg.0 <= l && r <= seg.1 {
            self.nodes[i].key += 1;

            if !is_leaf {
                self.nodes[sx_idx].lazy += 1;
                self.nodes[dx_idx].lazy += 1;
            }

            return;
        }

        let mid = l + (r - l) / 2;

        self.update(seg, l, mid, sx_idx);
        self.update(seg, mid + 1, r, dx_idx);

        self.nodes[i].key = max(self.nodes[sx_idx].key, self.nodes[dx_idx].key);
    }
}

// IMPLEMENTAZIONE QUERY ISTHERE
impl S2Tree {
    pub fn IsThere(&mut self, i: usize, j: usize, k: i32) -> i32 {
        if self.is_there(i, j, k, 0, self.length - 1, 0) {
            1
        }
        else { 0 }
    }

    fn is_there(&mut self, ql: usize, qr: usize, k: i32, l: usize, r: usize, i: usize) -> bool {
        let is_leaf = l==r;
        let sx_idx = 2 * i + 1;
        let dx_idx = 2 * i + 2;

        if self.nodes[i].lazy != 0 {
            self.nodes[i].key += self.nodes[i].lazy;

            if !is_leaf {
                self.nodes[sx_idx].lazy += self.nodes[i].lazy;
                self.nodes[dx_idx].lazy += self.nodes[i].lazy;
            }

            self.nodes[i].lazy = 0;
        }

        if self.nodes[i].key < k {
            return false;
        }

        if qr < l || r < ql {
            return false;
        }

        if l >= ql && r <= qr {
            if self.nodes[i].key == k {
                return true
            }
            if is_leaf {
                return false
            }
        }
        
        let mid = l + (r - l) / 2;

        let left_result = self.is_there(ql, qr, k, l, mid, sx_idx);
        let right_result = self.is_there(ql, qr, k, mid + 1, r, dx_idx);

        left_result || right_result
    }
}
