use crate::{
    node::{IntervalTreeNode, Node},
    insert::insert_treap,
    delete::delete_treap,
};

pub mod node;
pub mod build;
pub mod search;
pub mod insert;
pub mod delete;

// A treap root is an Option<Box<Node>>
type Tree<T> = Option<Box<IntervalTreeNode<T>>>;

fn main() {
    // ---------- 1) BUILD by repeated treap inserts ----------
    // (You can still keep your build::build_tree; here we build via treap to exercise rotations.)
    let items: Vec<(u32, u32, &str)> = vec![
        (15, 20, "A"),
        (10, 30, "B"),
        (17, 19, "C"),
        (5, 20,  "D"),
        (12, 15, "E"),
        (30, 40, "F"),
    ];

    let mut root: Tree<&str> = None;
    for (l, r, data) in items {
        root = insert_treap(root, boxed_interval(l, r, data));
        assert_invariants(&root);
    }

    println!("\n== After initial inserts ==");
    print_tree(&root, 0);

    // ---------- 2) SEARCH tests ----------
    println!("\n== Search tests ==");
    search_and_print(&root, 10, 18, true);
    search_and_print(&root, 21, 29, false);
    search_and_print(&root, 0, 6, false);
    search_and_print(&root, 30, 35, true);

    // ---------- 3) DELETE tests ----------
    // Delete a leaf-ish key, a node with one child, and a node with two children.
    // Keys are (left, right) pairs used for BST ordering.
    let deletions = vec![(12, 15), (17, 19), (10, 30)];
    for key in deletions {
        println!("\n== Deleting {:?} ==", key);
        root = delete_treap(root, key);
        assert_invariants(&root);
        print_tree(&root, 0);
    }

    // More searches after deletions
    println!("\n== Search after deletions ==");
    search_and_print(&root, 10, 18, true);
    search_and_print(&root, 28, 41, true);

    // ---------- 4) Delete everything to test empty-case paths ----------
    let to_remove_all = vec![(5, 20), (15, 20), (30, 40)];
    for key in to_remove_all {
        root = delete_treap(root, key);
        assert_invariants(&root);
    }
    println!("\n== After deleting all nodes ==");
    print_tree(&root, 0);

    // Searching an empty tree
    search_and_print(&root, 0, 100, true);
}

// ---------------------- Helpers ----------------------

/// Create a boxed interval node with (random or deterministic) priorityrity assigned by your IntervalTreeNode::new
fn boxed_interval<T>(left: u32, right: u32, data: T) -> Box<IntervalTreeNode<T>> {
    let n = Node::new(left, right, data).expect("valid interval");
    let t = IntervalTreeNode::new(n).expect("node");
    Box::new(t)
}

/// Pretty-print the tree showing (left,right), max, and priority
fn print_tree<T: std::fmt::Debug>(root: &Tree<T>, depth: usize) {
    if let Some(n) = root {
        print_tree(&n.right_child, depth + 1);
        println!(
            "{:indent$}└─ [{}, {}) max={} priority={} data={:?}",
            "",
            n.node.left,
            n.node.right,
            n.max,
            n.priority,
            n.node.data,
            indent = depth * 4
        );
        print_tree(&n.left_child, depth + 1);
    } else if depth == 0 {
        println!("(empty)");
    }
}

/// Run a search and print the overlapping intervals (adapts to your existing search signature)
fn search_and_print<T: std::fmt::Debug>(root: &Tree<T>, ql: u32, qr: u32, inclusive: bool) {
    let hits: Vec<(u32, u32, &T)> = if let Some(n) = root {
        search::search_interval(n, ql, qr, inclusive)
    } else {
        Vec::new()
    };
    println!(
        "query [{}, {}] inclusive={} -> {} hit(s): {:?}",
        ql,
        qr,
        inclusive,
        hits.len(),
        hits.iter()
            .map(|(l, r, _)| format!("[{}, {})", l, r))
            .collect::<Vec<_>>()
    );
}

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
