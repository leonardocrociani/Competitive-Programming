struct MaxPathSum {
    value : i32
}

impl Tree {
    pub fn maximum_path_sum(&self) -> i32 {
        let mut mps = MaxPathSum { value: i32::min_value() };
        
        self.traverse(Some(0), &mut mps);
        
        return mps.value;
    }

    fn traverse(&self, node_id: Option<usize>, mps: &mut MaxPathSum) -> i32 {
        match node_id {
            None => 0,
            Some(id) => {
                let node = &self.nodes[id];

                let left = max(0, self.traverse(node.id_left, mps));
                let right = max(0, self.traverse(node.id_right, mps));

		// path passing from ndoe.key
                mps.value = max(mps.value, left + right + node.key);

		// other node can use the best path
                return node.key + max(left, right);
            }
        }
    }
}


// HELPERS DATA STRUCTURES

use std::cmp::max;

pub struct Node {
    pub key: i32,
    pub id_left: Option<usize>,
    pub id_right: Option<usize>,
}

impl Node {
    pub fn new(key: i32) -> Self {
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
    pub fn with_root(key: i32) -> Self {
        Self {
            nodes: vec![Node::new(key)],
        }
    }

    pub fn add_node(&mut self, parent_id: usize, key: i32, is_left: bool) -> usize {
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

