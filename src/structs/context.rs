use {
	crate::Layer,
	quicksilver::{graphics::Color, Graphics},
};

pub struct Context<'a,'b: 'a> {
	layers: Vec<&'a Layer<'b>>, // i want this to live as long as the vec contains it
}

impl<'a,'b: 'a> Context<'a,'b> {
	pub fn new() -> Self {
		Context { layers: vec![] }
	}

	pub fn add_layer(&mut self, layer: &'static Layer) {
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
