type Link<T> = Option<Box<Node<T>>>;

struct Node<T: Ord> {
    key: T,
    left: Link<T>,
    right: Link<T>,
    height: i32,
}

impl<T: Ord> Node<T> {
    fn new(key: T) -> Box<Self> {
        Box::new(Node {
            key,
            left: None,
            right: None,
            height: 1,
        })
    }
}

fn link_height<T: Ord>(link: &Link<T>) -> i32 {
    link.as_ref().map_or(0, |n| n.height)
}

fn update_height<T: Ord>(n: &mut Node<T>) {
    n.height = 1 + link_height(&n.left).max(link_height(&n.right));
}

fn bf<T: Ord>(n: &Node<T>) -> i32 {
    link_height(&n.left) - link_height(&n.right)
}

fn rotate_right<T: Ord>(mut z: Box<Node<T>>) -> Box<Node<T>> {
    let mut y = z.left.take().unwrap();
    z.left = y.right.take();
    update_height(&mut z);
    y.right = Some(z);
    update_height(&mut y);
    y
}

fn rotate_left<T: Ord>(mut z: Box<Node<T>>) -> Box<Node<T>> {
    let mut y = z.right.take().unwrap();
    z.right = y.left.take();
    update_height(&mut z);
    y.left = Some(z);
    update_height(&mut y);
    y
}

fn rebalance<T: Ord>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    update_height(&mut node);
    let balance = bf(&node);
    if balance == 2 {
        // Left heavy
        if bf(node.left.as_ref().unwrap()) == -1 {
            // LR case: rotate left on left child first
            let left = node.left.take().unwrap();
            node.left = Some(rotate_left(left));
        }
        // LL case
        rotate_right(node)
    } else if balance == -2 {
        // Right heavy
        if bf(node.right.as_ref().unwrap()) == 1 {
            // RL case: rotate right on right child first
            let right = node.right.take().unwrap();
            node.right = Some(rotate_right(right));
        }
        // RR case
        rotate_left(node)
    } else {
        node
    }
}

fn insert_rec<T: Ord>(link: Link<T>, key: T) -> (Link<T>, bool) {
    match link {
        None => (Some(Node::new(key)), true),
        Some(mut node) => {
            let inserted;
            if key < node.key {
                let (new_left, ins) = insert_rec(node.left.take(), key);
                node.left = new_left;
                inserted = ins;
            } else if key > node.key {
                let (new_right, ins) = insert_rec(node.right.take(), key);
                node.right = new_right;
                inserted = ins;
            } else {
                // Duplicate
                return (Some(node), false);
            }
            (Some(rebalance(node)), inserted)
        }
    }
}

fn pop_min<T: Ord>(mut node: Box<Node<T>>) -> (T, Link<T>) {
    if node.left.is_none() {
        (node.key, node.right)
    } else {
        let (min_key, new_left) = pop_min(node.left.take().unwrap());
        node.left = new_left;
        (min_key, Some(rebalance(node)))
    }
}

fn delete_rec<T: Ord>(link: Link<T>, key: &T) -> (Link<T>, bool) {
    match link {
        None => (None, false),
        Some(mut node) => {
            if *key < node.key {
                let (new_left, deleted) = delete_rec(node.left.take(), key);
                node.left = new_left;
                (Some(rebalance(node)), deleted)
            } else if *key > node.key {
                let (new_right, deleted) = delete_rec(node.right.take(), key);
                node.right = new_right;
                (Some(rebalance(node)), deleted)
            } else {
                // Found the node to delete
                match (node.left.take(), node.right.take()) {
                    (None, right) => (right, true),
                    (left, None) => (left, true),
                    (left, right) => {
                        // Both children present: replace with in-order successor (min of right subtree)
                        let (min_key, new_right) = pop_min(right.unwrap());
                        let _ = std::mem::replace(&mut node.key, min_key);
                        node.left = left;
                        node.right = new_right;
                        (Some(rebalance(node)), true)
                    }
                }
            }
        }
    }
}

fn inorder_rec<'a, T: Ord>(link: &'a Link<T>, result: &mut Vec<&'a T>) {
    if let Some(node) = link {
        inorder_rec(&node.left, result);
        result.push(&node.key);
        inorder_rec(&node.right, result);
    }
}

