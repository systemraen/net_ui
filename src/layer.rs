use crate::widgets::prelude::Widget;

#[derive(Default)]
pub struct Layer {
	pub widgets: Vec<Box<&'static Widget>>
}

impl Layer {
	pub fn new() -> Self {
		Layer {
			widgets: vec![]
		}
	}
}