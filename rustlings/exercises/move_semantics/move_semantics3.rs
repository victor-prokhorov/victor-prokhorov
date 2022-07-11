// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0); // mutable borrow
                                        // vec1 had the borrow on vec0

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);
    // this mean we cannot mutate already borrowed
    // vec0.push(123);
    // you cannot have any other borrow not even non-mutable

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    // this was the last line vec1 was used
    // vec0 can be borrowed now
    // lets try to do multiple non mutable borrow
    let v0 = check_if_i_got_this(&vec0);
    let v1 = check_if_i_got_this(&v0);
    println!("{:?}, {:?}", v0, v1);

    // v0 and v1 droped
    // we can mutate again
    vec0.push(1234);
    println!("{:?}", vec0);
}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec // we return mutable borrow, we still hold it
}

fn check_if_i_got_this(vec: &Vec<i32>) -> &Vec<i32> {
    println!("{:?}", vec);
    return vec;
}
