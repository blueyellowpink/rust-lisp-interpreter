use crate::RispExp;

#[derive(Debug)]
pub struct Tree<T: Debug>(Option<Box<Node<T>>>);

impl<T: Debug> Tree<T> {
    pub fn new() -> Self {
        Self(None)
    }

    pub fn left(&self) -> Option<&Self> {
        if let Some(node) = &self.0 {
            return Some(&node.left);
        }
        None
    }

    pub fn right(&self) -> Option<&Self> {
        if let Some(node) = &self.0 {
            return Some(&node.right);
        }
        None
    }
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    pub value: T,
    pub left: Tree<T>,
    pub right: Tree<T>,
}

impl<T: Debug> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: Tree(None),
            right: Tree(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_add_new_node() {
        let mut tree = Tree::<RispExp>::new();
        /* tree.insert(5);
        tree.insert(6);
        tree.insert(7);
        tree.insert(2);

        assert_eq!(tree.left().unwrap().value().unwrap(), &2);
        assert_eq!(tree.right().unwrap().value().unwrap(), &6); */
    }
}
