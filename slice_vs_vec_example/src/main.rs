fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3];
    println!("initial vec");
    for i in &vec {
        println!("{}", i);
    }
    println!();
    hi(&mut vec);
}

fn hi(vec: &mut Vec<i32>) -> () {
    // no method mamed `push`
    // vec.push(4);
    // when vec: &mut [i32]

    vec.push(4);
    for i in vec.iter_mut() {
        *i += 1;
    }

    for i in vec.iter() {
        println!("{}", i);
    }
}
