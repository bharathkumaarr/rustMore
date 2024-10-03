enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
    Square(f64),
}
fn main() {
    let r = Shape::Rectangle(10.0, 20.0);
    println!("Area of Rectangle: {}", calculate_area(r));

    let c = Shape::Circle(7.0);
    println!("Area of Circle: {}", calculate_area(c));

    let s = Shape::Square(4.0);
    println!("Area of Square: {}", calculate_area(s));
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a,b) => a*b, 
        Shape::Circle(r) => 3.1415 * r *r,
        Shape::Square(side) => side*side,
    }
}
}
