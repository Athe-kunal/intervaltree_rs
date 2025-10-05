#[derive(Debug)]
struct Node {
    left: u32,
    right: u32,
    max: u32,
}

#[derive(Debug)]
struct Tree {
    node: Node,
    left_child: Option<Box<Tree>>,
    right_child: Option<Box<Tree>>,
}

impl Node {
    fn new(left: u32, right: u32) -> Result<Self, String>{
        if left >= right {
            return Err(format!("Invalid interval: left ({}) must be < right ({})", left, right));
        }
        Ok(Self { left, right, max: right })
    }

    fn update_max(&mut self, candidate: u32) {
        self.max = self.max.max(candidate)
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.left < other.left
    }
    
    fn inclusive_overlaps(&self, other: &Self) -> bool {
        self.left <= other.left
    }

    fn subset(&self, other: &Self) -> bool {
        self.left < other.left && other.right > self.right
    }
    
    fn inclusive_subset(&self, other: &Self) -> bool {
        self.left <= other.left && other.right >= self.right
    }


}


impl Tree {
    fn new(node: Node) -> Self {
        Self {
            node, left_child: None, right_child: None
        }
    }

    fn insert_left(&mut self, child: Tree) {
        self.left_child = Some(Box::new(child));
    }
    
    fn insert_right(&mut self, child: Tree) {
        self.right_child = Some(Box::new(child));
    }
}

fn main() {
    println!("Hello, world!");
}
