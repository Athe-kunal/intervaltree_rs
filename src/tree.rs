
use crate::node::Node;

#[derive(Debug)]
pub struct Tree {
    pub root: Node,
    
}



impl Tree {
    pub fn new(root: Node) -> Self {
        Self {
            root
        }
    }
}
