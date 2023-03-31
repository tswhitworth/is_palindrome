use std::fmt;
use std::fmt::Debug;

struct Node<T> {
    item: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(item: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { item, next }
    }
}

pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { top: None }
    }

    pub fn push_palindrome(&mut self, item: T) -> bool
        where
            T: PartialEq + Clone + Debug,
    {
        if is_palindrome(&item) {
            self.top = Some(Box::new(Node::new(item, self.top.take())));
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.top.take() {
            Some(node) => {
                self.top = node.next;
                Some(node.item)
            }
            None => None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }
}

impl<T: fmt::Debug> fmt::Debug for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        let mut node = self.top.as_ref();

        while let Some(curr) = node {
            write!(f, "{:?}", curr.item)?;
            node = curr.next.as_ref();
            if node.is_some() {
                write!(f, ", ")?;
            }
        }

        write!(f, "]")
    }
}

fn is_palindrome<T: PartialEq + Debug>(input: &T) -> bool {
    let input_str = format!("{:?}", input);
    let input_str_rev: String = input_str.chars().rev().collect();
    input_str == input_str_rev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_stack_operations() {
        let mut str_stack = Stack::new();
        assert_eq!(str_stack.push_palindrome("racecar"), true);
        assert_eq!(str_stack.push_palindrome("hello"), false);
        assert_eq!(str_stack.push_palindrome(""), true);
        assert_eq!(str_stack.push_palindrome("a"), true);
        assert_eq!(str_stack.push_palindrome("abcdcba"), true);
        assert_eq!(str_stack.pop(), Some("abcdcba"));
        assert_eq!(str_stack.pop(), Some("a"));
        assert_eq!(str_stack.pop(), Some(""));
        assert_eq!(str_stack.pop(), Some("racecar"));
        assert_eq!(str_stack.is_empty(), true);
    }

    #[test]
    fn test_int_stack_operations() {
        let mut int_stack = Stack::new();
        assert_eq!(int_stack.push_palindrome(1221), true);
        assert_eq!(int_stack.push_palindrome(123), false);
        assert_eq!(int_stack.push_palindrome(0), true);
        assert_eq!(int_stack.push_palindrome(9), true);
        assert_eq!(int_stack.push_palindrome(12321), true);
        assert_eq!(int_stack.pop(), Some(12321));
        assert_eq!(int_stack.pop(), Some(9));
        assert_eq!(int_stack.pop(), Some(0));
        assert_eq!(int_stack.pop(), Some(1221));
        assert_eq!(int_stack.is_empty(), true);
    }

    #[test]
    fn test_debug_implementation() {
        let mut int_stack = Stack::new();
        int_stack.push_palindrome(1221);
        int_stack.push_palindrome(9);
        assert_eq!(format!("{:?}", int_stack), "[9, 1221]");

        let mut str_stack = Stack::new();
        str_stack.push_palindrome("racecar");
        str_stack.push_palindrome("a");
        assert_eq!(format!("{:?}", str_stack), "[\"a\", \"racecar\"]");
    }
}