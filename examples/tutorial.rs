//! Tutorial: Self-balancing AVL tree for fleet agent priority queues

use avl_tree_rs::AvlTree;

fn main() {
    println!("=== AVL Tree Tutorial ===\n");

    // Part 1: Basic operations
    println!("Part 1: Building a priority queue");
    let mut tree: AvlTree<i32> = AvlTree::new();
    
    // Insert agents by priority (lower number = higher priority)
    let priorities = [42, 17, 85, 23, 91, 8, 56, 31, 67, 3];
    for p in &priorities {
        tree.insert(*p);
    }
    println!("  Inserted {:?} (duplicates return false)", priorities);
    println!("  Tree size: {}", tree.len());
    println!("  Tree height: {} (balanced AVL)", tree.height());
    println!("  Balance factor at root: {}", tree.balance_factor_root());
    println!("  Sorted order: {:?}", tree.inorder());
    println!();

    // Part 2: Delete and rebalance
    println!("Part 2: Delete and automatic rebalance");
    tree.delete(&42);
    tree.delete(&17);
    println!("  After deleting 42, 17:");
    println!("  Size: {}, Height: {}", tree.len(), tree.height());
    println!("  Still sorted: {:?}", tree.inorder());
    println!();

    // Part 3: Range queries via inorder
    println!("Part 3: Finding agents in priority range");
    let sorted = tree.inorder();
    let range: Vec<&i32> = sorted.iter().filter(|&&v| v >= 20 && v <= 60).collect();
    println!("  Priorities 20-60: {:?}", range);
    println!();

    // Part 4: String keys — agent registry
    println!("Part 4: Agent name registry");
    let mut agents: AvlTree<String> = AvlTree::new();
    for name in &["forgemaster", "oracle2", "builder-a", "conductor", "auditor"] {
        agents.insert(name.to_string());
    }
    println!("  Agent order: {:?}", agents.inorder());
    println!("  Contains 'oracle2': {}", agents.contains(&"oracle2".to_string()));
    println!("  Contains 'unknown': {}", agents.contains(&"unknown".to_string()));
}
