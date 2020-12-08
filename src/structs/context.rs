use {
	crate::Layer,
	quicksilver::{graphics::Color, Graphics},
};

pub struct Context {
	layers: Vec<Layer>, // i want this to live as long as the vec contains it
}

impl Context {
	pub fn new() -> Self {
		Context { layers: vec![] }
	}

	pub fn add_layer(&mut self, layer: Layer) {
		self.layers.push(layer);
	}

	pub fn clear_layers(&mut self) {
		self.layers.clear();
	}

	pub fn draw(&self, gfx: &mut Graphics, color: Color) {
		// maybe check some sort of clock, from qs maybe?

		for l in &self.layers {
			l.draw(gfx, color);
		}
	}
}
