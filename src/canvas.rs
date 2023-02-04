use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

use fltk::draw::{draw_line, draw_rect, set_line_style, LineStyle};
use fltk::prelude::SurfaceDevice;
use fltk::surface::ImageSurface;
use fltk::{app, button, draw, group};
use fltk::{
    draw::{draw_circle, draw_circle_fill, draw_point, draw_rect_fill, set_draw_color},
    enums::{Color, Event, FrameType},
    frame::Frame,
    prelude::{ImageExt, WidgetBase, WidgetExt},
};

use crate::figures::line::Line;
use crate::figures::point::Point;
use crate::figures::{circle, rectangle};
use crate::utils::{
    get_circles_intersection, get_line_circle_intersection, get_point_of_intersection,
};

#[derive(Clone)]
pub struct Canvas {
    frame: Frame,
    surf: Rc<RefCell<ImageSurface>>,
    r: Rc<RefCell<bool>>, // rectangle
    l: Rc<RefCell<bool>>, // lines
    c: Rc<RefCell<bool>>, // circle
    points: Rc<RefCell<Vec<Point>>>,
    buffer: Rc<RefCell<Vec<Point>>>,
    color: Rc<RefCell<Color>>,
    fill: Rc<RefCell<bool>>,
    lines: Rc<RefCell<Vec<Line>>>,
    circles: Rc<RefCell<Vec<circle::Circle>>>,
    lines_buffer: Rc<RefCell<Vec<Line>>>,
    circles_buffer: Rc<RefCell<Vec<circle::Circle>>>,
}

macro_rules! rcrc {
    ($obj:expr) => {
        Rc::from(RefCell::from($obj))
    };
}

// need to do some bugfixes with placig the points so that
// the circles and rectangles are drawn correctly

impl Canvas {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut frame = Frame::new(x, y, w, h, None).with_label(label);
        frame.set_color(Color::White);
        frame.set_frame(FrameType::DownBox);

        let surf = ImageSurface::new(frame.width(), frame.height(), false);
        ImageSurface::push_current(&surf);
        draw_rect_fill(0, 0, w, h, Color::White);
        ImageSurface::pop_current();

        let surf = rcrc!(surf);

        let c = false;
        let r = false;
        let l = true;
        let points: Vec<Point> = Vec::new();
        let buffer: Vec<Point> = Vec::new();
        let color = Color::Black;
        let fill = false;
        let lines: Vec<Line> = Vec::new();
        let circles: Vec<circle::Circle> = Vec::new();
        let lines_buffer: Vec<Line> = Vec::new();
        let circles_buffer: Vec<circle::Circle> = Vec::new();


        let c = rcrc!(c);
        let r = rcrc!(r);
        let l = rcrc!(l);
        let points = rcrc!(points);
        let buffer = rcrc!(buffer);
        let color = rcrc!(color);
        let fill = rcrc!(fill);
        let lines = rcrc!(lines);
        let circles = rcrc!(circles);
        let lines_buffer = rcrc!(lines_buffer);
        let circles_buffer = rcrc!(circles_buffer);

        // handlers
        frame.draw({
            let surf = surf.clone();
            move |f| {
                let surf = surf.borrow_mut();
                let mut img = surf.image().unwrap();
                img.draw(f.x(), f.y(), f.w(), f.h());
            }
        });

