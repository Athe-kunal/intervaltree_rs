pub mod node;
pub mod tree;
pub mod build;


fn main() {
    // Example intervals (u32,u32) — adjust as you like
    let items: Vec<(u32, u32)> = vec![(15, 20), (10, 30), (17, 19), (5, 20), (12, 15), (30, 40)];

    // Build the tree
    let tree = build::build_tree(items);

    // Pretty-print the tree (requires Debug on your Tree/Node)
    println!("Built interval tree:");
    println!("{:#?}", tree);

    // Optional: demonstrate the empty-input panic is triggered
    let panicked = std::panic::catch_unwind(|| {
        // This should panic in your build_tree() due to empty input
        let _ = build::build_tree(Vec::new());
    })
    .is_err();

    println!("Empty input panicked as expected? {}", panicked);
}