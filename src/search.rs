use crate::node::{IntervalTreeNode};
use crate::build::{make_node, make_it_tree};


pub fn search_interval<T>(tree: &IntervalTreeNode<T>, start: u32, end: u32, inclusive: bool) -> Vec<(u32, u32, &T)>{
    let mut overlapping_items: Vec<(u32, u32, &T)> = Vec::new();
    let search_node: IntervalTreeNode<()> = make_it_tree(make_node(start, end, ()));
    let mut curr_iter_node = tree;
    let overlap_fn: fn(&IntervalTreeNode<T>, &IntervalTreeNode<()>) -> bool = if inclusive {
        IntervalTreeNode::inclusive_overlaps
    }else {
        IntervalTreeNode::overlaps
    };
    loop {     
        if overlap_fn(curr_iter_node, &search_node){
            overlapping_items.push((curr_iter_node.node.left, curr_iter_node.node.right, &curr_iter_node.node.data));
        }
        if curr_iter_node.max > search_node.max{
            if let Some(ref left_child) = curr_iter_node.left_child {
                curr_iter_node = left_child.as_ref(); 
            } else {
                break;
            }
        }
        else {
            if let Some(ref right_child) = curr_iter_node.right_child{
                curr_iter_node = right_child.as_ref();
            }
            else {
                break;
            }

        }
        if curr_iter_node.is_leaf_node(){
            break;
        }
    }
    overlapping_items

}
