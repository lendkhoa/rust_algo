#![allow(warnings)]

pub mod data_structures;
pub mod algo;
pub mod search;

use algo::linked_list_problems;
use std::collections::LinkedList;
//use data_structures::Node;
//use data_structures::LinkedList;
use search::search_examples;

fn main() {
    //linked_list_problems::reverse_linked_list_example();
    search_examples::binary_search_example();
}