        frame.handle({
            let surf = surf.clone();
            let l_clone = l.clone();
            let c_clone = c.clone();
            let r_clone = r.clone();
            let points = points.clone(); // needed for later Tooltips
            let buffer = buffer.clone();
            let color = color.clone();
            let fill = fill.clone();
            let lines = lines.clone();
            let circles = circles.clone();
            let lines_buffer = lines_buffer.clone();
            let circles_buffer = circles_buffer.clone();


            move |f, ev| {
                // println!("{}", ev);
                // println!("coords {:?}", app::event_coords());
                // println!("get mouse {:?}", app::get_mouse());
                let surf = surf.borrow_mut();
                let l_bm = l_clone.borrow_mut();
                let c_bm = c_clone.borrow_mut();
                let r_bm = r_clone.borrow_mut();
                let mut points = points.borrow_mut();
                let mut buffer = buffer.borrow_mut();
                let color = color.borrow_mut();
                let fill = fill.borrow_mut();
                let mut lines = lines.borrow_mut();
                let mut circles = circles.borrow_mut();
                let mut lines_buffer = lines_buffer.borrow_mut();
                let mut circles_buffer = circles_buffer.borrow_mut();


                match ev {
                    Event::Push => {
                        ImageSurface::push_current(&surf);
                        let coords = app::event_coords();
                        set_line_style(LineStyle::Solid, 3);
                        draw_circle(coords.0 as f64, coords.1 as f64, 1.0);

                        &points.push(Point::new(coords.0 as f64, coords.1 as f64));
                        &buffer.push(Point::new(coords.0 as f64, coords.1 as f64));
                        let len = buffer.len();

                        // line
                        if len > 1 && l_bm.eq(&true) {
                            let first = &buffer[len - 1];
                            let second = &buffer[len - 2];
                            let l = Line::new(*first, *second);
                            lines_buffer.push(l);

                            set_draw_color(*color);
                            set_line_style(LineStyle::Solid, 3);

                            // draw an infinte line that should go through the first and second point

                            // y = mx + b
                            // x = (y - b) / m
                            // println!("\nFirst x: {:?}", first.get_x());
                            // println!("Second x: {:?}", second.get_x());
                            // println!("Slope: {:?}", l.get_slope());
                            if first.get_x() == second.get_x() {
                                // if the line is vertical
                                draw_line(first.get_x() as i32, 0, first.get_x() as i32, 20000);
                            } else if first.get_y() == second.get_y() {
                                // if the line is horizontal
                                draw_line(0, first.get_y() as i32, 20000, first.get_y() as i32);
                            } else {
                                let fx =
                                    (first.get_y() - l.get_y_intercept()) / l.get_slope() + 10000.0;
                                let fy = l.get_slope() * fx + l.get_y_intercept();

                                let sx =
                                    (second.get_y() - l.get_y_intercept()) / l.get_slope() - 10000.0;
                                let sy = l.get_slope() * sx + l.get_y_intercept();

                                draw_line(fx as i32, fy as i32, sx as i32, sy as i32);
                            }
                            &buffer.clear();
                        }

                        // circle
                        if len > 1 && c_bm.eq(&true) {
                            let sidepoint = &buffer[len - 1];
                            let center = &buffer[len - 2];
                            let circle = circle::Circle::new(*center, *sidepoint);
                            circles_buffer.push(circle);

                            set_draw_color(*color);
                            set_line_style(LineStyle::Solid, 3);
                            if *fill {
                                set_line_style(LineStyle::Solid, circle.get_rad() as i32 * 2);
                                draw_circle(
                                    circle.get_center().get_x() as f64,
                                    circle.get_center().get_y() as f64,
                                    1.0,
                                );
                            } else {
                                draw_circle(
                                    circle.get_center().get_x() as f64,
                                    circle.get_center().get_y() as f64,
                                    circle.get_rad(),
                                );
                            }
                            &buffer.clear();
                        }

                        // rectangle
                        if len > 1 && r_bm.eq(&true) {
                            let first = &buffer[len - 1];
                            let second = &buffer[len - 2];

                            let rect = rectangle::Rectangle::new(*first, *second);
                            set_draw_color(*color);
                            set_line_style(LineStyle::Solid, 3);
                            if *fill {
                                draw_rect_fill(
                                    rect.get_point_b().get_x() as i32,
                                    rect.get_point_b().get_y() as i32,
                                    rect.get_width() as i32,
                                    rect.get_height() as i32,
                                    *color,
                                );
                            } else {
                                draw_rect(
                                    rect.get_point_b().get_x() as i32,
                                    rect.get_point_b().get_y() as i32,
                                    rect.get_width() as i32,
                                    rect.get_height() as i32,
                                );
                            };

                            &buffer.clear();
                        }

                        let lb_len = lines_buffer.len();
                        let cb_len = circles_buffer.len();
                        let l_len = lines.len();
                        let c_len = circles.len();

                        if lb_len == 1 {
                            if l_len == 0 {
                                let l1 = lines_buffer.pop().unwrap();
                                for c in circles.iter() {
                                    let c1 = *c;
                                    let points = get_line_circle_intersection(l1, c1);
                                    // draw the intersection point
                                    set_line_style(LineStyle::Solid, 3);
                                    set_draw_color(Color::Red);
                                    for p in points.iter() {
                                        draw_circle(p.get_x() as f64, p.get_y() as f64, 1.0);
                                    }
                                }
                                lines.push(l1);
                            } else {
                                let l1 = lines_buffer.pop().unwrap();
                                for l in lines.iter() {
                                    let l2 = *l;
                                    let p = get_point_of_intersection(l1, l2);
                                    // draw the intersection point
                                    set_line_style(LineStyle::Solid, 3);
                                    set_draw_color(Color::Red);
                                    draw_circle(p.get_x() as f64, p.get_y() as f64, 1.0);
                                }
                                // calculate the intersection points between this line and circles
                                for c in circles.iter() {
                                    let c1 = *c;
                                    let points = get_line_circle_intersection(l1, c1);
                                    // draw the intersection point
                                    set_line_style(LineStyle::Solid, 3);
                                    set_draw_color(Color::Red);
                                    for p in points.iter() {
                                        draw_circle(p.get_x() as f64, p.get_y() as f64, 1.0);
                                    }
                                }
                                lines.push(l1);
                            }
                        }
                        
                        if cb_len == 1 {
                            if c_len == 0 {
                                let c1 = circles_buffer.pop().unwrap();
                                for l in lines.iter() {
                                    let l1 = *l;
                                    let points = get_line_circle_intersection(l1, c1);
                                    // draw the intersection point
                                    set_line_style(LineStyle::Solid, 3);
                                    set_draw_color(Color::Red);
                                    for p in points.iter() {
                                        draw_circle(p.get_x() as f64, p.get_y() as f64, 1.0);
                                    }
                                }
                                circles.push(c1);

                            } else {
                                let c1 = circles_buffer.pop().unwrap();
                                for c in circles.iter() {
                                    let c2 = *c;
                                    let points = get_circles_intersection(c1, c2);
                                    // draw the intersection point
                                    set_line_style(LineStyle::Solid, 3);
                                    set_draw_color(Color::Red);
                                    for p in points.iter() {
                                        draw_circle(p.get_x() as f64, p.get_y() as f64, 1.0);
                                    }
                                }
                                // calculate the intersection points between this line and circles
                                for l in lines.iter() {
                                    let l1 = *l;
                                    let points = get_line_circle_intersection(l1, c1);
                                    // draw the intersection point
                                    set_line_style(LineStyle::Solid, 3);
                                    set_draw_color(Color::Red);
                                    for p in points.iter() {
                                        draw_circle(p.get_x() as f64, p.get_y() as f64, 1.0);
                                    }
                                }
                                circles.push(c1);
                            }
                        }


                        ImageSurface::pop_current();
                        f.redraw();
                        true
                    }
                    _ => false,
                }
            }
        });

