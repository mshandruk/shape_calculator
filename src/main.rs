use shape_calculator::{ShapeAreaCalculator, Rectangle};

fn main() {
    let rectangle = Rectangle {
        width: 20,
        height: 30,
    };
    let rectangle_area = rectangle.calculate_area();
    println!("Rectangle area is: {}", rectangle_area);
}