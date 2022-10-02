use std::cmp::max;

#[cfg(test)]
mod tests {
    use super::*;

    /// Should be able to create a new AVL tree with no elements.
    #[test]
    fn test_new() {
        let tree: Tree<i32> = Tree::new();
        assert!(tree.is_empty());
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> Tree<T> {
    /// Create a new AVL tree.
    pub fn new() -> Tree<T> {
        Tree { root: None }
    }

    /// Check if the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

fn height<T: Ord>(node: &Option<Box<Node<T>>>) -> i32 {
    match *node {
        Some(ref n) => n.height,
        None => 0,
    }
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
