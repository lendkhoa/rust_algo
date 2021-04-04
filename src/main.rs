#![allow(warnings)]

pub mod data_structures;
pub mod algo;

use algo::linked_list_problems;
use std::collections::LinkedList;
//use data_structures::Node;
//use data_structures::LinkedList;

fn main() {
    linked_list_problems::reverse_linked_list_example();
}
