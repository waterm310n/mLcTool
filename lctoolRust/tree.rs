use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

// 值为i32的二叉树的定义
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

// 字符串转二叉树
pub fn string_to_binary_tree(input:&str) -> Option<Rc<RefCell<TreeNode>>> {
    let input = input.trim();
    assert!(input.len()>1);
    let mut input = input.chars();
    // 去掉第一个字符
    input.next();
    // 去掉第二个字符
    input.next_back();
    let input:Vec<_> = input.collect::<String>()
        .split(",")
        .map(|s| s.parse::<i32>())
        .collect();
    // 如果第一个字符串是null
    if input.len() == 0 || input[0].is_err(){
        return None;
    }
    let mut iters = input.into_iter();
    if let Some(Ok(x)) = iters.next() {
        let root = Rc::new(RefCell::new(TreeNode::new(x)));
        let mut queue:VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
        queue.push_back(Rc::clone(&root));
        loop {
            let node = queue.pop_front().unwrap();
            if let Some(item) = iters.next() {
                if let Ok(val) = item {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().left = Some(Rc::clone(&left));
                    queue.push_back(left);
                }
            }else{
                break;
            }
            if let Some(item) = iters.next() {
                if let Ok(val) = item {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.borrow_mut().right = Some(Rc::clone(&right));
                    queue.push_back(right);
                }
            }else{
                break;
            }
        }   
        return Some(root);
    }else{
        return None;
    }
}