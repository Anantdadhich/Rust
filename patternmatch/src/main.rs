enum Shape {
    Circle(f64),
    Square(f64), //we storing the data in the enum
    Rectangle(f64, f64),
}

// Function to calculate the area of a shape
fn calculate(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side) => side * side,
        Shape::Rectangle(length, width) => length * width,
    }
}

fn main() {
    let circle = Shape::Circle(4.0);
    let sq = Shape::Square(4.0);
    let rect = Shape::Rectangle(3.0, 4.0);

    println!("The area of the circle is: {}", calculate(circle));
    println!("The area of the square is: {}", calculate(sq));
    println!("The area of the rectangle is: {}", calculate(rect));
}
