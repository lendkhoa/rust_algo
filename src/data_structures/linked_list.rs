use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

/**
 * Defines a struct
 */
pub struct Node<T> {
    pub val: T,
    pub next: Option<NonNull<Node<T>>>,
    pub prev: Option<NonNull<Node<T>>>,
}

impl <T> Node<T> {
    pub fn new(t: T) -> Node<T>{
        Node {
            val: t,
            next: None,
            prev: None,
        }
    }
}

pub struct LinkedList<T> {
    pub length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        // get a pointer to node
        let node_ptr = Some(unsafe {NonNull::new_unchecked(Box::into_raw(node))});
        match self.end {
            // this is the case of empty list
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe {(*end_ptr.as_ptr()).next = node_ptr},
        }

        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { & (*next_ptr.as_ptr()).val}),
                _ => self.get_ith_node(unsafe{ (*next_ptr.as_ptr()).next}, index - 1),
            }
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked list is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn test_tempty_list() {
        let list = LinkedList::<i32>::new();
        assert_eq!(0, list.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        println!(">> Linked List is {}", list);
        let retrived_item = list.get(1);
        assert!(retrived_item.is_some());
        assert_eq!(2 as i32, *retrived_item.unwrap());
    }
}
