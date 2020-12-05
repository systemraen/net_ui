pub trait Widget {
	fn draw(&self);
	fn reposition(&self);
	fn enable(&self);
	fn disable(&self);
	fn toggle_int(&self); //interactability 
}

pub struct WidgetData {
	x: u32,
	y: u32,
	w: u32,
	h: u32
}