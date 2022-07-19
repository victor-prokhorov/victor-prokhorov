// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` for hints!

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[derive(Debug, PartialEq)]
struct S<T> {
    d: T,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }

    #[test]
    fn quick_check() {
        let s = S { d: 0_u8 };
        assert_eq!(Wrapper::new(S { d: 0_u8 }).value, s);
        assert_eq!(Wrapper::new(S { d: 0_u8 }).value.d, 0_u8);
    }
}
