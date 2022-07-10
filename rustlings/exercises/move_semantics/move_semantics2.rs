// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` for hints :)

// I AM NOT DONE

fn main() {
    let mut vec0 = Vec::new();

    // 1.
    // copy vector and pass the copy
    // let vec_copy = vec0.to_vec();

    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // vec1.push(88);
    // mutate directly the first vector
    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
// 2.
// borrow as slice then copy
// fn fill_vec(vec: &mut [i32]) -> () {
// 3.
// borrow as mutable vector since slice can't grow
// https://stackoverflow.com/questions/24102615/passing-a-vec-into-a-function-by-reference
fn fill_vec(vec: &mut Vec<i32>) -> () {
    vec.push(1);
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
