#![allow(dead_code, unused)]

use std::borrow::BorrowMut;

use fltk::enums::{Color, FrameType};
use fltk::{app, button, prelude::*, window};
use fltk::{frame, group};
use fltk_theme::{ThemeType, WidgetTheme};

use crate::canvas::Canvas;
use crate::figures::circle::Circle;
use crate::figures::line::Line;
use crate::figures::point::Point;
use crate::reader::*;
use crate::utils::*;

mod canvas;
mod figures;
mod reader;
mod utils;

fn main() {
    const PROGRAMMWIDTH: i32 = 1080;
    const PROGRAMMHEIGHT: i32 = 720;
    const BUTTONHEIGHT: i32 = 50;
    const BUTTONWIDTH: i32 = 200;

    // lets build some gui
    let a = app::App::default();
    let mut win = window::Window::new(200, 200, PROGRAMMWIDTH, PROGRAMMHEIGHT, "Geoviz");
    let theme = WidgetTheme::new(ThemeType::Greybird);
    theme.apply();

    // open file button
    let mut open_filebtn = button::Button::new(
        PROGRAMMWIDTH - BUTTONWIDTH,
        PROGRAMMHEIGHT - BUTTONHEIGHT,
        BUTTONWIDTH,
        BUTTONHEIGHT,
        "Open file",
    );

    // clear screen button
    let mut clrscrn = button::Button::new(
        PROGRAMMWIDTH - BUTTONWIDTH * 2,
        PROGRAMMHEIGHT - BUTTONHEIGHT,
        BUTTONWIDTH,
        BUTTONHEIGHT,
        "Clear",
    );

    // RadioRoundButtons
    let mut flex = group::Flex::new(
        PROGRAMMWIDTH - BUTTONWIDTH * 3 + 150,
        PROGRAMMHEIGHT - BUTTONHEIGHT,
        50,
        BUTTONHEIGHT,
        "",
    );
    let mut rad1 = button::RadioRoundButton::new(0, 0, 0, 0, "L");
    rad1.toggle(true);
    let mut rad2 = button::RadioRoundButton::new(0, 0, 0, 0, "C");
    rad2.toggle(false);
    let mut rad3 = button::RadioRoundButton::new(0, 0, 0, 0, "R");
    rad3.toggle(false);
    flex.end();

    let mut cvs: canvas::Canvas =
        canvas::Canvas::new(0, 0, PROGRAMMWIDTH, PROGRAMMHEIGHT - BUTTONHEIGHT, "");

    win.end();
    win.show();

    // if any RadioRoundButton is toggled they should use the toggled function
    flex.set_callback({
        let mut clone = cvs.clone();
        move |_| {
            let mut cvs_clone = clone.borrow_mut();
            cvs_clone.toggle((rad1.is_toggled(), rad2.is_toggled(), rad3.is_toggled()));
        }
    });

    open_filebtn.set_callback({
        let mut clone = cvs.clone();
        move |_| {
            let cvs_clone = clone.borrow_mut();
            cvs_clone.load(reader::filereader());
        }
    });

    clrscrn.set_callback({
        let mut clone = cvs.clone();
        move |_| {
            let clone = clone.borrow_mut();
            clone.clear();
        }
    });

    a.run().unwrap();
}
