use crate::node::IntervalTreeNode;

pub fn search_interval<'a, T>(
    root: &'a IntervalTreeNode<T>,
    ql: u32,
    qr: u32,
    inclusive: bool,
) -> Vec<(u32, u32, &'a T)> {
    debug_assert!(ql <= qr, "invalid query range");

    let mut out: Vec<(u32, u32, &'a T)> = Vec::new();
    let mut stack: Vec<&'a IntervalTreeNode<T>> = Vec::with_capacity(64);
    stack.push(root);

    while let Some(n) = stack.pop() {
        // Left: only if it can possibly overlap
        if let Some(left) = n.left_child.as_deref() {
            if left.max >= ql {
                stack.push(left);
            }
        }

        // Current node
        if n.overlaps_range(ql, qr, inclusive) {
            out.push((n.node.left, n.node.right, &n.node.data));
        }

        // Right: possible overlaps if the subtree can start before qr
        let can_right = if inclusive { n.node.left <= qr } else { n.node.left < qr };
        if can_right {
            if let Some(right) = n.right_child.as_deref() {
                stack.push(right);
            }
        }
    }

    out
}
