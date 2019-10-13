use crate::element;

pub enum EventType {
    EnterFrame
}

pub struct EventManager<F: Fn(&mut element::Element)> {
    pub enterframe: Vec<F>
}

impl<F: Fn(&mut element::Element)> EventManager<F> {
    pub fn on(&self, event: EventType, job: F) {
        match event {
            EventType::EnterFrame => {
                self.enterframe.push(job);
            },
            // _ => unimplemented!()
        }
    }

    pub fn flare(&self, element: &mut element::Element) {
        // enterframe を発火
        self.enterframe.iter().for_each(|job| job(element));
        // TODO: その他イベントも発火する
    }
}
