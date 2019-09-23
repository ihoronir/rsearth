use rsearth::{app::App, xcb, element};

fn main() {
  let mut app = App::new();
  let mut line = element::Line::new(&app);
  let line_point1 = element::Position {
    x: 5.0,
    y: 20.0
  };
  let line_point2 = element::Position {
    x: 50.0,
    y: 10.0
  };
  line.points.append(&mut vec![line_point1, line_point2]);

  app.elements.append(&mut vec![line]);
  app.start();
}
