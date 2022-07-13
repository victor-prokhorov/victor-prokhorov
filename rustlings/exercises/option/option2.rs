// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();

    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    for x in 3..5 {
        optional_integers_vec[x] = None;
    }

    println!("{:?}", optional_integers_vec);

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    // while let Some(option) = optional_integers_vec.pop() {
    //     if let Some(integer) = option {
    //         println!("current value: {}", integer);
    //     }
    // }
    // or
    while let Some(int) = optional_integers_vec.pop().flatten() {
        println!("current value: {}", int);
        // return ();
    }

    println!("{:?}", optional_integers_vec);
}
