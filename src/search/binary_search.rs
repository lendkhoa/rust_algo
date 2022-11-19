// Binary search

// arguments: array slice, array length, target integer
// return the index
pub fn search(a: &[i32], target: &i32) -> Option<usize> {
    let mut low_index: i8 = 0;
    let mut hi_index: i8 = a.len() as i8 - 1;

    while low_index <= hi_index {
        let mid = ((hi_index - low_index) / 2) + low_index;
        let mid_index = mid as usize;
        let val = &a[mid_index];
        if val == target {
            return Some(mid_index);
        }
        if val < target {
            low_index = mid + 1;
        }
        if val > target {
            hi_index = mid - 1;
        }

    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_iterative() {
        let correct_arr = [
		1, 10, 20, 47, 59, 63, 75, 88, 99,
		107, 120, 133, 155, 162, 176, 188,
		199, 200, 210, 222
		];
        for i in 0..correct_arr.len() {
            assert_eq!(i, search(&correct_arr, &correct_arr[i]).unwrap());
        }
    }
    #[test]
    fn incorrect_iterative() {
        let searched_arr = [
		1, 10, 20, 47, 59, 63, 75, 88, 99,
		107, 120, 133, 155, 162, 176, 188,
		199, 200, 210, 222
		];
        let incorrect_arr = [
		2, 22, 48, 58, 61, 73, 84, 90, 100,
		119, 132, 154, 160, 177, 187, 197,
		201, 211, 2242
		];
        for i in 0..incorrect_arr.len() {
            assert_eq!(None, search(&searched_arr, &incorrect_arr[i]));
        }
    }
}
