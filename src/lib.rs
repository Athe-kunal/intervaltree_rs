pub mod node;
pub mod build;
pub mod search;
pub mod insert;
pub mod delete;
pub mod python;

use node::{IntervalTreeNode, Node};

pub type Tree<T> = Option<Box<IntervalTreeNode<T>>>;

/// Assert core treap/interval invariants. Panic with a helpful message if something is wrong.
fn assert_invariants<T>(root: &Tree<T>) {
    fn check<T>(n: &Option<Box<IntervalTreeNode<T>>>) -> (u32, u64, (u32, u32)) {
        let Some(node) = n.as_ref() else { return (0, u64::MAX, (u32::MIN, u32::MIN)); };

        // Recurse
        let (lmax, _lpriority, lkey) = check(&node.left_child);
        let (rmax, _rpriority, rkey) = check(&node.right_child);

        // BST invariant: left < node < right (lexicographic by (left,right))
        let mykey = (node.node.left, node.node.right);
        if node.left_child.is_some() {
            assert!(lkey < mykey, "BST violated: left child key {:?} !< parent {:?}", lkey, mykey);
        }
        if node.right_child.is_some() {
            assert!(mykey < rkey, "BST violated: parent {:?} !< right child key {:?}", mykey, rkey);
        }

        // Heap (treap) invariant: parent.priority <= child.priority (min-heap on priority)
        if let Some(lc) = node.left_child.as_ref() {
            assert!(
                node.priority <= lc.priority,
                "Heap violated: parent priority {} > left priority {} at {:?}",
                node.priority, lc.priority, mykey
            );
        }
        if let Some(rc) = node.right_child.as_ref() {
            assert!(
                node.priority <= rc.priority,
                "Heap violated: parent priority {} > right priority {} at {:?}",
                node.priority, rc.priority, mykey
            );
        }

        // max augmentation invariant
        let expected_max = node
            .node
            .right
            .max(lmax)
            .max(rmax);
        assert!(
            node.max == expected_max,
            "Max violated at {:?}: stored {}, expected {} (lmax={}, rmax={})",
            mykey, node.max, expected_max, lmax, rmax
        );

        (expected_max, node.priority, mykey)
    }

    let _ = check(root);
}


fn boxed_interval<T>(l: u32, r: u32, data: T) -> Box<IntervalTreeNode<T>>{
    let n = Node::new(l, r, data).expect("Valid interval");
    let t = IntervalTreeNode::new(n).expect("node");
    Box::new(t)
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::insert::insert_treap;
    use crate::delete::delete_treap;
    use crate::search;

    fn build_sample() -> Tree<&'static str> {
        let items = [
            (15, 20, "A"),
            (10, 30, "B"),
            (17, 19, "C"),
            (5,  20, "D"),
            (12, 15, "E"),
            (30, 40, "F"),
        ];
        let mut root: Tree<&'static str> = None;
        for (l, r, d) in items {
            root = insert_treap(root, boxed_interval(l, r, d));
            assert_invariants(&root);
        }
        root
    }

    fn hits_of<T>(v: &[(u32, u32, &T)]) -> Vec<(u32,u32)> {
        let mut x: Vec<_> = v.iter().map(|(l,r,_) | (*l, *r)).collect();
        x.sort_unstable();
        x
    }

    #[test]
    fn insert_preserves_invariants_and_keys(){
        let root = build_sample();
        assert_invariants(&root);

        let got = if let Some(n) = &root {
            search::search_interval(n, 0, 100, true)
        }else { vec![] };
        let got_pairs = hits_of(&got);

        let mut expect = vec![(5,20), (10,30), (12,15), (15,20), (17,19), (30,40)];
        expect.sort_unstable();
        assert_eq!(got_pairs, expect, "All inserted intervals should be present")
    }

    #[test]
    fn search_inclusive_vs_exclusive() {
        let root = build_sample();
        let n = root.as_ref().unwrap();

        let inc_hits = hits_of(&search::search_interval(n, 10, 18, true));
        assert!(inc_hits.contains(&(12,15)) && inc_hits.contains(&(15,20)),
            "inclusive search should include touching/overlapping ranges");

        // Query [21,29], inclusive=false: only strict overlaps
        let exc_hits = hits_of(&search::search_interval(n, 21, 29, false));
        // (10,30) overlaps strictly; (15,20) does not
        assert!(exc_hits.contains(&(10,30)) && !exc_hits.contains(&(15,20)));
    }
    #[test]
    fn delete_removes_key_and_keeps_invariants() {
        let mut root = build_sample();
        // Delete a leaf-ish, one-child, two-children sequence like in main
        for key in &[(12,15), (17,19), (10,30)] {
            root = delete_treap(root, *key);
            assert_invariants(&root);

            // Ensure deleted key is gone from hits
            if let Some(n) = &root {
                let hits = hits_of(&search::search_interval(n, 0, 100, true));
                assert!(!hits.contains(key), "deleted key {:?} should be absent", key);
            }
        }
    }
    #[test]
    fn search_on_empty_tree_is_empty() {
        let root: Tree<&str> = None;
        let hits = if let Some(n) = &root { search::search_interval(n, 0, 100, true) } else { vec![] };
        assert!(hits.is_empty());
    }

    #[test]
    fn delete_nonexistent_keeps_structure() {
        let mut root = build_sample();
        let before = hits_of(&search::search_interval(root.as_ref().unwrap(), 0, 100, true));
        root = delete_treap(root, (42, 99)); // not present
        assert_invariants(&root);
        let after = hits_of(&search::search_interval(root.as_ref().unwrap(), 0, 100, true));
        assert_eq!(before, after);
    }

    #[test]
    fn inclusive_boundary_touching_behaves_as_expected() {
        let root = build_sample();
        let n = root.as_ref().unwrap();
        // Touching at boundary: [12,15) vs query ending at 15
        let inc = hits_of(&search::search_interval(n, 0, 15, true));
        let exc = hits_of(&search::search_interval(n, 0, 15, false));
        println!("Excluded {:?}",exc);
        assert!(inc.contains(&(15,20)));
        assert!(!exc.contains(&(15,20)));
    }
}
