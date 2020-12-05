pub trait Widget {
	fn draw(&self);
	fn reposition(&self, x: u32, y: u32) {
		//self.x; #todo: figure out how to use WidgetData here
	}
	fn enable(&self) {}
	fn disable(&self) {}
	fn toggle_active(&self) {} //interactability 
	fn toggle_vis(&self) {}
	fn hide(&self) {}
	fn show(&self) {}
}

pub struct WidgetData {
	x: u32,
	y: u32,
	w: u32,
	h: u32
}

/*
todo
inject js into patorjk.com 
*/