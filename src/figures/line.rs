#![allow(dead_code)]
use crate::utils::{self, get_y_intercept};

use super::point::Point;

#[derive(Debug, Clone, Copy)]
pub struct Line {
    a: Point,
    b: Point,
    slope: f64,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Line {
        let slope = utils::get_slope(a.get_x(), b.get_x(), a.get_y(), b.get_y());
        Line { a,  b,  slope}
    }

    pub fn get_point_a(&self) -> Point {
        self.a 
    }

    pub fn get_point_b(&self) -> Point {
        self.b
    }

    pub fn get_slope(&self) -> f64 {
        self.slope
    }

    pub fn get_y_intercept(&self) -> f64 {
        // this is from utils
        get_y_intercept(self.a.get_x(), self.b.get_x(), self.a.get_y(), self.b.get_y())
    }

} 
