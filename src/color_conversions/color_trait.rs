use eframe::egui::Color32;

pub trait ColorType where T: Clone, T: Copy {
    type FromHexTo;
    fn from_hex(hex: &str) -> Option<Self::FromHexTo>;
    fn to_hex(&self) -> String;
    fn to_color_32(&self) -> Color32;
}

