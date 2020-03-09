use std::io;

fn main() {
    let array = [5, 7, 8, 9, 10, 14, 6, -1, 2];
    let left_bound = prompt_for_bound(true);
    let right_bound = prompt_for_bound(false);
    let range_minimum = brute_force_rmq(array, left_bound, right_bound);
    println!("The minimum is {}", range_minimum);
}

fn prompt_for_bound(is_left: bool) -> usize {
	if is_left {
		println!("Enter the left bound.");
	} else {
		println!("Enter the right bound.");
	}

    let mut bound = String::new();

    io::stdin().read_line(&mut bound)
        .expect("Failed to read line");
    let bound: usize = bound.trim().parse()
        .expect("Please enter a number");

    bound
}

fn brute_force_rmq(array: [i32; 9], left_bound: usize, right_bound: usize) -> i32 {
	let mut minimum = array[left_bound];
	for i in left_bound+1..right_bound+1 {
		if array[i] < minimum {
			minimum = array[i];
		}
	}

	minimum
}
