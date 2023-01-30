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
    ((p1.get_x() - p2.get_x()).powf(2.0) + (p1.get_y() - p2.get_y()).powf(2.0)).sqrt()
}

pub fn get_line_circle_intersection(l: Line, c: Circle) -> Vec<Point> {
    if l.get_slope().is_infinite() == false {
        let center_intercect: bool =
            c.get_center().get_y() == l.get_slope() * c.get_center().get_x() + l.get_y_intercept();
        let lot_slope: f64 = -1.0 / l.get_slope();
        let rad = c.get_rad();
        let intersect_point = get_vector_points(rad, lot_slope, c.get_center());
        let lot_circle_intersect = get_point_of_intersection(
            Line::new(l.get_point_a(), l.get_point_b()),
            Line::new(c.get_center(), intersect_point[0]),
        );
        let lot = Line::new(lot_circle_intersect, c.get_center());
        let lot_len = get_distance(lot.get_point_a(), lot.get_point_b());

        if center_intercect {
            if l.get_slope() == 0.0 {
                let x_intersect = c.get_center().get_x() + rad;
                let x2_intersect = c.get_center().get_x() - rad;
                let y_intersect = c.get_center().get_y();
                return vec![
                    Point::new(x_intersect, y_intersect),
                    Point::new(x2_intersect, y_intersect),
                ];
            } else {
                return get_vector_points(rad, l.get_slope(), c.get_center());
            }
        } else if lot_len < rad {
            let kat_len = (rad.powf(2.0) - lot_len.powf(2.0)).sqrt();
            return get_vector_points(kat_len, l.get_slope(), lot_circle_intersect);
        } else if lot_len == rad {
            return vec![lot_circle_intersect];
        }
    } else if (c.get_center().get_x().abs() - l.get_point_a().get_x().abs()).abs() == 0.0 {
        return vec![
            Point::new(c.get_center().get_x(), c.get_center().get_y() + c.get_rad()),
            Point::new(c.get_center().get_x(), c.get_center().get_y() - c.get_rad()),
        ];
    } else if (c.get_center().get_x().abs() - l.get_point_a().get_x().abs()).abs() < c.get_rad() {
        let point_a = Point::new(l.get_point_a().get_x(), c.get_center().get_y());
        let cathete = (c.get_rad().powf(2.0) - (get_distance(point_a, c.get_center())).powf(2.0)).sqrt();
        return vec![
            Point::new(l.get_point_a().get_x(), c.get_center().get_y() + cathete),
            Point::new(l.get_point_a().get_x(), c.get_center().get_y() - cathete),
        ];
    } else if (c.get_center().get_x().abs() - l.get_point_a().get_x().abs()).abs() == c.get_rad() {
        return vec![Point::new(l.get_point_a().get_x(), c.get_center().get_y())];
    }

    Vec::new()
}

fn get_vector_points(len: f64, slope: f64, start: Point) -> Vec<Point> {
    let vec_len = (1.0 + slope.powf(2.0)).sqrt();
    let vec_times = len / vec_len;
    vec![
        Point::new(start.get_x() + vec_times, start.get_y() + vec_times * slope),
        Point::new(start.get_x() - vec_times, start.get_y() - vec_times * slope),
    ]
}

pub fn get_circles_intersection(c1: Circle, c2: Circle) -> Vec<Point> {
    let center_distance = get_distance(c1.get_center(), c2.get_center());
    let min_rad = c1.get_rad().min(c2.get_rad());
    let max_rad = c1.get_rad().max(c2.get_rad());
    let slope1 = (c2.get_center().get_x() - c1.get_center().get_x()) / center_distance;
    let slope2 = (c2.get_center().get_y() - c1.get_center().get_y()) / center_distance;
    let x_distance = (c1.get_rad().powf(2.0) + center_distance.powf(2.0) - c2.get_rad().powf(2.0))
        / (2.0 * center_distance);
    let y_distance = (c1.get_rad().powf(2.0) - x_distance.powf(2.0)).sqrt();

    if center_distance == (c1.get_rad() + c2.get_rad()) || center_distance + min_rad == max_rad {
        let x = c1.get_center().get_x() + x_distance * slope1;
        let y = c1.get_center().get_y() + y_distance * slope2;
        return vec![Point::new(x, y)];
    }

    if center_distance > c1.get_rad() + c2.get_rad()
        || center_distance + min_rad < max_rad
        || (center_distance == 0.0 && c1.get_rad() != c2.get_rad())
    {
        return Vec::new();
    }

    if center_distance + min_rad > max_rad || center_distance - c1.get_rad() - c2.get_rad() < 0.0 {
        let x1 = c1.get_center().get_x() + x_distance * slope1 - y_distance * slope2;
        let y1 = c1.get_center().get_y() + x_distance * slope2 + y_distance * slope1;
        let x2 = c1.get_center().get_x() + x_distance * slope1 + y_distance * slope2;
        let y2 = c1.get_center().get_y() + x_distance * slope2 - y_distance * slope1;
        return vec![Point::new(x1, y1), Point::new(x2, y2)];
    } else {
        return Vec::new();
    }
}
