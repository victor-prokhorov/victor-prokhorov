// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` for hints!!

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // borrow a section of the array as a slice
    // range is exclusive at upper bound
    let nice_slice = &a[2 - 1..a.len() + 2 - 3];

    assert_eq!([2, 3, 4], nice_slice)
}
