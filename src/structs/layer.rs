use {crate::widgets::Widget, quicksilver::graphics::Color, quicksilver::Graphics};

#[derive(Default)]
pub struct Layer {
	pub widgets: Vec<Box<dyn Widget>>,
}

impl Layer {
	pub fn new() -> Self {
		Layer { widgets: vec![] }
	}

	pub fn draw(&self, gfx: &mut Graphics, color: Color) {
		for w in &self.widgets {
			w.draw(gfx, color);
		}
	}
}
