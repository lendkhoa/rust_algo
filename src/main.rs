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
use algo::cses_problems;
use algo::cses_problems::increasing_array;
use algo::cses_problems::generate_permutations;
use algo::cses_problems::find_beautiful_permutation;
use std::io;

fn main() -> io::Result<()> {
    //linked_list_problems::reverse_linked_list_example();
    //search_examples::binary_search_example();
    //two_sum_problem::two_sum_example()
    // fibonacci_examples::finbonacci_examples();
    // cses_problems::weird_algorithm::find_sequence(7);
    // cses_problems::missing_number::find_missing_number();
    // generate_permutations::generate_permutation(3);
    // generate_permutations::beautiful_permutation(10);
    find_beautiful_permutation::count_beatiful_permutation(10);

    Ok(())
}
