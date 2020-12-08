pub use quicksilver::geom::{Rectangle, Vector};

pub struct WidgetData {
	pub rect: Rectangle,
	pub visible: bool,
	pub enabled: bool
}

impl WidgetData {
	pub fn new(x: f32, y: f32, w: f32, h: f32, visible: bool, enabled: bool) -> Self {
		WidgetData {
			rect: Rectangle::new(Vector::new(x, y), Vector::new(w, h)),
			visible: visible,
			enabled: enabled
		}
	}
}