/// AVL self-balancing binary search tree.
pub struct AvlTree<T: Ord> {
    root: Link<T>,
    size: usize,
}

impl<T: Ord> AvlTree<T> {
    /// Creates a new, empty AVL tree.
    pub fn new() -> Self {
        AvlTree { root: None, size: 0 }
    }

    /// Inserts a key. Returns `true` if the key was newly inserted, `false` if it was a duplicate.
    pub fn insert(&mut self, key: T) -> bool {
        let (new_root, inserted) = insert_rec(self.root.take(), key);
        self.root = new_root;
        if inserted {
            self.size += 1;
        }
        inserted
    }

    /// Deletes a key. Returns `true` if the key was found and removed.
    pub fn delete(&mut self, key: &T) -> bool {
        let (new_root, deleted) = delete_rec(self.root.take(), key);
        self.root = new_root;
        if deleted {
            self.size -= 1;
        }
        deleted
    }

    /// Returns `true` if the key is in the tree.
    pub fn contains(&self, key: &T) -> bool {
        let mut current = &self.root;
        while let Some(node) = current {
            if *key < node.key {
                current = &node.left;
            } else if *key > node.key {
                current = &node.right;
            } else {
                return true;
            }
        }
        false
    }

    /// Returns a sorted vector of references to all keys in ascending order.
    pub fn inorder(&self) -> Vec<&T> {
        let mut result = Vec::with_capacity(self.size);
        inorder_rec(&self.root, &mut result);
        result
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns `true` if the tree has no elements.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns the height of the tree (0 for empty tree).
    pub fn height(&self) -> i32 {
        link_height(&self.root)
    }

    /// Returns the balance factor of the root node (left_height - right_height).
    /// Returns 0 for an empty tree. Should be in [-1, 0, 1] for a valid AVL tree.
    pub fn balance_factor_root(&self) -> i32 {
        self.root.as_ref().map_or(0, |n| bf(n))
    }
}

impl<T: Ord> Default for AvlTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let tree: AvlTree<i32> = AvlTree::new();
        assert!(tree.is_empty());
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.height(), 0);
    }

    #[test]
    fn test_insert_contains() {
        let mut tree = AvlTree::new();
        tree.insert(5);
        assert!(tree.contains(&5));
    }

    #[test]
    fn test_contains_miss_empty() {
        let tree: AvlTree<i32> = AvlTree::new();
        assert!(!tree.contains(&1));
    }

    #[test]
    fn test_contains_miss_after_inserts() {
        let mut tree = AvlTree::new();
        for &v in &[1, 3, 5] {
            tree.insert(v);
        }
        assert!(!tree.contains(&2));
    }

    #[test]
    fn test_insert_returns_true_new() {
        let mut tree = AvlTree::new();
        assert!(tree.insert(42));
    }

    #[test]
    fn test_insert_returns_false_dup() {
        let mut tree = AvlTree::new();
        tree.insert(10);
        assert!(!tree.insert(10));
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_inorder_empty() {
        let tree: AvlTree<i32> = AvlTree::new();
        assert_eq!(tree.inorder(), Vec::<&i32>::new());
    }

    #[test]
    fn test_inorder_single() {
        let mut tree = AvlTree::new();
        tree.insert(7);
        assert_eq!(tree.inorder(), vec![&7]);
    }

    #[test]
    fn test_inorder_sorted() {
        let mut tree = AvlTree::new();
        for &v in &[5, 3, 7, 1, 4, 6, 8] {
            tree.insert(v);
        }
        assert_eq!(tree.inorder(), vec![&1, &3, &4, &5, &6, &7, &8]);
    }

    #[test]
    fn test_len_increments() {
        let mut tree = AvlTree::new();
        for &v in &[10, 20, 30, 40, 50] {
            tree.insert(v);
        }
        assert_eq!(tree.len(), 5);
    }

    #[test]
    fn test_len_no_dup() {
        let mut tree = AvlTree::new();
        tree.insert(99);
        tree.insert(99);
        assert_eq!(tree.len(), 1);
    }

    #[test]
    fn test_delete_empty() {
        let mut tree: AvlTree<i32> = AvlTree::new();
        assert!(!tree.delete(&5));
    }

    #[test]
    fn test_delete_leaf() {
        let mut tree = AvlTree::new();
        for &v in &[2, 1, 3] {
            tree.insert(v);
        }
        assert!(tree.delete(&1));
        assert_eq!(tree.len(), 2);
        assert!(!tree.contains(&1));
    }

    #[test]
    fn test_delete_one_child() {
        let mut tree = AvlTree::new();
        for &v in &[2, 1, 3, 4] {
            tree.insert(v);
        }
        assert!(tree.delete(&3));
        assert!(tree.contains(&4));
        assert!(!tree.contains(&3));
    }

    #[test]
    fn test_delete_two_children() {
        let mut tree = AvlTree::new();
        for &v in &[4, 2, 6, 1, 3, 5, 7] {
            tree.insert(v);
        }
        assert!(tree.delete(&4));
        let inorder = tree.inorder();
        let vals: Vec<i32> = inorder.iter().map(|&&v| v).collect();
        let mut sorted = vals.clone();
        sorted.sort();
        assert_eq!(vals, sorted);
        assert!(!tree.contains(&4));
    }

    #[test]
    fn test_delete_root_single() {
        let mut tree = AvlTree::new();
        tree.insert(5);
        assert!(tree.delete(&5));
        assert!(tree.is_empty());
    }

    #[test]
    fn test_delete_nonexistent() {
        let mut tree = AvlTree::new();
        tree.insert(1);
        tree.insert(2);
        assert!(!tree.delete(&99));
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn test_delete_inorder_sorted() {
        let mut tree = AvlTree::new();
        for v in 1..=10 {
            tree.insert(v);
        }
        tree.delete(&5);
        let inorder = tree.inorder();
        let vals: Vec<i32> = inorder.iter().map(|&&v| v).collect();
        let mut sorted = vals.clone();
        sorted.sort();
        assert_eq!(vals, sorted);
    }

    #[test]
    fn test_ll_rotation() {
        let mut tree = AvlTree::new();
        for &v in &[3, 2, 1] {
            tree.insert(v);
        }
        assert_eq!(tree.height(), 2);
        assert_eq!(tree.inorder(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_rr_rotation() {
        let mut tree = AvlTree::new();
        for &v in &[1, 2, 3] {
            tree.insert(v);
        }
        assert_eq!(tree.height(), 2);
        assert_eq!(tree.inorder(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_lr_rotation() {
        let mut tree = AvlTree::new();
        for &v in &[3, 1, 2] {
            tree.insert(v);
        }
        assert_eq!(tree.height(), 2);
        assert_eq!(tree.inorder(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_rl_rotation() {
        let mut tree = AvlTree::new();
        for &v in &[1, 3, 2] {
            tree.insert(v);
        }
        assert_eq!(tree.height(), 2);
        assert_eq!(tree.inorder(), vec![&1, &2, &3]);
    }

    #[test]
    fn test_height_balanced() {
        let mut tree = AvlTree::new();
        for v in 1..=7 {
            tree.insert(v);
        }
        assert_eq!(tree.height(), 3);
    }

    #[test]
    fn test_mass_insert_inorder() {
        let mut tree = AvlTree::new();
        for v in (1..=100).rev() {
            tree.insert(v);
        }
        let inorder = tree.inorder();
        let vals: Vec<i32> = inorder.iter().map(|&&v| v).collect();
        let expected: Vec<i32> = (1..=100).collect();
        assert_eq!(vals, expected);
        assert!(tree.height() <= 14);
    }

    #[test]
    fn test_mass_delete() {
        let mut tree = AvlTree::new();
        for v in 1..=50 {
            tree.insert(v);
        }
        for v in (2..=50).step_by(2) {
            tree.delete(&v);
        }
        assert_eq!(tree.len(), 25);
        let inorder = tree.inorder();
        let vals: Vec<i32> = inorder.iter().map(|&&v| v).collect();
        for &v in &vals {
            assert_eq!(v % 2, 1, "expected only odd numbers, found {}", v);
        }
    }

    #[test]
    fn test_balance_after_mass_ops() {
        let mut tree = AvlTree::new();
        for v in 1..=200 {
            tree.insert(v);
        }
        for v in 1..=100 {
            tree.delete(&v);
        }
        let bf = tree.balance_factor_root();
        assert!(bf >= -1 && bf <= 1, "balance factor out of range: {}", bf);
    }
}
