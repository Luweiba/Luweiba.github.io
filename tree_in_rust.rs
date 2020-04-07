#[derive(Debug)]
struct Node {
    key: i32,
    val: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: i32, val: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self {
            key,
            val,
            left,
            right,
        }
    }
}
#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }
    pub fn insert(&mut self, key: i32, val: i32) {
        let node = Box::new(Node::new(key, val, None, None));
        if self.root.is_none() {
            self.root = Some(node);
            return;
        }
        let mut curr_node_wraped = self.root.as_mut();
        loop {
            let curr_node = curr_node_wraped.unwrap();
            if curr_node.key < node.key {
                if curr_node.right.is_none() {
                    curr_node.right = Some(node);
                    break;
                }
                curr_node_wraped = curr_node.right.as_mut();
            } else {
                if curr_node.left.is_none() {
                    curr_node.left = Some(node);
                    break;
                }
                curr_node_wraped = curr_node.left.as_mut();
            }
        }
    }
    
    fn find(&self, key: i32) -> Option<i32> {
        let mut curr_node_wraped = self.root.as_ref();
        loop {
            if curr_node_wraped.is_none() {
                return None;
            }
            let curr_node = curr_node_wraped.unwrap();
            if curr_node.key == key {
                return Some(curr_node.val);
            } else if curr_node.key < key {
                curr_node_wraped = curr_node.right.as_ref();
            } else {
                curr_node_wraped = curr_node.left.as_ref();
            }
        }
    }
}
fn main() {
    let mut tree = Tree::new();
    for i in 0..5 {
        tree.insert(i,i);
    }
    tree.insert(0,1);
    println!("{:?}", tree);
    if let Some(val) = tree.find(0) {
        println!("Find it {}", val);
    }
}
