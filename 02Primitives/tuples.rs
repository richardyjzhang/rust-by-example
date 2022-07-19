use std::fmt::{self, Display, Formatter};

// tuples can be used as function arguments and as return values
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "( {}, {} )\n( {}, {} )", self.0, self.1, self.2, self.3)
    }
}

fn main() {
    // a tuple with a bunch of different types
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // values can be extracted from the tuple using tuple indexing
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // tuples can be tuple members
    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    // tuples are printable
    println!("tuple of tuples: {:?}", tuple_of_tuples);
    // println!("tuple of tuples: {}", tuple_of_tuples);

    // long tuples(more than 12 elements) cannot be printed
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);

    let pair = (1, true);
    println!("pair is {:?}", pair);
    println!("the reversed pair is {:?}", reverse(pair));

    // to create one element tuples, the comma is required
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // tuples can be destructured to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{}", matrix);
}
