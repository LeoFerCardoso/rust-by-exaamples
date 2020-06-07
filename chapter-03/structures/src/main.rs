#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

// Implementation of a function inside of the Struct
impl Rectangle {
    fn area(&self) -> f32 {
        let Rectangle { 
            top_left: Point { x: ref _x1, y: ref _y1 },
            bottom_right: Point { x: ref _x2, y: ref _y2 }
        } = *self;
    
        (_x2 - _x1) * (_y1 - _y2)
    }
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle { 
        top_left: Point { x: ref _x1, y: ref _y1 },
        bottom_right: Point { x: ref _x2, y: ref _y2 }
    } = *rect;

    (_x2 - _x1) * (_y1 - _y2)
}

fn create_square(point: Point, size: f32) -> Rectangle {
    let _rect = Rectangle {
        top_left: Point { x: point.x, y: point.y + size },
        bottom_right: Point { x: point.x + size, y: point.y }
    };

    _rect
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    println!("area of rectangle: {}", rect_area(&_rectangle));
    println!("area of rectangle: {}", _rectangle.area());

    let _square = create_square(point, 5.0);
    
    println!("square rectangle: p1({}, {}) p2({}, {})",
        _square.top_left.x, _square.top_left.y,
        _square.bottom_right.x, _square.bottom_right.y
    );
    println!("area of square rectangle: {}", _square.area());

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
