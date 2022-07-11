// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references

// https://doc.rust-lang.org/rust-by-example/scope/move.html
fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);

    let mut mut_vec: Vec<i32> = vec![1, 2, 3];
    let mut mut_vec_moved = try_it(mut_vec); // moved here because not impl 'Copy' trait

    // cannot borrow from moved value
    // println!("{:?}", mut_vec);

    println!("{:?}", mut_vec_moved); // but can borrow because moved to mut_vec_moved
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}

fn try_it(mut vec: Vec<i32>) -> Vec<i32> {
    vec[0] = 666;
    println!("{:?}", vec);
    vec
}
