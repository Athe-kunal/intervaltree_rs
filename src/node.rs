#[derive(Debug)]
pub struct Node {
    pub left: u32,
    pub right: u32,
    pub max: u32,
    pub left_child: Option<Box<Node>>,
    pub right_child: Option<Box<Node>>,
}

impl Node {
    pub fn new(left: u32, right: u32) -> Result<Self, String>{
        if left >= right {
            return Err(format!("Invalid interval: left ({}) must be < right ({})", left, right));
        }
        Ok(Self { left, right, max: right, left_child: None, right_child: None })
    }

    pub fn update_max(&mut self, candidate: u32) {
        self.max = self.max.max(candidate)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.left < other.left
    }
    
    pub fn inclusive_overlaps(&self, other: &Self) -> bool {
        self.left <= other.left
    }

    pub fn subset(&self, other: &Self) -> bool {
        self.left < other.left && other.right > self.right
    }
    
    pub fn inclusive_subset(&self, other: &Self) -> bool {
        self.left <= other.left && other.right >= self.right
    }

    pub fn insert_left(&mut self, child: Node) {
        self.left_child = Some(Box::new(child));
    }
    
    pub fn insert_right(&mut self, child: Node) {
        self.right_child = Some(Box::new(child));
    }

    pub fn is_leaf_node(&self) -> bool {
        self.right_child.is_none() && self.left_child.is_none()
    }

    pub fn has_left(&self) -> bool {
        self.left_child.is_some()
    }

    pub fn has_right(&self) -> bool{
        self.right_child.is_some()
    }

}
