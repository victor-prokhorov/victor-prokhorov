// structs1.rs
// Address all the TODOs to make the tests pass!

struct ColorClassicStruct {
    name: String,
    hex: String,
}

struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitStruct;

trait Yes {
    fn yes() -> ();
}
impl Yes for UnitStruct {
    fn yes() {
        println!("yes");
    }
}
impl UnitStruct {
    fn no() {
        println!("no");
    }
}
struct Sheep {
    naked: bool,
    name: &'static str,
}
trait Animal {
    fn new(name: &'static str) -> Self;
    fn from_animal_trait() -> ();
}
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
}
impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name, naked: false }
    }
    fn from_animal_trait() {
        println!("from animal trait");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn classic_c_structs() {
        let green = ColorClassicStruct {
            name: "green".to_string(),
            hex: "#00FF00".to_string(),
        };

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        let green = ColorTupleStruct("green".to_string(), "#00FF00".to_string());
        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // let unit_struct = UnitStruct {};
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}

fn main() {
    UnitStruct::yes();
    UnitStruct::no();
    Sheep::from_animal_trait();
    let sheep = Sheep::new("sheep");
    let a: bool = Sheep::is_naked(&sheep);
    let b: bool = sheep.is_naked();
    assert_eq!(a, b);
}
