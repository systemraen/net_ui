use crate::layer::Layer;

pub struct Context {
	layers: Vec<Layer>
}

impl Context {
	pub fn new() -> Self {
		Context {
			layers: vec![]
		}
	}

	pub fn add_layer(&mut self, layer: Layer) {
		self.layers.push(layer);
	}
}