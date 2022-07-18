// iterators4.rs

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    // 1. using recursion
    // if num == 1 {
    //     return 1;
    // } else {
    //     return num * factorial(num - 1);
    // }
    // 2. using while loop
    // let mut r = 1;
    // let mut n = num;
    // while n != 1 {
    //     r *= n;
    //     n -= 1;
    // }
    // r
    // 3. using iter?
    // (1..=num)
    //     .into_iter()
    //     .reduce(|accum, item| accum * item)
    //     .unwrap()
    // 4. with fold you pass the fallback value so no unwrap
    // (1..=num).into_iter().fold(1, |a, i| a * i)
    // 5. build in
    (1..=num).product() // it uses folder under the hood
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
