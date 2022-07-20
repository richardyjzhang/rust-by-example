#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// a unit struct
struct Unit;

// a tuple struct
struct Pair(i32, f32);

// a struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// structs can be reused as fields
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

// activity 1
fn rect_area(rect: &Rectangle) -> f32 {
    let width = (rect.top_left.x - rect.bottom_right.x).abs();
    let height = (rect.top_left.y - rect.bottom_right.y).abs();
    width * height
}

// activity 2
fn square(point: &Point, width: f32) -> Rectangle {
    let p1 = Point { x: point.x, y: point.y };
    let p2 = Point { x: point.x + width, y: point.y + width };
    Rectangle { top_left: p1, bottom_right: p2 }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("point coordinates: ({}, {})", point.x, point.y);

    // make a new point by using struct update syntax
    let bottom_right = Point { x: 5.2, ..point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;
    let rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    println!("rect is {:?}", rectangle);

    // activity 1
    println!("area of rectangle: {}", rect_area(&rectangle));

    // activity 2
    println!("Square: {:?}", square(&point, 3.5));

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}