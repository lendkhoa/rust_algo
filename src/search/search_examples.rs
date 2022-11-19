use crate::search::binary_search;

pub fn binary_search_example() {
	let array = [1,3,4,554,667,4546];
	let target = 667;
	if let Some(result) = binary_search::search(&array, &target) {
		println!("{} found at index {}", target, result);
	} else {
		println!("{} not found!", target)
	}
}