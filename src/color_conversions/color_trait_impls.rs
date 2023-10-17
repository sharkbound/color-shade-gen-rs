use eframe::egui::Color32;
use crate::color_conversions::ColorType;

impl ColorType for Color32 {
    type FromHexTo = Color32;

    fn from_hex(hex: &str) -> Option<Color32> {
        todo!()
    }

    fn to_hex(&self) -> String {
        todo!()
    }

    fn to_color_32(&self) -> Color32 {
        *self
    }
}