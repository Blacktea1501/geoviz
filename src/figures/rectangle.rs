#![allow(dead_code)]

use crate::utils::get_point_of_intersection;

use super::{line::Line, point::Point};

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
    position: Point,
    diagonal: Line,
    height: f64,
    width: f64,
    area: f64,
    perimeter: f64,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point) -> Rectangle {
        let diagonal = Line::new(p1, p2);
        let height = p1.get_y() - p2.get_y();
        let width = p1.get_x() - p2.get_x();
        let area = height * width;
        let perimeter = 2.0 * width + 2.0 * height;
        let position = get_point_of_intersection(
            Line::new(
                Point::new(p1.get_x(), p2.get_y()),
                Point::new(p2.get_x(), p1.get_y()),
            ),
            diagonal,
        );
        Rectangle {
            p1,
            p2,
            position,
            diagonal,
            height,
            width,
            area,
            perimeter,
        }
    }

    pub fn get_point_a(&self) -> Point {
        self.p1
    }

    pub fn get_point_b(&self) -> Point {
        self.p2
    }

    pub fn get_position(&self) -> Point {
        self.position
    }

    pub fn get_diagonal(&self) -> Line {
        self.diagonal
    }

    pub fn get_area(&self) -> f64 {
        self.area
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn get_perimeter(&self) -> f64 {
        self.perimeter
    }
}
