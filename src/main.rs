fn main() {
    let array = [5, 7, 8, 9, 10, 14, 6, -1, 2];

    let i = 2;
    let j = 8;

    let mut minimum = array[i];
    for n in i+1..j+1 {
    	if array[n] < minimum {
    		minimum = array[n];
    	}
    }

    println!("The minimum is {}", minimum);
}
