use crate::node::Node;
use crate::tree::Tree;
use crate::build::make_node;


pub fn search_interval(tree: &Tree, start: u32, end: u32, inclusive: bool) -> Vec<(u32, u32)>{
    let mut overlapping_items: Vec<(u32, u32)> = Vec::new();
    let search_node = make_node(start, end);
    let mut curr_iter_node = &tree.root;
    let overlap_fn: fn(&Node, &Node) -> bool = if inclusive {
        Node::inclusive_overlaps
    }else {
        Node::overlaps
    };
    loop {     
        if overlap_fn(curr_iter_node, &search_node){
            overlapping_items.push((curr_iter_node.left, curr_iter_node.right));
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
