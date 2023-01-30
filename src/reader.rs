#![allow(dead_code)]
use std::{path::{Path, PathBuf}, fs::File, io::{BufReader, BufRead}};

use crate::figures::point::Point;
use tinyfiledialogs;

pub fn filereader() -> Vec<Point> {
    let p = tinyfiledialogs::open_file_dialog("Select a file: ", "", None);
    let path = match p {
        Some(path) => Some(PathBuf::from(path)),
        _ => None
    };

    let f = File::open(path.unwrap()).expect("File not found!");
    let buf = BufReader::new(f);
    let input: Vec<String> = buf.lines().map(|l| l.expect("could not parse line")).collect();
    
    let mut points: Vec<Point> = Vec::new();
    
    for line in input {
        let mut split = line.split(',');
        let x = split.next().unwrap().parse::<f64>().unwrap();
        let y = split.next().unwrap().parse::<f64>().unwrap();
        points.push(Point::new(x, y));
    }

    points
}
