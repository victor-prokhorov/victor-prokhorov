// hashmap1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute the command `rustlings hint hashmap1` if you need
// hints.

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // TODO: declare your hash map here.

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);

    // TODO: Put more fruits in your basket here.

    basket.insert("1".to_string(), 1);
    basket.insert("2".to_string(), 2);
    basket.insert("3".to_string(), 3);
    basket.insert("4".to_string(), 4);

    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(1, "test".to_string());
    let first = &map[&1];
    let again = &map[&1];
    assert_eq!(first, &"test");
    assert_eq!(again, &"test");
    let value = map.get(&1);
    assert_eq!(value.unwrap(), &"test".to_string());
    assert_eq!(value, Some(&"test".to_string()));
    assert_eq!(value, Some(&"test".to_string()));

    let f = basket.get("1");
    assert_eq!(f, Some(&1));
    assert_eq!(f.unwrap(), &1);
    assert_eq!(*f.unwrap(), 1);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
