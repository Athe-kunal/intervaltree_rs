// tests/treap_integration.rs
use intervaltree_rs::{insert::insert_treap, delete::delete_treap};
use intervaltree_rs::{node::IntervalTreeNode, Tree};
use intervaltree_rs::search;

fn boxed_interval<T>(l: u32, r: u32, data: T) -> Box<IntervalTreeNode<T>> {
    // If not public, expose a small test-only constructor in your crate root behind cfg(test)
    let n = intervaltree_rs::node::Node::new(l, r, data).unwrap();
    let t = IntervalTreeNode::new(n).unwrap();
    Box::new(t)
}

#[test]
fn end_to_end_insert_search_delete() {
    let mut root: Tree<&str> = None;
    for (l, r, d) in [(1,3,"x"), (5,9,"y"), (2,8,"z")] {
        root = insert_treap(root, boxed_interval(l, r, d));
    }
    let n = root.as_ref().unwrap();
    let hits = search::search_interval(n, 3, 5, true);
    let pairs: std::collections::BTreeSet<_> =
        hits.into_iter().map(|(l, r, _)| (l, r)).collect();
    assert!(pairs.contains(&(1,3)) && pairs.contains(&(2,8)) && pairs.contains(&(5,9)));

    // delete one and confirm it’s gone
    root = delete_treap(root, (2,8));
    let n = root.as_ref().unwrap();
    let pairs: std::collections::BTreeSet<_> =
        search::search_interval(n, 0, 10, true).into_iter().map(|(l,r,_)|(l,r)).collect();
    assert!(!pairs.contains(&(2,8)));
}
