#![allow(warnings)]

pub mod data_structures;
pub mod algo;
pub mod search;

use algo::linked_list_problems;
use algo::two_sum_problem;
use algo::fibonacci_examples;
use std::collections::LinkedList;
//use data_structures::Node;
//use data_structures::LinkedList;
use search::search_examples;

fn main() {
    //linked_list_problems::reverse_linked_list_example();
    //search_examples::binary_search_example();
    //two_sum_problem::two_sum_example()
    fibonacci_examples::finbonacci_examples();

}
