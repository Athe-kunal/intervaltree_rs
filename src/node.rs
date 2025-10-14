use rand::random;

#[derive(Debug)]
pub struct IntervalTreeNode<T> {
    pub node: Node<T>,
    pub max: u32,
    pub priority: u64,
    pub left_child: Option<Box<IntervalTreeNode<T>>>,
    pub right_child: Option<Box<IntervalTreeNode<T>>>,
}

#[derive(Debug)]
pub struct Node<T = ()> {
    pub left: u32,
    pub right: u32,
    pub data: T
}

impl<T> Node<T> {
    pub fn new(left: u32, right: u32, data: T) -> Result<Self, String> {
        if left >= right {
            return Err(format!("Invalid interval: left ({}) >= right ({})", left, right));
        }
        Ok(Self {
            left,
            right,
            data,
        })
    }
}


impl<T> IntervalTreeNode<T>{
    pub fn new( root_node: Node<T>) -> Result<Self, String>{
        let max_val = root_node.right;
        let priority: u64 = random();
        Ok(Self { node: root_node, max: max_val, priority: priority, left_child: None, right_child: None })
    }
    pub fn recalc_max(&mut self) {
        let mut m = self.node.right;
        if let Some(ref l) = self.left_child { m = m.max(l.max)};
        if let Some(ref r) = self.right_child { m = m.max(r.max)};
        self.max = m;

    }
    pub fn update_max(&mut self, candidate: u32) {
        self.max = self.max.max(candidate)
    }

    pub fn overlaps_range(&self, ql: u32, qr:u32, inclusive: bool) -> bool {
        if inclusive {
            self.node.left <= qr && ql <= self.node.right
        }
        else {
            println!("Excluded {}, {}, {}, {}", self.node.left, self.node.right,qr, ql);
            self.node.left < qr && ql < self.node.right
        }
    }

    pub fn subset<U>(&self, other: &IntervalTreeNode<U>) -> bool {
        self.node.left < other.node.left && other.node.right > self.node.right
    }
    
    pub fn inclusive_subset<U>(&self, other: &IntervalTreeNode<U>) -> bool {
        self.node.left <= other.node.left && other.node.right >= self.node.right
    }

    pub fn insert_left(&mut self, child:IntervalTreeNode<T>) {
        self.left_child = Some(Box::new(child));
    }
    
    pub fn insert_right(&mut self, child:IntervalTreeNode<T>) {
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
