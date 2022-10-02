use std::cmp::{max, Ordering};

#[cfg(test)]
mod tests {
    use super::*;

    /// Should be able to create a new AVL tree with no elements.
    #[test]
    fn test_new() {
        let tree: Tree<i32> = Tree::new();
        assert!(tree.is_empty());
    }

    // Should be able to insert a new element into an empty tree.
    #[test]
    fn test_insert() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(1);
        assert!(!tree.is_empty());
    }

    // Should be able to insert multiple elements into an empty tree.
    #[test]
    fn test_insert_multiple() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert!(!tree.is_empty());
    }

    // Should return an vector of elements in order.
    #[test]
    fn test_in_order() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        assert_eq!(vec![1, 2, 3], tree.in_order());
    }

    // should perform a left rotation
    #[test]
    fn test_insert_balance() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        assert_eq!(vec![2, 1, 3], tree.pre_order());
    }

    // should perform a right rotation
    #[test]
    fn test_insert_balance2() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        assert_eq!(vec![2, 1, 3], tree.pre_order());
    }

    // should perform a left-right rotation
    #[test]
    fn test_insert_balance3() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);

        assert_eq!(vec![2, 1, 3], tree.pre_order());
    }

    // should perform a right-left rotation
    #[test]
    fn test_insert_balance4() {
        let mut tree: Tree<i32> = Tree::new();
        tree.insert(1);
        tree.insert(3);
        tree.insert(2);

        assert_eq!(vec![2, 1, 3], tree.pre_order());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Clone> Tree<T> {
    /// Create a new AVL tree with an empty root.
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Insert a new element into the tree.
    pub fn insert(&mut self, value: T) {
        self.root = insert_recursive(self.root.take(), value);
    }

    pub fn in_order(&self) -> Vec<T> {
        let mut result = Vec::new();
        in_order_recursive(&self.root, &mut result);
        result
    }

    pub fn pre_order(&self) -> Vec<T> {
        let mut result = Vec::new();
        pre_order_recursive(&self.root, &mut result);
        result
    }
}

fn in_order_recursive<T: Ord + Clone>(node: &Option<Box<Node<T>>>, result: &mut Vec<T>) {
    if let Some(ref n) = *node {
        in_order_recursive(&n.left, result);
        result.push(n.key.clone());
        in_order_recursive(&n.right, result);
    }
}

fn pre_order_recursive<T: Ord + Clone>(node: &Option<Box<Node<T>>>, result: &mut Vec<T>) {
    if let Some(ref n) = *node {
        result.push(n.key.clone());
        pre_order_recursive(&n.left, result);
        pre_order_recursive(&n.right, result);
    }
}

fn insert_recursive<T: Ord + Clone>(root: Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>> {
    match root {
        None => Some(Box::new(Node::new(value))),

        Some(mut node) => match value.cmp(&node.key) {
            Ordering::Less | Ordering::Equal => {
                node.left = insert_recursive(node.left, value.clone());

                if should_balance(&node.left, &node.right) {
                    if value < node.left.as_ref().unwrap().key {
                        node = single_rotate_left(node);
                    } else {
                        node = double_rotate_left(node);
                    }
                }

                update_height(&mut node);
                Some(node)
            }

            Ordering::Greater => {
                node.right = insert_recursive(node.right, value.clone());

                if should_balance(&node.right, &node.left) {
                    if value > node.right.as_ref().unwrap().key {
                        node = single_rotate_right(node);
                    } else {
                        node = double_rotate_right(node);
                    }
                }

                update_height(&mut node);
                Some(node)
            }
        },
    }
}

fn should_balance<T: Ord>(node1: &Option<Box<Node<T>>>, node2: &Option<Box<Node<T>>>) -> bool {
    const MAX_IMBALANCE: i32 = 1;
    height(&node1) - height(&node2) > MAX_IMBALANCE
}

fn double_rotate_left<T: Ord>(mut z: Box<Node<T>>) -> Box<Node<T>> {
    let y = z.left.take().unwrap();
    z.left = Some(single_rotate_right(y));

    single_rotate_left(z)
}

fn double_rotate_right<T: Ord>(mut z: Box<Node<T>>) -> Box<Node<T>> {
    let y = z.right.take().unwrap();
    z.right = Some(single_rotate_left(y));

    single_rotate_right(z)
}

fn single_rotate_left<T: Ord>(mut x: Box<Node<T>>) -> Box<Node<T>> {
    let mut w = x.left.unwrap();
    x.left = w.right;

    update_height(&mut x);
    w.right = Some(x);

    update_height(&mut w);
    w
}

fn single_rotate_right<T: Ord>(mut w: Box<Node<T>>) -> Box<Node<T>> {
    let mut x = w.right.unwrap();
    w.right = x.left;

    update_height(&mut w);
    x.left = Some(w);

    update_height(&mut x);
    x
}

fn update_height<T: Ord>(node: &mut Box<Node<T>>) {
    const ITSELF: i32 = 1;

    let left_height = height(&node.left);
    let right_height = height(&node.right);

    node.height = max(left_height, right_height) + ITSELF;
}

fn height<T: Ord>(node: &Option<Box<Node<T>>>) -> i32 {
    const LEAF_HEIGHT: i32 = -1;

    node.as_ref().map(|n| n.height).unwrap_or(LEAF_HEIGHT)
}

#[derive(Debug, PartialEq, Clone)]
struct Node<T: Ord> {
    key: T,
    height: i32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: Ord> Node<T> {
    fn new(key: T) -> Self {
        Node {
            key,
            height: 0,
            left: None,
            right: None,
        }
    }
}
