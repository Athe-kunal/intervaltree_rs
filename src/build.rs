use crate::node::Node;
use crate::tree::Tree;

pub fn make_node(left: u32, right: u32) -> Node {
    let node: Node = match Node:: new(left, right) {
        Ok(node) => node,
        Err(err) => panic!("Failed to create root node: {}", err)
    };
    return node;
}

pub fn build_tree(items: Vec<(u32, u32)>) -> Tree{
    if items.is_empty() {
        panic!("Cannot build tree: no intervals provided");
    }
    let (left, right) = items[0];
    let root_node = make_node(left, right);
    let mut it_tree: Tree = Tree::new(root_node);
    let root = &it_tree.root;
    for (left, right) in items.iter().skip(1){
        let curr_node = make_node(*left, *right);
        let mut curr_iter_node = root;
        loop {
            if *left <= curr_iter_node.left{
                if curr_node.max > curr_iter_node.max{
                    curr_iter_node.update_max(curr_node.max);
                }
                if curr_iter_node.has_left(){
                    if let Some(child) = curr_iter_node.left_child.as_deref(){
                        curr_iter_node = child;
                    }
                    break;
                }
                else {
                    curr_iter_node.insert_left(curr_node);
                    break;
                }
            }
            else {
                if curr_iter_node.has_right(){
                    curr_iter_node = curr_iter_node.right;
                }
                else {
                    curr_iter_node.insert_right(curr_node);
                    break;
                }
            }
        }
    };
    return it_tree;
}