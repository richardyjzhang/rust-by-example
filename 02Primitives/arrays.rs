use std::mem;

// this function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // fixed-size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // all elements can be initialized to the same value
    let ys: [i32; 500] = [0; 500];

    // indexing starts at 0
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` returns the count of elements
    println!("number of elements in array: {}", xs.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // arrays can be automatically borrowed as slices
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slices can point to a section of an array
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1..4]);

    // example of empty slice
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // out of bound indexing
    // println!("{}", xs[5]);
    // analyze_slice(&[]);
}
