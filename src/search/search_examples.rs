use crate::search::binary_search;

pub fn binary_search_example() {
	let array = [1,3,4,667,554,4546];
	let target = &65;
	if let Some(result) = binary_search::search(&array, array.len(), target) {
		println!("{} found at index {}", target, result);
	} else {
		println!("{} not found", target)
	}
}