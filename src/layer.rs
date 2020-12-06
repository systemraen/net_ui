use {crate::widgets::prelude::Widget, quicksilver::Graphics};

#[derive(Default)]
pub struct Layer {
	pub widgets: Vec<Box<&'static Widget>>,
}

impl Layer {
	pub fn new() -> Self {
		Layer { widgets: vec![] }
	}

	pub fn draw(&self, gfx: &mut Graphics) {
		for w in &self.widgets {
			w.draw(gfx);
		}
	}
}
