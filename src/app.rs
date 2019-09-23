use std::ptr::{null, null_mut};
use std::time::{Instant, Duration};
use std::thread::sleep;
use crate::{xcb, element};

pub struct App {
  pub connection: *mut xcb::Connection,
  pub screen: *mut xcb::Screen,
  pub window: xcb::window::Window,
  pub elements: Vec<element::Line>
}

impl App {
  pub fn new() -> Self {
    // Open the connect to the X server
    let connection = unsafe { xcb::connect(null(), null_mut()) };

    // Get the first screen
    let screen = unsafe { xcb::setup_roots_iterator(xcb::get_setup(connection)).data };
    
    // Create a window
    let window = unsafe { xcb::generate_id(connection) };
    let mask = xcb::window::CW_BACK_PIXEL | xcb::window::CW_EVENT_MASK;
    let values = unsafe { [(*screen).white_pixel, xcb::event::EVENT_MASK_EXPOSURE] };
    unsafe {
      xcb::window::create_window(
        connection,
        xcb::window::COPY_FROM_PARENT,
        window,
        (*screen).root,
        0, 0,
        150, 150,
        10,
        xcb::window::WINDOW_CLASS_INPUT_OUTPUT,
        (*screen).root_visual,
        mask, values.as_ptr() as * const u32
      );

      xcb::window::map_window(connection, window);

      xcb::flush(connection);
    }

    Self {
      connection,
      screen,
      window,
      elements: vec![]
    }
  }

  /// Start app
  pub fn start(&self) {
    let one_frame = Duration::from_secs(1) / 30;
    
    loop {
      let start = Instant::now();
      self.elements.iter().for_each(|element| element.render());
      unsafe {xcb::flush(self.connection)};
      let dur = start.elapsed();
      sleep(one_frame - dur);
    }
  }
}
