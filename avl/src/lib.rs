#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree = Tree::new();
        assert!(tree.is_empty());
    }
}

pub struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    pub fn new() -> Tree {
        Tree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

struct Node {
    key: i32,
    height: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: i32) -> Node {
        Node {
            key: key,
            height: 0,
            left: None,
            right: None,
        }
    }
}
