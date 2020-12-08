use {super::Widget, crate::structs::widget_data::WidgetData /* (idk...) */, quicksilver::{Graphics, graphics::Color}};

pub struct Button {
	pub data: WidgetData,
}

impl Widget for Button {
	fn draw(&self, gfx: &mut Graphics, color: Color) {
		gfx.stroke_rect(&self.data.rect, color);
	}
}
