use std::{
    fmt,
    sync::{Arc, Mutex},
};

type TreeNode = Arc<Mutex<Node>>;
struct Node {
    pub value: i64,
    pub parent: Option<TreeNode>,
    pub left: Option<TreeNode>,
    pub right: Option<TreeNode>,
}
impl Node {
    fn new(value: i64) -> Self {
        Self {
            value,
            parent: None,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Clone)]
struct TreeOperationError;
impl fmt::Display for TreeOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tree OP Error")
    }
}

type TreeOperationResult<T> = std::result::Result<T, TreeOperationError>;

trait TreeOperations {
    fn insert(&mut self, value: i64) -> TreeOperationResult<()>;
    fn find(&self, value: i64) -> TreeOperationResult<Option<i64>>;
    fn remove(&mut self, value: i64) -> TreeOperationResult<Option<i64>>;
}

struct Tree {
    root: Option<TreeNode>,
}

impl Tree {
    pub fn new() -> Self {
        Self { root: None }
    }

    fn insert_node(visiting_node: &TreeNode, mut node_to_insert: Node) {
        let this_node_lock = visiting_node.lock();
        let mut this_node = this_node_lock.unwrap();

        let child_node = if this_node.value >= node_to_insert.value {
            &mut this_node.left
        } else {
            &mut this_node.right
        };

        match child_node {
            Some(visiting) => Self::insert_node(&visiting, node_to_insert),
            None => {
                node_to_insert.parent = Some(visiting_node.clone());
                child_node.replace(Arc::new(Mutex::new(node_to_insert)));
            }
        }
    }

    fn find_node(visiting_node: &TreeNode, search_value: i64) -> Option<TreeNode> {
        let this_node_lock = visiting_node.lock();
        let this_node = this_node_lock.unwrap();

        if this_node.value == search_value {
            return Some(visiting_node.clone());
        }

        let child_node = if this_node.value >= search_value {
            this_node.left.clone()
        } else {
            this_node.right.clone()
        };

        if let Some(visiting) = child_node {
            Self::find_node(&visiting.clone(), search_value)
        } else {
            None
        }
    }

    fn find_value(visiting_node: &TreeNode, search_value: i64) -> Option<i64> {
        match Self::find_node(visiting_node, search_value) {
            Some(node) => Some(node.lock().unwrap().value),
            None => None,
        }
    }

    fn find_sucessor(node: &TreeNode) -> TreeNode {
        let this_node_lock = node.lock();
        let this_node = this_node_lock.unwrap();

        match &this_node.left {
            Some(left_node) => Self::find_sucessor(&left_node),
            None => node.clone(),
        }
    }

    fn remove_node(&mut self, node: &mut TreeNode) -> Option<i64> {
        let this_node_lock = node.lock();
        let mut this_node = this_node_lock.unwrap();

        let value = (&this_node).value;

        // Find next sucessor, parent and side
        let parent = this_node.parent.clone();
        let sucessor = match &this_node.right {
            Some(right_node) => Some(Self::find_sucessor(right_node)),
            None => None,
        };
        match sucessor {
            // TODO (Otavio): Fix sucessor is not leaf
            Some(sucessor_node) => {
                this_node.value = sucessor_node.lock().unwrap().value;
                let sucessor_lock = sucessor_node.lock();
                let sucessor_lock_node = sucessor_lock.unwrap();
                let sucessor_parent = sucessor_lock_node.parent.as_ref().unwrap();
                let sucessor_parent_lock = sucessor_parent.lock();
                let mut sucessor_parent_lock_node = sucessor_parent_lock.unwrap();
                sucessor_parent_lock_node.left = None;
            }
            None => match parent {
                Some(parent_node) => parent_node.lock().unwrap().right = None,
                None => self.root = None,
            },
        }

        Some(value)
    }
}

impl TreeOperations for Tree {
    fn insert(&mut self, value: i64) -> TreeOperationResult<()> {
        let node = Node::new(value);
        match &self.root {
            // if root is none
            None => {
                // new node is root
                self.root = Some(Arc::new(Mutex::new(node)));
            }
            // else
            Some(root) => {
                Self::insert_node(root, node);
            }
        }
        Ok(())
    }

    fn find(&self, value: i64) -> TreeOperationResult<Option<i64>> {
        match &self.root {
            Some(node) => Ok(Self::find_value(node, value)),
            None => Ok(None),
        }
    }

    fn remove(&mut self, value: i64) -> TreeOperationResult<Option<i64>> {
        let node_to_remove = match &self.root {
            Some(node) => Self::find_node(node, value),
            None => None,
        };
        match node_to_remove {
            Some(mut node) => Ok(self.remove_node(&mut node)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Tree, TreeOperations};

    #[test]
    fn test_1_2() {
        let mut tree = Tree::new();
        let _ = tree.insert(1);
        {
            let root_node = &tree.root.as_ref();

            assert!(root_node.is_some());
            assert_eq!(root_node.unwrap().lock().unwrap().value, 1);
        }

        let _ = tree.insert(2);
        {
            let right_node_root_lock = &tree.root.as_ref().unwrap().lock().unwrap();
            let first_child = right_node_root_lock.right.as_ref();

            assert!(first_child.is_some());
            assert_eq!(first_child.unwrap().lock().unwrap().value, 2);
        }

        for value in 1..=2i64 {
            let found = &tree.find(value);
            assert!(found.as_ref().is_ok());
            assert!(found.as_ref().unwrap().is_some());
            assert_eq!(found.as_ref().unwrap().unwrap(), value);
        }
    }

    #[test]
    fn test_2_3_1() {
        let mut tree = Tree::new();
        let _ = tree.insert(2);
        let _ = tree.insert(3);
        let _ = tree.insert(1);

        {
            let left_node_root_lock = &tree.root.as_ref().unwrap().lock().unwrap();
            let left_node = left_node_root_lock.left.as_ref().unwrap().lock().unwrap();
            assert_eq!(left_node.value, 1);
        }

        for value in 1..=3i64 {
            let found = &tree.find(value);
            assert!(found.as_ref().is_ok());
            assert!(found.as_ref().unwrap().is_some());
            assert_eq!(found.as_ref().unwrap().unwrap(), value);
        }
    }

    #[test]
    fn test_remove() {
        let mut tree = Tree::new();
        let _ = tree.insert(1);
        let _ = tree.insert(2);
        let _ = tree.insert(3);

        for value in 1..=3i64 {
            let found = &tree.find(value);
            assert!(found.as_ref().is_ok());
            assert!(found.as_ref().unwrap().is_some());
            assert_eq!(found.as_ref().unwrap().unwrap(), value);
        }
    }
}
