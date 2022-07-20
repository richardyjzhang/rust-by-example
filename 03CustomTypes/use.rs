// the use declaration can be used
// so manual scoping isn't needed

#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // explicitly `use` each name so 
    // they are available without manual scoping
    use crate::Status::{Poor, Rich};
    // automatically `use` each name inside `Work`
    use crate::Work::*;

    // Status::Poor
    let status = Poor;
    // Work::Civilian
    let work = Civilian;

    match status {
        Rich => println!("the rich has lots of money!"),
        Poor => println!("the poor have no money..."),
    };

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight"),
    };
}