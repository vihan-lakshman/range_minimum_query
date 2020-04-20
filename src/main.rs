use std::cmp;
use std::io;

fn main() {
    let array = [5, 7, 8, 9, 10, 14, 6, -1, 2];

    let left_bound = prompt_for_bound(true);
    let right_bound = prompt_for_bound(false);

    let bf_range_minimum = brute_force_rmq(array, left_bound, right_bound);
    let dp_range_minimum = dp_rmq(array, left_bound, right_bound);

    assert_eq!(bf_range_minimum, dp_range_minimum);

    println!("The minimum is {}", dp_range_minimum);
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

// Preprocessing Time: O(1)
// Query Time: O(n)
fn brute_force_rmq(array: [i32; 9], left_bound: usize, right_bound: usize) -> i32 {
    let mut minimum = array[left_bound];

    for i in left_bound+1..right_bound+1 {
        if array[i] < minimum {
            minimum = array[i];
        }
    }

    minimum
}

// Preprocessing Time: O(n^2) (via dynamic programming)
// Query Time: O(1)
fn dp_rmq(array: [i32; 9], left_bound: usize, right_bound: usize) -> i32 {
    let mut minimum_value;
    let mut dp_rmq_structure = vec![vec![100; 9]; 9];
    
    // Initial linear pass over the array
    // Complexity: O(n)
    for i in 0..array.len() {
        dp_rmq_structure[i][i] = array[i];
    }

    // Secondary pass to fill in the dp rmq structure via dynamic programming
    // Complexity: O(n^2)
    for i in 0..array.len() {
        for j in i+1..array.len() {
            minimum_value = cmp::min(dp_rmq_structure[i][j-1], dp_rmq_structure[j][j]);
            dp_rmq_structure[i][j] = minimum_value;
        }
    }

    dp_rmq_structure[left_bound][right_bound]
}
