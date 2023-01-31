#![allow(dead_code)]
use std::f64;

use crate::utils::get_distance;

use super::point::Point;

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    rad: f64,
    perimeter: f64,
    diameter: f64,
    area: f64,
    center: Point,
    sidepoint: Point,
}

impl Circle {
    pub fn new(center: Point, sidepoint: Point) -> Circle {
        let rad = get_distance(center, sidepoint);
        let perimeter = 2.0 * f64::consts::PI * rad;
        let diameter = rad * 2.0;
        let area = f64::consts::PI * rad.powi(2);
        Circle {
            rad,
            perimeter,
            diameter,
            area,
            center,
            sidepoint,
        }
    }

    pub fn get_rad(&self) -> f64 {
        self.rad
    }

    pub fn get_perimeter(&self) -> f64 {
        self.perimeter
    }

    pub fn get_diameter(&self) -> f64 {
        self.diameter
    }

    pub fn get_area(&self) -> f64 {
        self.area
    }

    pub fn get_center(&self) -> Point {
        self.center
    }

    pub fn get_sidepoint(&self) -> Point {
        self.sidepoint
    }
}
