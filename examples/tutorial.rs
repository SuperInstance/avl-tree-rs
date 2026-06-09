//! # AVL Tree Tutorial
//!
//! A progressive walkthrough of the `avl_tree` crate — a zero-dependency AVL
//! self-balancing binary search tree with O(log n) insert, delete, and lookup.
//!
//! Run:
//!
//!     cargo run --example tutorial

use avl_tree_rs::AvlTree;

fn main() {
    println!("=== AVL Tree Tutorial ===\n");

    lesson_1_create_and_insert();
    lesson_2_lookup();
    lesson_3_inorder_traversal();
    lesson_4_delete();
    lesson_5_balance_rotations();
    lesson_6_string_keys();
    lesson_7_stress_test();

    println!("\n✅ All lessons complete!");
}

// ── Lesson 1: Create and Insert ───────────────────────────────────────────────
//
// An AVL tree starts empty. `insert()` returns `true` when the key is new and
// `false` when it's a duplicate (the tree is unchanged).

fn lesson_1_create_and_insert() {
    println!("--- Lesson 1: Create and Insert ---");

    let mut tree: AvlTree<i32> = AvlTree::new();

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);

    println!("  Inserting 5, 3, 7, 1, 9...");
    for &v in &[5, 3, 7, 1, 9] {
        let was_new = tree.insert(v);
        println!("    insert({}) → {} (new={})", v, was_new, was_new);
    }

    println!("  Size: {}", tree.len());
    println!("  Empty? {}", tree.is_empty());

    // Duplicate insertion returns false, size stays the same
    assert!(!tree.insert(5));
    assert_eq!(tree.len(), 5);
    println!("  Inserting duplicate 5 → false, size still {}", tree.len());

    println!();
}

// ── Lesson 2: Lookup with contains() ─────────────────────────────────────────
//
// `contains()` performs O(log n) search. It works on an immutable reference
// so you can query a shared tree.

fn lesson_2_lookup() {
    println!("--- Lesson 2: Lookup with contains() ---");

    let mut tree = AvlTree::new();
    for &v in &[10, 20, 30, 40, 50] {
        tree.insert(v);
    }

    for &query in &[10, 25, 30, 99] {
        let found = tree.contains(&query);
        println!("  contains({}) → {}", query, found);
    }

    // Empty tree
    let empty: AvlTree<i32> = AvlTree::new();
    assert!(!empty.contains(&1));
    println!("  Empty tree contains(1) → false");

    println!();
}

// ── Lesson 3: In-Order Traversal ──────────────────────────────────────────────
//
// `inorder()` returns a `Vec<&T>` of all keys in ascending sorted order —
// guaranteed by the BST invariant.

fn lesson_3_inorder_traversal() {
    println!("--- Lesson 3: In-Order Traversal ---");

    let mut tree = AvlTree::new();

    // Insert in a deliberately unsorted order
    let values = [50, 30, 70, 10, 40, 60, 90, 5, 15];
    for &v in &values {
        tree.insert(v);
    }

    let sorted: Vec<i32> = tree.inorder().into_iter().copied().collect();
    println!("  Inserted: {:?}", values);
    println!("  In-order: {:?}", sorted);

    // Verify sorted
    let mut expected = values.to_vec();
    expected.sort();
    assert_eq!(sorted, expected);

    // Empty tree returns empty vec
    let empty: AvlTree<i32> = AvlTree::new();
    assert!(empty.inorder().is_empty());
    println!("  Empty tree inorder → []");

    println!();
}

// ── Lesson 4: Delete ─────────────────────────────────────────────────────────
//
// `delete()` removes a key and returns `true` if it was found. The tree
// rebalances automatically after removal.

fn lesson_4_delete() {
    println!("--- Lesson 4: Delete ---");

    let mut tree = AvlTree::new();
    for &v in &[20, 10, 30, 5, 15, 25, 35] {
        tree.insert(v);
    }
    println!("  After inserting 7 keys: size={}, inorder={:?}",
        tree.len(), tree.inorder().into_iter().copied().collect::<Vec<_>>());

    // Delete a leaf
    assert!(tree.delete(&5));
    println!("  delete(5) [leaf] → true,  size={}", tree.len());
    assert!(!tree.contains(&5));

    // Delete a node with one child
    assert!(tree.delete(&10));
    println!("  delete(10) [one child] → true, size={}", tree.len());

    // Delete a node with two children
    assert!(tree.delete(&30));
    println!("  delete(30) [two children] → true, size={}", tree.len());

    // Delete a nonexistent key
    assert!(!tree.delete(&999));
    println!("  delete(999) [not found] → false, size={}", tree.len());

    // Verify remaining keys are still sorted
    let remaining: Vec<i32> = tree.inorder().into_iter().copied().collect();
    println!("  Remaining (sorted): {:?}", remaining);
    let mut check = remaining.clone();
    check.sort();
    assert_eq!(remaining, check);

    println!();
}

// ── Lesson 5: Balance and Rotations ───────────────────────────────────────────
//
// AVL trees stay balanced through four rotation cases: LL, RR, LR, RL.
// This lesson demonstrates each case and shows how height stays logarithmic.

