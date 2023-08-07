#[derive(PartialEq, Eq, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        return ListNode { next: None, val };
    }

    pub fn from_vec(vec: Vec<T>) -> Option<Box<ListNode<T>>> {
        let mut current = None;
        for val in vec.into_iter().rev() {
            let mut node = ListNode::new(val);
            node.next = current;
            current = Some(Box::new(node));
        }
        return current;
    }
}

#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {ListNode::from_vec(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {ListNode::from_vec(vec![$($e.to_owned()), *])};
}
