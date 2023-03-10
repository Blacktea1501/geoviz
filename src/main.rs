#![allow(dead_code, unused)]

use std::borrow::BorrowMut;

use fltk::dialog::{color_chooser, ColorMode};
use fltk::enums::{Color, FrameType};
use fltk::{app, button, prelude::*, window};
use fltk::{frame, group};
use fltk_theme::{ThemeType, WidgetTheme};

use crate::canvas::Canvas;
use crate::figures::circle::{Circle, self};
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
    let flex = group::Flex::new(
        PROGRAMMWIDTH - BUTTONWIDTH * 3 + 150,
        PROGRAMMHEIGHT - BUTTONHEIGHT,
        50,
        BUTTONHEIGHT,
        "",
    );
    let mut rad1 = button::RadioRoundButton::new(0, 0, 0, 0, "L");
    rad1.toggle(true);
    let mut rad2 = button::RadioRoundButton::new(0, 0, 0, 0, "R");
    rad2.toggle(false);
    let mut rad3 = button::RadioRoundButton::new(0, 0, 0, 0, "C");
    rad3.toggle(false);
    flex.end();
    
    // color selector button
    let mut colorbtn = button::Button::new(
        PROGRAMMWIDTH - BUTTONWIDTH * 4 + 150,
        PROGRAMMHEIGHT - BUTTONHEIGHT,
        BUTTONWIDTH,
        BUTTONHEIGHT,
        "Colorselector",
    );

    // check box for color fill
    let mut fill = button::CheckButton::new(
        PROGRAMMWIDTH - BUTTONWIDTH * 5 + 250,
        PROGRAMMHEIGHT - BUTTONHEIGHT,
        50,
        BUTTONHEIGHT,
        "Fill",
    );


    // canvas
    let cvs: canvas::Canvas =
        canvas::Canvas::new(0, 0, PROGRAMMWIDTH, PROGRAMMHEIGHT - BUTTONHEIGHT, "");

    win.end();
    win.show();

    // if any RadioRoundButton is toggled they should use the toggled function
    rad1.set_callback({
        let mut clone = cvs.clone();
        move |_| {
            let cvs_clone = clone.borrow_mut();
            cvs_clone.toggle((true, false, false));
        }
    });

    rad2.set_callback({
        let mut clone = cvs.clone();
        move |_| {
            let cvs_clone = clone.borrow_mut();
            cvs_clone.toggle((false, true, false));
        }
    });

    rad3.set_callback({
        let mut clone = cvs.clone();
        move |_| {
            let cvs_clone = clone.borrow_mut();
            cvs_clone.toggle((false, false, true));
        }
    });

    colorbtn.set_callback({
        let mut clone = cvs.clone();
        move |_| {
            let cvs_clone = clone.borrow_mut();
            let color = color_chooser("Color selector", ColorMode::Rgb);
            cvs_clone.set_color(color.unwrap());
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

    fill.set_callback({
        let mut clone = cvs.clone();
        let fill = fill.clone();
        move |_| {
            let cvs_clone = clone.borrow_mut();
            cvs_clone.set_fill(fill.value());
        }
    });

    a.run().unwrap();
}
