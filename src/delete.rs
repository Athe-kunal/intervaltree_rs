use crate::node::IntervalTreeNode;
use crate::insert::{key_of, rotate_left, rotate_right};

pub fn delete_treap<T>(
    root: Option<Box<IntervalTreeNode<T>>>,
    key: (u32, u32),
) -> Option<Box<IntervalTreeNode<T>>> {
    let mut r = root?;

    match key.cmp(&key_of(&r)) {
        std::cmp::Ordering::Less => {
            r.left_child = delete_treap(r.left_child.take(), key);
            r.recalc_max();
            Some(r)
        }
        std::cmp::Ordering::Greater => {
            r.right_child = delete_treap(r.right_child.take(), key);
            r.recalc_max();
            Some(r)
        }
        std::cmp::Ordering::Equal => {
            match (r.left_child.is_some(), r.right_child.is_some()){
                (false, false) => None,
                (true, false) => r.left_child.take(),
                (false, true) => r.right_child.take(),
                (true, true) => {
                    if r.left_child.as_ref().unwrap().priority < r.right_child.as_ref().unwrap().priority {
                        let mut new_root = rotate_right(r);
                        new_root.right_child = delete_treap(new_root.right_child.take(), key);
                        new_root.recalc_max();
                        Some(new_root)
                    }else {
                        let mut new_root = rotate_left(r);
                        new_root.left_child = delete_treap(new_root.left_child.take(), key);
                        new_root.recalc_max();
                        Some(new_root)
                    }
                }
            }
        }
    }
}