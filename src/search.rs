use crate::node::{IntervalTreeNode};

pub fn search_interval<'a, T>(
    root: &'a IntervalTreeNode<T>,
    ql: u32,
    qr: u32,
    inclusive: bool,
) -> Vec<(u32, u32, &'a T)> {
    let mut out = Vec::new();
    search_rec(Some(root), ql, qr, inclusive, &mut out);
    out
}

fn search_rec<'a, T>(
    node: Option<&'a IntervalTreeNode<T>>,
    ql: u32,
    qr: u32,
    inclusive: bool,
    out: &mut Vec<(u32, u32, &'a T)>,
) {
    let Some(n) = node else { return; };

    // Explore left if it can possibly overlap: left.max >= ql
    if let Some(ref left) = n.left_child {
        if left.max >= ql {
            search_rec(Some(left.as_ref()), ql, qr, inclusive, out);
        }
    }

    // Check current node
    if n.overlaps_range(ql, qr, inclusive) {
        out.push((n.node.left, n.node.right, &n.node.data));
    }

    // Explore right if keys there can start before qr
    let can_right = if inclusive { n.node.left <= qr } else { n.node.left < qr };
    if can_right {
        if let Some(ref right) = n.right_child {
            search_rec(Some(right.as_ref()), ql, qr, inclusive, out);
        }
    }
}

