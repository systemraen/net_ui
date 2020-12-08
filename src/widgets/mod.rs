pub mod prelude;
pub mod button;
use quicksilver::{Graphics, graphics::Color};

pub trait Widget {
	fn draw(&self, gfx: &mut Graphics, color: Color);
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