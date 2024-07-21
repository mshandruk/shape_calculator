pub trait ShapeAreaCalculator {
    fn calculate_area(&self) -> u32;
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}


impl ShapeAreaCalculator for Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.height
    }
}
