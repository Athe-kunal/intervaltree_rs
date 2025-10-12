use crate::node::{IntervalTreeNode};

pub fn insert_node<T>(root_node: &mut IntervalTreeNode<T>, new_node: IntervalTreeNode<T>) {
    root_node.update_max(new_node.max);
    if new_node.node.left <= root_node.node.left{
        if let Some(child) = root_node.left_child.as_deref_mut(){
            insert_node(child, new_node);
        }else {
            root_node.insert_left(new_node);
        }
    } else {
        if let Some(child) = root_node.right_child.as_deref_mut() {
            insert_node(child, new_node);
        }else {
            root_node.insert_right(new_node);
        }
    }
}

pub fn rotate_right<T>(mut y: Box<IntervalTreeNode<T>>) -> Box<IntervalTreeNode<T>>{
    let mut x = y.left_child.take().expect("rotate_right needs left child");
    y.left_child = x.right_child.take();
    y.recalc_max();
    x.right_child = Some(y);
    x.recalc_max();
    x

} 

pub fn rotate_left<T>(mut x: Box<IntervalTreeNode<T>>) -> Box<IntervalTreeNode<T>>{
    let mut y = x.right_child.take().expect("rotate_left expects a right child");
    x.right_child = y.left_child.take();
    x.recalc_max();
    y.left_child = Some(x);
    y.recalc_max();
    y
}

#[inline]
pub fn key_of<T>(n: &IntervalTreeNode<T>) -> (u32,u32) {
    (n.node.left, n.node.right)
}

pub fn insert_treap<T>(
    root: Option<Box<IntervalTreeNode<T>>>,
    new_node: Box<IntervalTreeNode<T>>,
) -> Option<Box<IntervalTreeNode<T>>>{
    match root {
        None => Some(new_node),
        Some(mut r) => {
            if key_of(&new_node) < key_of(&r) {
                r.left_child = insert_treap(r.left_child.take(), new_node);
                if let Some(ref lc) = r.left_child {
                    if lc.priority < r.priority{
                        r = rotate_right(r);
                    }
                }
            } else {
                r.right_child = insert_treap(r.right_child.take(), new_node);
                if let Some(ref rc) = r.right_child {
                    if rc.priority < r.priority{
                        r = rotate_left(r);
                    }
                }
            }
            r.recalc_max();
            Some(r)
        }
    }   
}