// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` for hints!

struct S {
    v: u8,
    dot_dot: u8,
    underscore: u8,
    pattern_matching: u8,
}

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    let s = S {
        v: 1,
        dot_dot: 0,
        underscore: 0,
        pattern_matching: 0,
    };

    let S { v, .. } = s;
    println!("same but for struct {}", v);

    let mut tup = (1, 2, 3, 4);
    // is copied not moved
    let (mut f, _, _, mut l) = tup;
    f = 11;
    tup.0 = 123;
    println!("{}, {}", f, l); // 11, 4

    println!("{} is {} years old.", name, age);
}
