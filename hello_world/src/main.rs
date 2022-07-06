fn main() {
    hello("s".to_string());
}

fn hello(_unused_string_for_fun: String) -> () {
    let mut hi: String = "hi".to_owned();
    let world: String = " world".to_owned();
    hi.push_str(&world);
    println!("{}", hi);
}

#[allow(dead_code)]
fn yes() -> String {
    let hello = String::from("hello");
    return hello;
}
