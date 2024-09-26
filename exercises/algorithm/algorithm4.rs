/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if self.root.is_none() {
            self.root = Some(Box::new(TreeNode { value, left: None, right: None }));
            return;
        }
        
        let mut current = &mut self.root;

        while let Some(ref mut node) = current {
            println!("1");

            if node.value == value {return}

            if node.value < value {
                if(node.left.is_none()) {
                    node.left = Some(Box::new(TreeNode { value, left: None, right: None }));
                    break;
                };
                current = &mut node.right;
                continue;
            }

            if node.value > value {
                if(node.right.is_none()) {
                    node.right = Some(Box::new(TreeNode { value, left: None, right: None }));
                    break;
                };
                current = &mut node.right;
                continue;
            }

            panic!("Should not reach this line");
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if self.root.is_none() {
            return false;
        }
        
        let mut current = &self.root;

        while let Some(ref node) = current { 
            if node.value == value {return true}

            if node.value < value {
                current = &node.left;
                continue;
            }

            if node.value > value {
                current = &node.right;
                continue;
            }
        }

        false
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


