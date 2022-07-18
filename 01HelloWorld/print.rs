fn main() {
    // In general, the '{}' will be automatically replaced with
    // any  arguments. These will be stringified
    println!("{} days", 31);

    // Positional arguments can be used.
    // Specifying an integer insiede '{}'
    // Arguments start at 0
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // Different formatting by specified format character after a ':'
    println!("Base 10 repr: {}", 69420);
    println!("Base 2 (binary) repr: {:b}", 69420);
    println!("Base 8 (octal) repr: {:o}", 69420);
    println!("Base 16 (hexadecimal) repr: {:x}", 69420);
    println!("Base 16 (hexadecimal) repr: {:X}", 69420);

    // Right align text
    println!("{number:>5}", number = 1);
    // Pad numbers with extra zeros
    println!("{number:0>5}", number = 1);
    println!("{number:#>5}", number = 1);
    println!("{number:艹>5}", number = 1);
    // Named arguments
    println!("{number:0>width$}", number = 1, width = 5);

    // For Rust 1.58 and above, you can directly capture the
    // argument from surrounding variable.
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // format! macro
    let s = format!("{}", "FUCK");
    println!("{}", s);
}
