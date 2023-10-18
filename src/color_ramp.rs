use eframe::egui;
use eframe::egui::Color32;

pub struct ColorRamp {
    origin_color: Color32,
}

impl ColorRamp {
    pub fn render(&self, ui: &mut egui::Ui) {
        let mut val = false;
        ui.checkbox(&mut val, "Test Text");
    }
}

impl Default for ColorRamp {
    fn default() -> Self {
        Self {
            origin_color: Color32::from_rgb(255, 255, 255),
        }
    }
}

