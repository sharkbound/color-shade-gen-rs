use eframe::egui as egui;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    RGB { r: u8, g: u8, b: u8 },
    HSV { hue: u8, saturation: u8, value: u8 },
    HSL { hue: u8, saturation: u8, luminance: u8 },
}


impl Color {
    pub fn step(&self, amount: u8) -> Color {
        match self {
            Color::RGB { .. } => { todo!() }
            Color::HSV { .. } => { todo!() }
            Color::HSL { hue, saturation, luminance } => {
                match luminance.checked_add(amount) {
                    Some(new_luminance) => Color::HSL {
                        hue: *hue,
                        saturation: *saturation,
                        luminance: new_luminance,
                    },
                    None => self
                }
            }
        }
    }
}
