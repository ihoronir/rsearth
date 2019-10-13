use crate::app;
use crate::event;
use crate::geom;

// Element!

pub trait Element {
    //fn addChild(&self, child: Box<Element>);
    fn update(&self, app: app::App);
    fn draw(&self, app: app::App);
}

// TODO: element::Line に切り分け

pub struct Line<F: Fn(&mut Element)> {
    // 共通実装
    //children: Vec<Box<Element>>,
    event: event::EventManager<F>,
    //appearance

    points: Vec<geom::Vector2>
}

impl<F: Fn(&mut Element)> Line<F> {
    pub fn addPoint(&mut self, point: geom::Vector2) {
        self.points.push(point);
    }
}

impl<F: Fn(&mut Element)> Element for Line<F> {
    //fn addChild(&mut self, child: Box<Element>) {
    //    self.children.push(child);
    // }
    fn update(&self, app: app::App/*, イベントオブジェクト的な何か */) {
        // 自分の子供を全て update
        self.event.flare(&mut self);
    }
    fn draw(&self, app: app::App/*, コネクション */) {
        unimplemented!();
        // 自分を draw
        // 自分の子供を全て draw
    }
}


//impl Line {
//
//  // Constructor
//  pub fn old_new(app: &app::App) -> Self {
//    let connection = app.connection;
//    let screen = app.screen;
//    let window = app.window;
//
//    let gcontext = connection.generate_id();
//    xcb::create_gc(
//      &connection,
//      gcontext,
//      screen.root(),
//      &[
//        (xcb::GC_FOREGROUND        , screen.black_pixel()),
//        (xcb::GC_GRAPHICS_EXPOSURES, 0)
//      ]
//    ); 
//
//      Self {
//        position: Position { x: 0.0, y: 0.0 },
//        points: vec![],
//        gcontext: gcontext
//      }
//  }
//
//  // Render
//  pub fn render(&self) {
//    let len = self.points.len() as u32;
//    let points: Vec<xcb::draw::Point> = self.points.iter()
//      .map(|point| point.to_xcb_point()).collect();
//
//    let points_ptr = points.as_ptr() as *const xcb::draw::Point;
//
//    unsafe {
//      xcb::draw::poly_line(
//        self.connection,
//        xcb::draw::COORD_MODE_ORIGIN,
//        self.window,
//        self.gcontext,
//        len,
//        points_ptr
//      );
//    }
//  }
//
//  /// Set foreground colorpixel
//  pub fn set_foreground(&mut self, value: u32) {
//    let values = [value];
//    let value_list = values.as_ptr() as *const u32;
//    self.set_gc_values(
//      xcb::draw::GC_FOREGROUND,
//      value_list
//    );
//  }
//
//  /// Set whether to ExposureEvents should be generated (1) or not (0).
//  pub fn set_graphics_exposures(&mut self, value: u32) {
//    let values = [value];
//    let value_list = values.as_ptr() as *const u32;
//    self.set_gc_values(
//      xcb::draw::GC_GRAPHICS_EXPOSURES,
//      value_list
//    );
//  }
//
//  fn set_gc_values(
//    &mut self,
//    value_mask: u32,
//    value_list: *const u32
//  ) {
//    unsafe {
//      xcb::draw::change_gc(
//        self.connection,
//        self.gcontext,
//        value_mask,
//        value_list
//      );
//    }
//  }
//}


