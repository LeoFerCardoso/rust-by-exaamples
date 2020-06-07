#[derive(Debug)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
    let y = Operations::Subtract;

    println!("Operation: X = {:?}, run(2,3) = {}", x, x.run(2, 3));
    println!("Operation: Y = {:?}, run(2,3) = {}", y, y.run(2, 3));
}