fn lesson_5_balance_rotations() {
    println!("--- Lesson 5: Balance and Rotations ---");

    // LL rotation: inserting in decreasing order (left-left heavy)
    let mut ll = AvlTree::new();
    for &v in &[30, 20, 10] {
        ll.insert(v);
    }
    println!("  LL case (inserted 30,20,10): height={}, inorder={:?}",
        ll.height(), ll.inorder().into_iter().copied().collect::<Vec<_>>());
    assert_eq!(ll.height(), 2);

    // RR rotation: inserting in increasing order (right-right heavy)
    let mut rr = AvlTree::new();
    for &v in &[10, 20, 30] {
        rr.insert(v);
    }
    println!("  RR case (inserted 10,20,30): height={}, inorder={:?}",
        rr.height(), rr.inorder().into_iter().copied().collect::<Vec<_>>());
    assert_eq!(rr.height(), 2);

    // LR rotation
    let mut lr = AvlTree::new();
    for &v in &[30, 10, 20] {
        lr.insert(v);
    }
    println!("  LR case (inserted 30,10,20): height={}, inorder={:?}",
        lr.height(), lr.inorder().into_iter().copied().collect::<Vec<_>>());
    assert_eq!(lr.height(), 2);

    // RL rotation
    let mut rl = AvlTree::new();
    for &v in &[10, 30, 20] {
        rl.insert(v);
    }
    println!("  RL case (inserted 10,30,20): height={}, inorder={:?}",
        rl.height(), rl.inorder().into_iter().copied().collect::<Vec<_>>());
    assert_eq!(rl.height(), 2);

    // Balance factor at root should always be in {-1, 0, 1}
    println!("  Root balance factors: LL={}, RR={}, LR={}, RL={}",
        ll.balance_factor_root(), rr.balance_factor_root(),
        lr.balance_factor_root(), rl.balance_factor_root());

    // Inserting 1..=7 in sorted order (worst case for naive BST) stays balanced
    let mut balanced = AvlTree::new();
    for v in 1..=7 {
        balanced.insert(v);
    }
    println!("  Inserted 1..=7 in order: height={} (unbalanced BST would be 7)",
        balanced.height());
    assert_eq!(balanced.height(), 3);

    println!();
}

// ── Lesson 6: Generic Keys (Strings, Tuples, Custom Types) ────────────────────
//
// `AvlTree<T>` works with any `T: Ord`. This lesson uses `String` keys.

fn lesson_6_string_keys() {
    println!("--- Lesson 6: String Keys ---");

    let mut tree: AvlTree<String> = AvlTree::new();

    let names = vec!["delta", "alpha", "charlie", "bravo", "echo"];
    for name in &names {
        tree.insert(name.to_string());
    }

    println!("  Inserted: {:?}", names);

    let sorted: Vec<&String> = tree.inorder();
    println!("  Sorted:   {:?}", sorted.into_iter().map(|s| s.as_str()).collect::<Vec<_>>());

    assert!(tree.contains(&"charlie".to_string()));
    assert!(!tree.contains(&"zulu".to_string()));

    // Delete by reference
    tree.delete(&"bravo".to_string());
    assert!(!tree.contains(&"bravo".to_string()));
    println!("  After deleting \"bravo\": size={}", tree.len());

    println!();
}

// ── Lesson 7: Stress Test ─────────────────────────────────────────────────────
//
// Insert 1,000 keys in reverse sorted order (worst case for plain BST), then
// delete every other key. The AVL tree stays balanced throughout.

fn lesson_7_stress_test() {
    println!("--- Lesson 7: Stress Test (1000 keys) ---");

    let n: i32 = 1000;
    let mut tree = AvlTree::new();

    // Insert in reverse order
    for v in (1..=n).rev() {
        tree.insert(v);
    }

    let height = tree.height();
    let theoretical_max = (n as f64).log2().ceil() as i32 + 1;
    println!("  Inserted {} keys in reverse order", n);
    println!("  Height: {} (log₂({}) ≈ {})", height, n, theoretical_max);
    assert!(height <= theoretical_max + 1, "AVL tree is not balanced!");

    // Verify sorted
    let sorted: Vec<i32> = tree.inorder().into_iter().copied().collect();
    assert_eq!(sorted.len(), n as usize);
    for window in sorted.windows(2) {
        assert!(window[0] < window[1]);
    }

    // Delete even numbers
    for v in (2..=n).step_by(2).map(|v| v as i32) {
        tree.delete(&v);
    }
    println!("  After deleting evens: size={}, height={}", tree.len(), tree.height());
    assert_eq!(tree.len(), (n / 2) as usize);

    // All remaining keys are odd
    let remaining: Vec<i32> = tree.inorder().into_iter().copied().collect();
    for &v in &remaining {
        assert!(v % 2 == 1, "found even number: {}", v);
    }

    // Root balance factor still valid
    let bf = tree.balance_factor_root();
    assert!(bf >= -1 && bf <= 1, "root balance factor out of range: {}", bf);
    println!("  Root balance factor: {} ✓", bf);

    println!();
}
