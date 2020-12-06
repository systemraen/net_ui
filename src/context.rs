use { crate::layer::Layer, quicksilver::graphics::Graphics };

pub struct Context {
	layers: Vec<&'static Layer>,
}

impl Context {
	pub fn new() -> Self {
		Context { layers: vec![] }
	}

	pub fn add_layer(&mut self, layer: &'static Layer) {
		self.layers.push(layer);
	}

	pub fn clear_layers(&mut self) {
		self.layers.clear();
	}

	pub fn draw(&self, gfx: &mut Graphics) {

	}
}
