use crate::data_structures::Node;
use crate::data_structures::LinkedList;
use std::ptr::NonNull;

pub fn reverse_linked_list_example() {
    let mut list = LinkedList::new();
    list.add(1);
    list.add(2);
    let size = list.length;

    let mut prev: Node<i32>;
    let mut current = list.get(0);

    // TODO 
    // while current != None {
    //     let mut next = Some(unsafe { (current) });
    //     match current.next {
    //         None => return,
    //         Some(ptr) => (
    //             unsafe {
    //                 (*ptr.as_ptr()).next = prev;
    //             }
    //         ),
    //     }
    //     println!("Good");
    // }

    // for i in 0..size {
    //     match list.get(i as i32) {
    //         Some(ptr) => {
    //             println!("{}", ptr);
    //         },
    //         None => return,
    //     }
    // }
}


///    This is just an example to see how to manually link the node
fn manual_add_node() {
    let mut node = Node::new(1);

    let mut node2 = Box::new(Node::new(2));
    node.next = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node2)) });

    // level 2
    let mut node3 = Box::new(Node::new(3));
    match node.next {
        Some(next_ptr) =>(unsafe {
            (*next_ptr.as_ptr()).next = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node3)) });
        }),
        _ => return,
    }

    print_linked_list(Some(unsafe{NonNull::new_unchecked(Box::into_raw(Box::new(node)))}));
}

pub fn print_linked_list(node: Option<NonNull<Node<i32>>>) {
    match node {
        None => return,
        Some(ptr) => unsafe {
            println!("{}", *ptr.as_ptr());
            //(*ptr.as_ptr()).next = ptr
        },
    }
}
