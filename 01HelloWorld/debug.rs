// Derive the `fmt::Debug` implementation for Structure
#[derive(Debug)]
struct Structure(i32);

// Put a Structure inside of the structure Deep
#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`
    println!("{:?} months in a year.", 12);
    println!(
        "{:?} {:?} is the {actor:?} name.",
        "Slaster",
        "Christian",
        actor = "actor's"
    );

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));
    // println!("Still {} will not print", Structure(3));

    // `derive` can't control over how the results look
    println!("Now {:?} will print!", Deep(Structure(3)));

    // Pretty print
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:#?}", peter);
}