        Self {
            frame,
            surf,
            r,
            c,
            l,
            points,
            buffer,
            color,
            fill,
            lines,
            circles,
            lines_buffer,
            circles_buffer,
        }
    }

    pub fn clear(&mut self) {
        let surf = self.surf.borrow_mut();
        let mut buf = self.buffer.borrow_mut();
        let mut points = self.points.borrow_mut();
        let mut lines = self.lines.borrow_mut();
        let mut circles = self.circles.borrow_mut();
        points.clear();
        buf.clear();
        lines.clear();
        circles.clear();
        ImageSurface::push_current(&surf);
        draw_rect_fill(0, 0, self.frame.w(), self.frame.h(), Color::White);
        ImageSurface::pop_current();
        self.frame.redraw();
    }

    pub fn load(&mut self, points: Vec<Point>) {
        let surf = self.surf.borrow_mut();
        ImageSurface::push_current(&surf);
        for p in points {
            draw_circle(p.get_x(), p.get_y(), 5.0);
        }
        ImageSurface::pop_current();
        self.frame.redraw();
    }

    // a function that should change l, r, c to a difrent bool value
    pub fn toggle(&mut self, buttons: (bool, bool, bool)) {
        let mut l = self.l.borrow_mut();
        let mut r = self.r.borrow_mut();
        let mut c = self.c.borrow_mut();

        match buttons {
            (true, false, false) => {
                *l = true;
                *r = false;
                *c = false;
            }
            (false, true, false) => {
                *l = false;
                *r = true;
                *c = false;
            }
            _ => {
                *l = false;
                *r = false;
                *c = true;
            }
        }
    }

    pub fn set_color(&mut self, color: (u8, u8, u8)) {
        let mut c = self.color.borrow_mut();
        *c = Color::from_rgb(color.0, color.1, color.2);
    }

    pub fn set_fill(&mut self, fill: bool) {
        let mut f = self.fill.borrow_mut();
        *f = fill;
    }
}
