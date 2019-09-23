use std::ptr::{null};
use crate::{xcb, app};

pub struct Position {
  pub x: f64,
  pub y: f64,
}

impl Position {
  pub fn to_xcb_point(&self) -> xcb::draw::Point {
    xcb::draw::Point {
      x: self.x as i16,
      y: self.y as i16
    }
  }
}

pub struct Line {
  pub position: Position,
  pub points: Vec<Position>,
  connection: *mut xcb::Connection,
  window    : xcb::window::Window,
  gcontext  : xcb::draw::GContext
}

impl Line {
  // Constructor
  pub fn new(app: &app::App) -> Self {
    let connection = app.connection;
    let screen = app.screen;
    let window = app.window;

    unsafe {
      let root_window = (*screen).root;
      let gcontext = xcb::generate_id(app.connection);

      xcb::draw::create_gc(
        connection,
        gcontext,
        root_window,
        0, null()
      ); 

      Self {
        position: Position { x: 0.0, y: 0.0 },
        points: vec![],
        connection: connection,
        window: window,
        gcontext: gcontext
      }
    }
  }

  // Render
  pub fn render(&self) {
    let len = self.points.len() as u32;
    let points: Vec<xcb::draw::Point> = self.points.iter()
      .map(|point| point.to_xcb_point()).collect();

    let points_ptr = points.as_ptr() as *const xcb::draw::Point;

    unsafe {
      xcb::draw::poly_line(
        self.connection,
        xcb::draw::COORD_MODE_ORIGIN,
        self.window,
        self.gcontext,
        len,
        points_ptr
      );
    }
  }

  /// Set foreground colorpixel
  pub fn set_foreground(&mut self, value: u32) {
    let values = [value];
    let value_list = values.as_ptr() as *const u32;
    self.set_gc_values(
      xcb::draw::GC_FOREGROUND,
      value_list
    );
  }

  /// Set whether to ExposureEvents should be generated (1) or not (0).
  pub fn set_graphics_exposures(&mut self, value: u32) {
    let values = [value];
    let value_list = values.as_ptr() as *const u32;
    self.set_gc_values(
      xcb::draw::GC_GRAPHICS_EXPOSURES,
      value_list
    );
  }

  fn set_gc_values(
    &mut self,
    value_mask: u32,
    value_list: *const u32
  ) {
    unsafe {
      xcb::draw::change_gc(
        self.connection,
        self.gcontext,
        value_mask,
        value_list
      );
    }
  }
}

