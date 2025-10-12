use crate::node::{IntervalTreeNode,Node};
use crate::insert::{insert_node};

pub fn make_node<T>(left: u32, right: u32, data: T) -> Node<T> {
    let node: Node<T> = match Node :: new(left, right, data) {
        Ok(node) => node,
        Err(err) => panic!("Failed to create root node: {}", err)
    };
    return node;
}

pub fn make_it_tree<T>(node: Node<T>) -> IntervalTreeNode<T>{
    let it_tree: IntervalTreeNode<T> = match IntervalTreeNode:: new(node) {
        Ok(it_tree) => it_tree,
        Err(err) => panic!("Failed to create root node: {}", err)
    };
    return it_tree;
}

pub fn build_tree<T>(items: Vec<(u32, u32, T)>) -> IntervalTreeNode<T>{
    if items.is_empty() {
        panic!("Cannot build tree: no intervals provided");
    }
    let mut it = items.into_iter();
    let (left, right, data) = it.next().unwrap();
    let root_node: Node<T> = make_node(left, right, data);
    let mut it_tree: IntervalTreeNode<T> = IntervalTreeNode::new(root_node).unwrap();
    for (left, right, data) in it{
        insert_node(&mut it_tree, make_it_tree(make_node(left, right, data)))
    };
    return it_tree;
}