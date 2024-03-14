fn main() {
    assert_eq!("PAHNAPLSIIGYIR", convert("PAYPALISHIRING".into(), 3));
    assert_eq!(
        "PAHNAPLSIIGYIR",
        convert_optimal("PAYPALISHIRING".into(), 3)
    );
}

use std::cell::RefCell;
use std::rc::Rc;
fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    return vec![];
}

fn convert(s: String, num_rows: i32) -> String {
    let mut vec: Vec<Vec<char>> = (0..num_rows).into_iter().map(|_| vec![]).collect();
    let mut zig_row = 0;
    let mut step: i8 = 1;
    for (i, c) in s.char_indices() {
        vec[zig_row].push(c);

        if zig_row == 0 {
            step = 1;
        } else if zig_row == (num_rows as usize - 1) {
            step = -1;
        }
        zig_row = (zig_row as i32 + step as i32) as usize;
    }
    let result = vec.concat();

    result.iter().collect()
}

fn convert_optimal(s: String, num_rows: i32) -> String {
    if num_rows == 1 || s.len() <= num_rows as usize {
        return s;
    }

    let n = s.len();
    let char_vec: Vec<char> = s.chars().map(|x| x).collect();

    let mut result = String::new();
    let cycle_len = 2 * num_rows as usize - 2;
    for i in 0..num_rows as usize {
        for j in (0..n - i).step_by(cycle_len) {
            result.push(char_vec[i + j]);

            let idx = cycle_len + j - i;
            if i != 0 && i != num_rows as usize - 1 && idx < n {
                result.push(char_vec[idx]);
            }
        }
    }

    result
}

fn reverse_integer(x: i32) -> i32 {
    let is_negative = x < 0;
    let mut i = if is_negative { 0 - x } else { x };
    let mut result: i64 = 0;
    while i != 0 {
        result = 10 * result + i as i64 % 10;
        i = i / 10;
        if result < i32::MIN as i64 || result > i32::MAX as i64 {
            return 0;
        }
        println!("{}, {}", result, i)
    }
    result = if is_negative { 0 - result } else { result };
    println!("{}", result);

    result as i32
}

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
            right: None,
        }
    }
}

fn in_order_traversal() {}

mod test {
    use crate::{in_order_traversal, reverse_integer};

    #[test]
    fn test_reverse_integer() {
        assert_eq!(0, reverse_integer(1534236469));
    }

    #[test]
    fn test_in_order_traversal() {
        in_order_traversal()
    }
}
