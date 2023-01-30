use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

use fltk::draw::{draw_line, set_line_style, LineStyle};
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

#[derive(Clone)]
pub struct Canvas {
    frame: Frame,
    surf: Rc<RefCell<ImageSurface>>,
    r: Rc<RefCell<bool>>, // rectangle
    l: Rc<RefCell<bool>>, // lines
    c: Rc<RefCell<bool>>, // circle
    points: Rc<RefCell<Vec<Point>>>,
}

macro_rules! rcrc {
    ($obj:expr) => {
        Rc::from(RefCell::from($obj))
    };
}

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

        let c = rcrc!(c);
        let r = rcrc!(r);
        let l = rcrc!(l);
        let points = rcrc!(points);

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
            let points = points.clone();
            move |f, ev| {
                // println!("{}", ev);
                // println!("coords {:?}", app::event_coords());
                // println!("get mouse {:?}", app::get_mouse());
                let surf = surf.borrow_mut();
                let l_lmao = l_clone.borrow_mut();
                let mut points = points.borrow_mut();

                match ev {
                    Event::Push => {
                        ImageSurface::push_current(&surf);
                        let coords = app::event_coords();
                        draw_circle_fill(coords.0, coords.1, 5, Color::Black);

                        &points.push(Point::new(coords.0 as f64, coords.1 as f64));
                        let len = points.len();

                        if len > 1 && l_lmao.eq(&true) {
                            let first = &points[len - 1];
                            let second = &points[len - 2];
                            set_draw_color(Color::Black);
                            set_line_style(LineStyle::Solid, 2);
                            draw_line(
                                first.get_x() as i32,
                                first.get_y() as i32,
                                second.get_x() as i32,
                                second.get_y() as i32,
                            );
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
        }
    }

    pub fn clear(&mut self) {
        let surf = self.surf.borrow_mut();
        let mut points = self.points.borrow_mut();
        points.clear();
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
        println!("Toggle!");
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
}
