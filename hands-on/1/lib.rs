use std::{
    cmp::{max, min},
    f32::INFINITY,
};

struct Node {
    key: u32,
    id_left: Option<usize>,
    id_right: Option<usize>,
}

impl Node {
    fn new(key: u32) -> Self {
        Self {
            key,
            id_left: None,
            id_right: None,
        }
    }
}

pub struct Tree {
    nodes: Vec<Node>,
}

impl Tree {
    pub fn with_root(key: u32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub fn add_node(&mut self, parent_id: usize, key: u32, is_left: bool) -> usize {
        assert!(
            parent_id < self.nodes.len(),
            "Parent node id does not exist"
        );
        if is_left {
            assert!(
                self.nodes[parent_id].id_left == None,
                "Parent node has the left child already set"
            );
        } else {
            assert!(
                self.nodes[parent_id].id_right == None,
                "Parent node has the right child already set"
            );
        }

        let child_id = self.nodes.len();
        self.nodes.push(Node::new(key));

        let child = if is_left {
            &mut self.nodes[parent_id].id_left
        } else {
            &mut self.nodes[parent_id].id_right
        };

        *child = Some(child_id);

        child_id
    }
}

impl Tree {
    pub fn sum(&self) -> u32 {
        self.rec_sum(Some(0))
    }

    fn rec_sum(&self, node_id: Option<usize>) -> u32 {
        if let Some(id) = node_id {
            assert!(id < self.nodes.len(), "Node id is out of range");
            let node = &self.nodes[id];

            let sum_left = self.rec_sum(node.id_left);
            let sum_right = self.rec_sum(node.id_right);

            return sum_left + sum_right + node.key;
        }

        0
    }
}

// --- IS BST ---

impl Tree {
    pub fn is_bst(&self, node_id: Option<usize>) -> (bool, i32, i32) {
        if let Some(id) = node_id {
            let node = &self.nodes[id];

            let (bl, minl, maxl) = self.is_bst(node.id_left);
            let (br, minr, maxr) = self.is_bst(node.id_right);

            let _min = min(min(minl, minr), node.key as i32);
            let _max = max(max(maxl, maxr), node.key as i32);

            if maxl > node.key as i32 || minr <= node.key as i32 {
                return (false, _min, _max);
            }

            return (bl && br, _min, _max);
        }

        (true, i32::MAX, i32::MIN)
    }
}

// --- IS BALANCED ---

impl Tree {
    pub fn is_balanced(&self, node_id: Option<usize>) -> (bool, i32) {
        if let Some(id) = node_id {
            let node = &self.nodes[id];

            let (cl, hsx) = self.is_balanced(node.id_left);
            let (cr, hdx) = self.is_balanced(node.id_right);

            let diff = (hsx - hdx).abs();

            return (cl && cr && diff <= 1, 1 + max(hsx, hdx));
        }

        (true, -1)
    }
}

// --- IS MAX HEAP ---

impl Tree {
    pub fn is_max_heap(&self) -> bool {
        self.respect_max_heap(Some(0), 0, self.nodes.len()).0
    }
    pub fn respect_max_heap(&self, node_id: Option<usize>, index:usize, nodes_count: usize) -> (bool, bool, i32) {
        if let Some(id) = node_id {
            let node = &self.nodes[id];
            
            let (mhl, cl, max_sx) = self.respect_max_heap(node.id_left, 2*index + 1, nodes_count);
            let (mhr, cr, max_dx) = self.respect_max_heap(node.id_right,2*index+2, nodes_count);

            let _max = max(node.key as i32, max(max_sx, max_dx)); 
            let max_cond = node.key as i32 >= max_sx && node.key as i32 >= max_dx;
            
            let complete_cond = if index >= nodes_count { false } else { cl && cr };

            return (
                mhl && mhr && max_cond && complete_cond, 
                complete_cond,
                _max, 
            );
        }

        (true, true, i32::MIN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- self build tests ---

    #[test]
    fn test_is_bst() {
        let tree = Tree::with_root(10);

        assert!(tree.is_bst(Some(0)).0);

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 3, true);
        tree.add_node(1, 1, true);
        tree.add_node(1, 6, false);
        tree.add_node(3, 4, true);
        tree.add_node(3, 7, false);

        tree.add_node(0, 10, false);
        tree.add_node(6, 14, false);
        tree.add_node(7, 13, true);

        assert!(!tree.is_bst(Some(0)).0);

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);
        tree.add_node(2, 12, true);

        assert!(tree.is_bst(Some(0)).0);

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);
        tree.add_node(1, 20, true);

