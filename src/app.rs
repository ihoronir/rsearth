use std::time::{Instant, Duration};
use std::thread::sleep;

use crate::element;

// TODO: AppBuilder 作る

pub struct App {
    connection: xcb::Connection,
    screen_num: i32,
    window: xcb::Window,
    pub scene: Scene
}

impl App {

    /// Constructor
    pub fn new(scene: Scene) -> Self {
        let (connection, screen_num) = xcb::Connection::connect(None).expect("Failed to connect.");

        let setup = connection.get_setup();
        let screen: xcb::Screen = setup.roots().nth(screen_num as usize).expect("Failed to get root");

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
            screen_num,
            window,
            scene,
        }
    }

    /// Start app
    pub fn start(&self) {
        let one_frame = Duration::from_secs(1) / 30;

        loop {
            let start = Instant::now();

            //self.elements.iter().for_each(|element| element.render(self.connection));
            self.connection.flush();
            let dur = start.elapsed();
            sleep(one_frame - dur);
        }
    }
}


pub struct Scene {
    elements: Vec<Box<element::Element>>
}

impl Scene {
    fn render(&self, app: App) {
        self.elements.iter().for_each(|element| element.update(app));
        self.elements.iter().for_each(|element| element.draw(app));
    }
}
