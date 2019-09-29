use std::time::{Instant, Duration};
use std::thread::sleep;
use crate::scene;

// TODO: AppBuilder 作る

pub struct App<'a> {
    connection: xcb::Connection,
    screen: xcb::Screen<'a>,
    window: xcb::Window,
    pub elements: Vec<scene::Scene>
}

impl<'a> App<'a> {

    /// Constructor
    pub fn new() -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).expect("Failed to connect.");
        let setup = connection.get_setup();
        let screen: xcb::Screen<'a> = setup.roots().nth(screen_num as usize).expect("Failed to get root");

        let window = connection.generate_id();
        xcb::create_window(
            &connection,
            xcb::COPY_FROM_PARENT as u8,
            window,
            screen.root(),
            0, 0,
            150, 150,
            10,
            xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
            screen.root_visual(),
            &[(xcb::CW_BACK_PIXEL, screen.white_pixel()),
              (xcb::CW_EVENT_MASK, xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_KEY_PRESS)]
        );
        xcb::map_window(&connection, window);

        connection.flush();

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

            self.elements.iter().for_each(|element| element.render(self.connection));
            self.connection.flush();
            let dur = start.elapsed();
            sleep(one_frame - dur);
        }
    }
}
