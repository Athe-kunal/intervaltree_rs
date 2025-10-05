use crate::node::Node;
use crate::tree::Tree;

pub fn make_node(left: u32, right: u32) -> Node {
    let node: Node = match Node:: new(left, right) {
        Ok(node) => node,
        Err(err) => panic!("Failed to create root node: {}", err)
    };
    return node;
}

pub fn insert(root_node: &mut Node, new_node: Node) {
    root_node.update_max(new_node.max);
    if new_node.left <= root_node.left{
        if let Some(child) = root_node.left_child.as_deref_mut(){
            insert(child, new_node);
        }else {
            root_node.insert_left(new_node);
        }
    } else {
        if let Some(child) = root_node.right_child.as_deref_mut() {
            insert(child, new_node);
        }else {
            root_node.insert_right(new_node);
        }
    }
}
pub fn build_tree(items: Vec<(u32, u32)>) -> Tree{
    if items.is_empty() {
        panic!("Cannot build tree: no intervals provided");
    }
    let (left, right) = items[0];
    let root_node = make_node(left, right);
    let mut it_tree: Tree = Tree::new(root_node);
    for (left, right) in items.iter().skip(1){
        insert(&mut it_tree.root, make_node(*left, *right))
    };
    return it_tree;
}