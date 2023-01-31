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

use crate::figures::point::Point;
use crate::figures::{circle, rectangle};

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

        let c = rcrc!(c);
        let r = rcrc!(r);
        let l = rcrc!(l);
        let points = rcrc!(points);
        let buffer = rcrc!(buffer);
        let color = rcrc!(color);
        let fill = rcrc!(fill);

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
            let points = points.clone();
            let buffer = buffer.clone();
            let color = color.clone();
            let fill = fill.clone();
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
                            set_draw_color(*color);
                            set_line_style(LineStyle::Solid, 3);
                            draw_line(
                                first.get_x() as i32,
                                first.get_y() as i32,
                                second.get_x() as i32,
                                second.get_y() as i32,
                            );
                            &buffer.clear();
                        }

                        // circle
                        if len > 1 && c_bm.eq(&true) {
                            let first = &buffer[len - 1];
                            let second = &buffer[len - 2];
                            let circle = circle::Circle::new(*first, *second);

                            set_draw_color(*color);
                            set_line_style(LineStyle::Solid, 3);
                            if *fill {
                                // draw_circle_fill seems kinda buggy
                                // need to change this to draw cicle and fill it using
                                // LineStyle::Solid and radius as diameter
                                set_line_style(LineStyle::Solid, circle.get_rad() as i32 * 2);
                                draw_circle(
                                    circle.get_sidepoint().get_x() as f64,
                                    circle.get_sidepoint().get_y() as f64,
                                    1.0,
                                );
                            } else {
                                draw_circle(
                                    circle.get_sidepoint().get_x() as f64,
                                    circle.get_sidepoint().get_y() as f64,
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
        }
    }

    pub fn clear(&mut self) {
        let surf = self.surf.borrow_mut();
        let mut buf = self.buffer.borrow_mut();
        let mut points = self.points.borrow_mut();
        points.clear();
        buf.clear();
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
