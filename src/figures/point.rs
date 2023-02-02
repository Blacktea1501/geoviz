#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point{

    pub fn new(x: f64, y: f64 ) -> Self {
        Point { x, y }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn add_point(&mut self, other: Point) {
        self.x += other.x;
        self.y += other.y;
    }

}
