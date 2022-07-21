use rayon::prelude::*;
fn sum_of_squares(input: &[i32]) -> i32 {
    input
        .par_iter() // <-- just change that!
        .map(|&i| i * i)
        .sum()
}

fn main() {
    sum_of_squares(&vec![1, 2, 3]);
}
