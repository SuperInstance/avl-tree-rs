# avl-tree-rs

[![crates.io](https://img.shields.io/crates/v/avl-tree-rs.svg)](https://crates.io/crates/avl-tree-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

AVL self-balancing binary search tree in Rust — LL, RR, LR, and RL rotations with guaranteed O(log n) height.

## The Problem

A plain binary search tree degrades to O(n) when keys are inserted in sorted or nearly-sorted order. This happens often in practice: sequential IDs, timestamps, auto-incrementing counters. You need a tree that stays balanced regardless of insertion order, without paying the overhead of a B-tree's wide nodes.

## The Insight

The AVL tree (Adelson-Velsky and Landis, 1962) maintains the invariant: **for every node, the heights of its left and right subtrees differ by at most 1.** When an insertion or deletion violates this invariant (balance factor leaves the range `[-1, 0, 1]`), a local rotation restores it.

There are four cases:
- **LL** (left-left): Left child is left-heavy → single right rotation
- **RR** (right-right): Right child is right-heavy → single left rotation
- **LR** (left-right): Left child is right-heavy → left rotation on left child, then right rotation
- **RL** (right-left): Right child is left-heavy → right rotation on right child, then left rotation

Each rotation is O(1) — just pointer rewiring — and at most one rotation (double rotation counts as two) is needed per level. So rebalancing costs O(log n) total.

## How It Works

The tree uses `Option<Box<Node<T>>>` links — a classic owned recursive structure. Each node stores its height, enabling O(1) balance factor computation.

**Insert:** Recursive descent to find the insertion point, then propagate back up, calling `rebalance()` at each ancestor. The rebalance function checks the balance factor and applies the appropriate rotation.

**Delete:** Recursive descent to find the node. Three sub-cases for the found node:
- Leaf → remove directly
- One child → replace with child
- Two children → swap with in-order successor (minimum of right subtree), then delete the successor

Rebalancing propagates back up the recursion stack. Deletion can trigger rotations at multiple levels (unlike insertion, which triggers at most one).

**Contains:** Iterative descent from root — no recursion needed for lookups.

## Usage

```rust
use avl_tree_rs::AvlTree;  // crate name on crates.io; lib name is avl_tree

let mut tree = AvlTree::new();

// Insert — returns false for duplicates
assert!(tree.insert(5));
assert!(tree.insert(3));
assert!(tree.insert(7));
assert!(!tree.insert(5));  // duplicate, returns false

// Search
assert!(tree.contains(&3));
assert!(!tree.contains(&4));

// Sorted traversal
assert_eq!(tree.inorder(), vec![&3, &5, &7]);

// Delete
assert!(tree.delete(&5));
assert!(!tree.contains(&5));
assert_eq!(tree.len(), 2);

// Balance information
println!("height: {}, balance_factor: {}", tree.height(), tree.balance_factor_root());

// Works with any Ord type
let mut str_tree = AvlTree::new();
str_tree.insert("banana");
str_tree.insert("apple");
str_tree.insert("cherry");
assert_eq!(str_tree.inorder(), vec![&"apple", &"banana", &"cherry"]);
```

## Module Map

All types in the crate root (`src/lib.rs`):

| Type | Description |
|---|---|
| `AvlTree<T: Ord>` | The AVL tree. `new`, `insert`, `delete`, `contains`, `inorder`, `len`, `is_empty`, `height`, `balance_factor_root` |

Internal (private): `Node<T>`, rotation functions, recursive helpers.

## Design Decisions

- **Owned `Box<Node>` links.** Each node is heap-allocated. This is idiomatic Rust for recursive tree structures. The `take()` + rebalance + reassign pattern avoids borrow-checker issues with recursive mutation.
- **Recursive insert/delete.** The implementation uses recursive functions that return `(Link<T>, bool)` tuples — the new subtree root and a success flag. This avoids maintaining parent pointers or using a zipper.
- **Height stored, not computed.** Each node caches its height. Computing height from children would add O(n) traversals. Caching makes balance factor checks O(1).
- **In-order successor for deletion.** When deleting a node with two children, the implementation finds the minimum of the right subtree (in-order successor), swaps the key, and recursively deletes the successor. This is the standard textbook approach.
- **No iterators (yet).** `inorder()` returns `Vec<&T>`. A lazy iterator would avoid the allocation but adds implementation complexity for a tree structure.

## Complexity

| Operation | Time | Notes |
|---|---|---|
| `insert` | O(log n) | At most one (double) rotation |
| `delete` | O(log n) | May trigger rotations at multiple levels |
| `contains` | O(log n) | Iterative — no recursion overhead |
| `inorder` | O(n) | Allocates a `Vec` |
| `height` | O(1) | Cached at root |
| Space | O(n) | 3 words overhead per node (key, left, right, height) |

## Limitations

- **No generic key-value pairs.** The tree stores keys only. For a map, you'd need to modify `Node` to hold a `(K, V)` pair and compare by `K`.
- **No range queries.** Only exact lookup and full in-order traversal. Adding range queries would require a method that prunes subtrees outside the range.
- **No concurrent access.** `&mut self` on every mutation. For concurrent use, wrap in a `Mutex` or use a lock-free tree.
- **Recursive deletion.** Deep trees (millions of elements) may hit stack overflow in debug builds. Release builds typically have sufficient stack or use stack growth.

## Status

Published to [crates.io](https://crates.io/crates/avl-tree-rs). A clean, educational AVL implementation suitable for ordered sets where you need guaranteed O(log n) operations and want to understand the rotation mechanics. For production key-value storage, `std::collections::BTreeMap` is almost always the better choice — it's heavily optimized and battle-tested.
