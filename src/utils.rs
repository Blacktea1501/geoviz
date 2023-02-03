use crate::figures::{circle::Circle, line::Line, point::Point};

pub fn get_slope(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    (y2 - y1) / (x2 - x1)
}

pub fn round2(val: f64) -> f64 {
    (val * 100.0).round() as f64 / 100.0
}

pub fn get_y_intercept(x1: f64, x2: f64, y1: f64, y2: f64) -> f64 {
    // y = mx + b
    // b = y - mx;
    y1 - (get_slope(x1, x2, y1, y2) * x1)
}

pub fn is_parallel(slope1: f64, slope2: f64) -> bool {
    slope1 == slope2
}

pub fn is_orthogonal(slope1: f64, slope2: f64) -> bool {
    slope1 * slope2 == -1.0
}

pub fn get_point_of_intersection(l1: Line, l2: Line) -> Point {
    // check infinity slope
    if l1.get_slope().is_infinite() || l2.get_slope().is_infinite() {
        if l1.get_slope().is_infinite() {
            let y = l2.get_slope() * l1.get_point_a().get_x() + l2.get_y_intercept();
            return Point::new(l1.get_point_a().get_x(), y);
        } else {
            let y = l1.get_slope() * l2.get_point_a().get_x() + l1.get_y_intercept();
            return Point::new(l2.get_point_a().get_x(), y);
        }
    } else {
        let x = (l2.get_y_intercept() - l1.get_y_intercept()) / (l1.get_slope() - l2.get_slope());
        let y = (l1.get_slope() * x) + (l1.get_y_intercept());
        return Point::new(x, y);
    }
}

pub fn get_distance(p1: Point, p2: Point) -> f64 {
    ((p1.get_x() - p2.get_x()).powi(2) + (p1.get_y() - p2.get_y()).powi(2)).sqrt()
}

// mathworld.wolfram.com/Circle-LineIntersection.html
pub fn get_line_circle_intersection(line: Line, c: Circle) -> Vec<Point> {
    let mut l = line.clone();
    l.move_line(Point::new(-c.get_center().get_x(), -c.get_center().get_y()));
    let dx = l.get_point_b().get_x() - l.get_point_a().get_x();
    let dy = l.get_point_b().get_y() - l.get_point_a().get_y();
    let dr = (dx.powi(2) + dy.powi(2)).sqrt();
    let big_d = l.get_point_a().get_x() * l.get_point_b().get_y()
        - l.get_point_b().get_x() * l.get_point_a().get_y();
    let discriminant = c.get_rad().powi(2) * dr.powi(2) - big_d.powi(2);
    if discriminant < 0.0 {
        return vec![];
    } else if discriminant == 0.0 {
        let x = big_d * dy / dr.powi(2);
        let y = -big_d * dx / dr.powi(2);
        let mut p = Point::new(x, y);
        p.add_point(c.get_center());
        return vec![p];
    } else {
        let x1 = (big_d * dy + sgn(dy) * dx * discriminant.sqrt()) / dr.powi(2);
        let x2 = (big_d * dy - sgn(dy) * dx * discriminant.sqrt()) / dr.powi(2);
        let y1 = (-big_d * dx + dy.abs() * discriminant.sqrt()) / dr.powi(2);
        let y2 = (-big_d * dx - dy.abs() * discriminant.sqrt()) / dr.powi(2);
        let mut points = vec![Point::new(x1, y1), Point::new(x2, y2)];
        for p in &mut points {
            p.add_point(c.get_center());
        }
        return points;
    }
}

fn sgn(x: f64) -> f64 {
    if x < 0.0 {
        return -1.0;
    } else if x > 0.0 {
        return 1.0;
    } else {
        return 0.0;
    }
}

// this is from my original java project translatet to rust
pub fn get_circles_intersection(c1: Circle, c2: Circle) -> Vec<Point> {
    let center_distance = get_distance(c1.get_center(), c2.get_center());
    let min_rad = c1.get_rad().min(c2.get_rad());
    let max_rad = c1.get_rad().max(c2.get_rad());
    let slope1 = (c2.get_center().get_x() - c1.get_center().get_x()) / center_distance;
    let slope2 = (c2.get_center().get_y() - c1.get_center().get_y()) / center_distance;
    let x_distance = (c1.get_rad().powf(2.0) + center_distance.powf(2.0) - c2.get_rad().powf(2.0))
        / (2.0 * center_distance);
    let y_distance = (c1.get_rad().powf(2.0) - x_distance.powf(2.0)).sqrt();
    if double_comparison(center_distance, c1.get_rad() + c2.get_rad())
        || double_comparison(center_distance + min_rad, max_rad)
    {
        let x = c1.get_center().get_x() + x_distance * slope1;
        let y = c1.get_center().get_y() + x_distance * slope2;

        return vec![Point::new(x, y)];
    } else if center_distance > c1.get_rad() + c2.get_rad()
        || center_distance + min_rad < max_rad
        || (double_comparison(center_distance, 0.0) && c1.get_rad() != c2.get_rad())
    {
        return Vec::new();
    } else if center_distance + min_rad > max_rad
        || center_distance - c1.get_rad() - c2.get_rad() < 0.0
    {
        let x1 = c1.get_center().get_x() + x_distance * slope1 - y_distance * slope2;
        let y1 = c1.get_center().get_y() + x_distance * slope2 + y_distance * slope1;
        let x2 = c1.get_center().get_x() + x_distance * slope1 + y_distance * slope2;
        let y2 = c1.get_center().get_y() + x_distance * slope2 - y_distance * slope1;
        return vec![Point::new(x1, y1), Point::new(x2, y2)];
    } else {
        return Vec::new();
    }
}

pub fn double_comparison(a: f64, b: f64) -> bool {
    (a - b).abs() < 0.0000000000000001
}
