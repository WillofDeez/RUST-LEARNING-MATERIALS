use std::f64::consts::PI;

enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * PI * radius,
            Shape::Rectangle(length, width) => 2.0 * (length * width),
        }
    }

    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => 2.0 * PI * radius.powf(2.0),
            Shape::Rectangle(length, width) => length * width,
        }
    }
}

fn main() {
    let circle = Shape::Circle(7.0);
    let rectangle = Shape::Rectangle(5.0, 5.0);

    println!("The perimetre of the circle is: {}", circle.perimeter());
    println!("The area of the circle is: {} \n", circle.area());
    println!("==========================================================");
    println!(
        "\n The perimetre of the Rectangle is: {} ",
        rectangle.perimeter()
    );
    println!("The area of the Rectangle is: {}", rectangle.area());
}
