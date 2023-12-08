// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut result: Vec<String> = Vec::new();

        if let Some(node) = root {
            let mut empty = true;
            result.push(format!("{}", node.borrow().val));
            if let Some(left) = &node.borrow().left {
                result.push("(".to_string());
                let mid = Solution::tree2str(Some(left.clone()));
                result.push(format!("{})", mid));
                empty = false;
            }
            if let Some(right) = &node.borrow().right {
                if empty {
                    result.push("()".to_string())
                }
                result.push("(".to_string());
                let mid = Solution::tree2str(Some(right.clone()));
                result.push(format!("{})", mid));
            }
        }

        result.into_iter().collect::<String>()
    }
}


struct Solution {}
fn main() {
}