        assert!(!tree.is_bst(Some(0)).0);
    }

    #[test]
    fn test_is_balanced() {
        let mut tree = Tree::with_root(8);
        tree.add_node(0, 3, true);
        tree.add_node(1, 1, true);
        tree.add_node(1, 6, false);
        tree.add_node(3, 4, true);
        tree.add_node(3, 7, false);

        tree.add_node(0, 10, false);
        tree.add_node(6, 14, false);
        tree.add_node(7, 13, true);

        assert!(!tree.is_balanced(Some(0)).0);

        let mut tree = Tree::with_root(8);
        tree.add_node(0, 3, true);
        tree.add_node(1, 1, true);
        tree.add_node(1, 6, false);
        tree.add_node(3, 4, true);
        tree.add_node(3, 7, false);

        tree.add_node(0, 10, false);
        tree.add_node(6, 14, false);

        assert!(tree.is_balanced(Some(0)).0);

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(1, 3, true);

        assert!(!tree.is_balanced(Some(0)).0);

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);
        tree.add_node(2, 12, true);
        tree.add_node(2, 18, false);

        assert!(tree.is_balanced(Some(0)).0);

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(1, 3, true);
        tree.add_node(2, 2, true);

        assert!(!tree.is_balanced(Some(0)).0);
    }

    #[test]
    fn test_is_max_heap() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 3, true);
        tree.add_node(1, 1, true);
        tree.add_node(1, 6, false);
        tree.add_node(3, 4, true);
        tree.add_node(3, 7, false);

        tree.add_node(0, 10, false);
        tree.add_node(6, 14, false);

        assert!(!tree.is_max_heap());

        println!("!!!");

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 15, false);

        assert!(!tree.is_max_heap());

        let mut tree = Tree::with_root(15);
        tree.add_node(0, 6, true);
        tree.add_node(0, 10, false);
        tree.add_node(1, 3, true);
        tree.add_node(1, 5, false);

        assert!(tree.is_max_heap());

        let mut tree = Tree::with_root(10);
        tree.add_node(0, 5, true);
        tree.add_node(0, 15, false);
        tree.add_node(1, 8, false);

        assert!(!tree.is_max_heap());
    }

    // --- stolen tests ---

    #[test]
    fn test_is_bst_1() {
        let mut tree = Tree::with_root(10);

        assert!(tree.is_bst(Some(0)).0);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2

        assert!(tree.is_bst(Some(0)).0);

        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        assert!(tree.is_bst(Some(0)).0);

        tree.add_node(3, 6, false); // id 5

        assert!(!tree.is_bst(Some(0)).0);
    }

    #[test]
    fn test_is_bst_2() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2
        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        tree.add_node(3, 8, true); // id 5

        assert!(!tree.is_bst(Some(0)).0);
    }

    #[test]
    fn test_is_bst_3() {
        let mut tree = Tree::with_root(10);

        tree.add_node(0, 5, true); // id 1
        tree.add_node(0, 22, false); // id 2
        tree.add_node(1, 7, false); // id 3
        tree.add_node(2, 20, true); // id 4

        tree.add_node(4, 8, true); // id 5

        assert!(!tree.is_bst(Some(0)).0);
    }

    #[test]
    fn test_is_bst_4() {
        let mut tree = Tree::with_root(0);

        tree.add_node(0, 1, false); // id 1
        tree.add_node(1, 2, false); // id 2
        tree.add_node(2, 3, false); // id 3

        assert!(tree.is_bst(Some(0)).0);
    }

    #[test]
    fn test_is_bst_5() {
        let mut tree = Tree::with_root(3);

        tree.add_node(0, 2, true); // id 1
        tree.add_node(1, 1, true); // id 2
        tree.add_node(2, 0, true); // id 3

        assert!(tree.is_bst(Some(0)).0);
    }

    #[test]
    fn test_is_balanced_1() {
        let mut tree = Tree::with_root(0);
        assert!(tree.is_balanced(Some(0)).0);

        tree.add_node(0, 0, true); // id 1
        assert!(tree.is_balanced(Some(0)).0);

        tree.add_node(0, 0, false); // id 2
        assert!(tree.is_balanced(Some(0)).0);

        tree.add_node(1, 0, false); // id 3
        assert!(tree.is_balanced(Some(0)).0);

        tree.add_node(2, 0, true); // id 4
        assert!(tree.is_balanced(Some(0)).0);
    }

    #[test]
    fn test_is_balanced_2() {
        let mut tree = Tree::with_root(0);

        tree.add_node(0, 0, true); // id 1
        tree.add_node(0, 0, false); // id 2
        tree.add_node(1, 0, false); // id 3
        tree.add_node(2, 0, true); // id 4

        tree.add_node(3, 0, true); // id 5

        assert!(!tree.is_balanced(Some(0)).0);
    }

    #[test]
    fn test_is_balanced_3() {
        let mut tree = Tree::with_root(0);

        tree.add_node(0, 0, true); // id 1
        tree.add_node(0, 0, false); // id 2
        tree.add_node(1, 0, false); // id 3
        tree.add_node(2, 0, true); // id 4

        tree.add_node(3, 0, false); // id 5

        assert!(!tree.is_balanced(Some(0)).0);
    }

    #[test]
    fn test_is_balanced_4() {
        let mut tree = Tree::with_root(0);

        tree.add_node(0, 0, true); // id 1
        tree.add_node(1, 0, false); // id 2

        assert!(!tree.is_balanced(Some(0)).0);
    }

    #[test]
    fn test_is_max_heap_1() {
        let mut tree = Tree::with_root(10);
        //assert!(tree.is_max_heap());

        tree.add_node(0, 8, true); // id 1
        //assert!(tree.is_max_heap());

        tree.add_node(0, 6, false); // id 2
        //assert!(tree.is_max_heap());

        tree.add_node(1, 7, false); // id 3
        assert!(!tree.is_max_heap());

        tree.add_node(1, 5, true); // id 4
        assert!(tree.is_max_heap());
    }

    #[test]
    fn test_is_max_heap_2() {
        let mut tree = Tree::with_root(10);
        assert!(tree.is_max_heap());

        tree.add_node(0, 9, true); // id 1
        assert!(tree.is_max_heap());

        tree.add_node(0, 11, false); // id 2
        assert!(!tree.is_max_heap());
    }

    #[test]
    fn test_is_max_heap_3() {
        let mut tree = Tree::with_root(10);
        assert!(tree.is_max_heap());

        tree.add_node(0, 9, true); // id 1
        tree.add_node(1, 8, true); // id 2
        assert!(!tree.is_max_heap());

        tree.add_node(1, 9, false); // id 3
        assert!(!tree.is_max_heap());

        tree.add_node(0, 6, false); // id 4
        assert!(tree.is_max_heap());

        tree.add_node(4, 3, true); // id 5
        assert!(tree.is_max_heap());

        tree.add_node(4, 2, false); // id 6
        assert!(tree.is_max_heap());

        tree.add_node(5, 2, true); // id 7
        assert!(!tree.is_max_heap());
    }

    #[test]
    fn test_is_max_heap_4() {
        let mut tree = Tree::with_root(100);

        tree.add_node(0, 99, true);
        tree.add_node(0,98,false);
        tree.add_node(1, 97,true);
        tree.add_node(1,96, false);
        tree.add_node(2,95, true);
        tree.add_node(2, 94, false);
        tree.add_node(3, 93, true);
        tree.add_node(3, 92,false);
        tree.add_node(4, 91, true);
        tree.add_node(4, 90, false);
        tree.add_node(5,89,true);
        tree.add_node(5, 88,false);
        tree.add_node(6, 87, true);
        tree.add_node(7, 86, true);
        tree.add_node(7, 85,false);

        assert!(!tree.is_max_heap())
    }

    #[test]
    fn more_tests() {

        // should print true
        let mut bst0 = Tree::with_root(43);
        bst0.add_node(0, 64, false);
        bst0.add_node(1, 51, true);
        bst0.add_node(0, 24, true);
        bst0.add_node(1, 80, false);
        bst0.add_node(2, 55, false);
        bst0.add_node(2, 44, true);
        bst0.add_node(3, 13, true);
        bst0.add_node(6, 49, false);
        bst0.add_node(5, 63, false);
        bst0.add_node(4, 83, false);
        bst0.add_node(10, 88, false);
        bst0.add_node(4, 66, true);
        bst0.add_node(12, 69, false);
        assert!(bst0.is_bst(Some(0)).0);

        // should print true
        let mut bst1 = Tree::with_root(7);
        bst1.add_node(0, 11, false);
        bst1.add_node(1, 20, false);
        bst1.add_node(2, 34, false);
        bst1.add_node(3, 85, false);
        bst1.add_node(4, 96, false);
        bst1.add_node(4, 66, true);
        bst1.add_node(3, 25, true);
        bst1.add_node(0, 5, true);
        bst1.add_node(6, 40, true);
        bst1.add_node(5, 100, false);
        assert!(bst1.is_bst(Some(0)).0);

        // should print true
        let mut bst2 = Tree::with_root(48);
        assert!(bst2.is_bst(Some(0)).0);

        // should print true
        let mut bst3 = Tree::with_root(39);
        bst3.add_node(0, 81, false);
        bst3.add_node(1, 80, true);
        bst3.add_node(1, 90, false);
        bst3.add_node(2, 49, true);
        bst3.add_node(4, 51, false);
        assert!(bst3.is_bst(Some(0)).0);

        // should print true
        let mut bst4 = Tree::with_root(69);
        bst4.add_node(0, 97, false);
        bst4.add_node(0, 66, true);
        bst4.add_node(2, 8, true);
        bst4.add_node(3, 41, false);
        bst4.add_node(4, 56, false);
        bst4.add_node(4, 24, true);
        bst4.add_node(5, 49, true);
        bst4.add_node(3, 4, true);
        bst4.add_node(6, 16, true);
        bst4.add_node(8, 5, false);
        bst4.add_node(1, 88, true);
        bst4.add_node(6, 27, false);
        bst4.add_node(12, 37, false);
        bst4.add_node(7, 53, false);
        assert!(bst4.is_bst(Some(0)).0);

        // should print true
        let mut bst5 = Tree::with_root(40);
        bst5.add_node(0, 83, false);
        bst5.add_node(1, 54, true);
        bst5.add_node(2, 66, false);
        bst5.add_node(0, 37, true);
        bst5.add_node(1, 97, false);
        bst5.add_node(5, 95, true);
        bst5.add_node(6, 89, true);
        bst5.add_node(3, 63, true);
        bst5.add_node(4, 4, true);
        bst5.add_node(5, 100, false);
        bst5.add_node(8, 57, true);
        bst5.add_node(9, 5, false);
        bst5.add_node(12, 36, false);
        bst5.add_node(3, 77, false);
        bst5.add_node(6, 96, false);
        bst5.add_node(7, 85, true);
        bst5.add_node(13, 23, true);
        assert!(bst5.is_bst(Some(0)).0);

        // should print true
        let mut bst6 = Tree::with_root(55);
        bst6.add_node(0, 45, true);
        bst6.add_node(0, 84, false);
        bst6.add_node(2, 78, true);
        bst6.add_node(1, 19, true);
        bst6.add_node(4, 11, true);
        bst6.add_node(4, 35, false);
        bst6.add_node(2, 95, false);
        assert!(bst6.is_bst(Some(0)).0);

        // should print true
        let mut bst7 = Tree::with_root(6);
        bst7.add_node(0, 84, false);
        bst7.add_node(1, 25, true);
        bst7.add_node(2, 36, false);
        bst7.add_node(2, 18, true);
        bst7.add_node(1, 92, false);
        bst7.add_node(3, 53, false);
        bst7.add_node(0, 1, true);
        bst7.add_node(6, 81, false);
        bst7.add_node(3, 30, true);
        bst7.add_node(9, 26, true);
        bst7.add_node(8, 70, true);
        bst7.add_node(10, 28, false);
        assert!(bst7.is_bst(Some(0)).0);

        // should print true
        let mut bst8 = Tree::with_root(39);
        bst8.add_node(0, 50, false);
        bst8.add_node(1, 64, false);
        bst8.add_node(2, 60, true);
        bst8.add_node(0, 11, true);
        bst8.add_node(2, 84, false);
        bst8.add_node(3, 52, true);
        bst8.add_node(5, 81, true);
        bst8.add_node(1, 48, true);
        bst8.add_node(7, 83, false);
        bst8.add_node(8, 43, true);
        assert!(bst8.is_bst(Some(0)).0);

        // should print true
        let mut bst9 = Tree::with_root(26);
        assert!(bst9.is_bst(Some(0)).0);
        
        // should print false
        let mut non_bst0 = Tree::with_root(71);
        non_bst0.add_node(0, 16, true);
        non_bst0.add_node(1, 27, true);
        non_bst0.add_node(0, 77, false);
        non_bst0.add_node(2, 68, true);
        non_bst0.add_node(3, 80, false);
        non_bst0.add_node(3, 57, true);
        non_bst0.add_node(6, 98, false);
        non_bst0.add_node(4, 54, false);
        non_bst0.add_node(5, 100, false);
        non_bst0.add_node(7, 46, true);
        non_bst0.add_node(1, 31, false);
        non_bst0.add_node(6, 56, true);
        non_bst0.add_node(7, 92, false);
        non_bst0.add_node(12, 84, false);
        assert!(!non_bst0.is_bst(Some(0)).0);

        // should print false
        let mut non_bst1 = Tree::with_root(92);
        non_bst1.add_node(0, 51, true);
        non_bst1.add_node(1, 52, false);
        non_bst1.add_node(0, 14, false);
        non_bst1.add_node(3, 73, false);
        non_bst1.add_node(3, 58, true);
        non_bst1.add_node(4, 9, false);
        non_bst1.add_node(5, 88, true);
        non_bst1.add_node(2, 91, false);
        assert!(!non_bst1.is_bst(Some(0)).0);

        // should print false
        let mut non_bst2 = Tree::with_root(27);
        non_bst2.add_node(0, 83, true);
        non_bst2.add_node(0, 25, false);
        assert!(!non_bst2.is_bst(Some(0)).0);

        // should print false
        let mut non_bst3 = Tree::with_root(38);
        non_bst3.add_node(0, 22, true);
        non_bst3.add_node(1, 33, true);
        non_bst3.add_node(0, 95, false);
        non_bst3.add_node(1, 0, false);
        non_bst3.add_node(2, 74, true);
        non_bst3.add_node(5, 9, true);
        assert!(!non_bst3.is_bst(Some(0)).0);

        // should print false
        let mut non_bst4 = Tree::with_root(49);
        non_bst4.add_node(0, 89, true);
        non_bst4.add_node(0, 50, false);
        non_bst4.add_node(2, 57, true);
        non_bst4.add_node(2, 73, false);
        non_bst4.add_node(4, 23, true);
        non_bst4.add_node(1, 6, true);
        non_bst4.add_node(6, 38, false);
        non_bst4.add_node(1, 42, false);
        non_bst4.add_node(5, 52, true);
        non_bst4.add_node(3, 15, true);
        non_bst4.add_node(10, 27, true);
        non_bst4.add_node(7, 82, false);
        assert!(!non_bst4.is_bst(Some(0)).0);

        // should print false
        let mut non_bst5 = Tree::with_root(46);
        non_bst5.add_node(0, 38, true);
        non_bst5.add_node(1, 78, false);
        non_bst5.add_node(0, 22, false);
        non_bst5.add_node(1, 26, true);
        non_bst5.add_node(3, 92, false);
        non_bst5.add_node(2, 80, true);
        non_bst5.add_node(3, 13, true);
        non_bst5.add_node(4, 0, true);
        assert!(!non_bst5.is_bst(Some(0)).0);

        // should print false
        let mut non_bst6 = Tree::with_root(40);
        non_bst6.add_node(0, 29, true);
        non_bst6.add_node(0, 4, false);
        non_bst6.add_node(2, 55, true);
        assert!(!non_bst6.is_bst(Some(0)).0);

        // should print false
        let mut non_bst7 = Tree::with_root(62);
        non_bst7.add_node(0, 73, true);
        assert!(!non_bst7.is_bst(Some(0)).0);

        // should print false
        let mut non_bst8 = Tree::with_root(93);
        non_bst8.add_node(0, 24, true);
        non_bst8.add_node(0, 39, false);
        assert!(!non_bst8.is_bst(Some(0)).0);

        // should print false
        let mut non_bst9 = Tree::with_root(28);
        non_bst9.add_node(0, 36, true);
        assert!(!non_bst9.is_bst(Some(0)).0);
    }
}
