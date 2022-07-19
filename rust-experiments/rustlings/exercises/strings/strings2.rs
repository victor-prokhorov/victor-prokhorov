// strings2.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings2` for hints :)

fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }

    let mut sentence = "cool story amigo".to_string();
    let string_slice = first_word(&sentence); // borrow occurs here
    println!("{}", string_slice);
    println!("{sentence}");
    sentence = "something else".to_string(); // assignement to borrowed
    let another_string_slice = first_word(&sentence);
    println!("another: {}", another_string_slice);
    // println!("initial slice: {}", string_slice); // will throw an error

    // let mut s: &str = "hello"
    // The variable s is a mutable reference to a str. The reference can be mutated to point to another str but the str itself never changes.
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

// https://doc.rust-lang.org/nightly/book/ch04-03-slices.html
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
