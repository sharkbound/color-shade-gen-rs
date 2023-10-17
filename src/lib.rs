pub mod color_row;
pub mod colorvalue;
pub mod color_conversions;
mod stepmode;
mod clamp_to_percent;

use eframe::{App, Frame};
use eframe::egui::Context;
use eframe::egui as egui;
use eframe::egui::color_picker::Alpha;
use crate::color_row::ColorRow;

pub struct MainApp {
    color_rows: Vec<ColorRow>,
}

impl MainApp {
    pub fn name() -> &'static str {
        "Color Shade Generator"
    }

    pub fn run(&self) -> eframe::Result<()> {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            Self::name(),
            options,
            Box::new(|_cc| Box::new(MainApp::default())),
        )
    }
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            color_rows: vec![],
        }
    }
}

impl App for MainApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    egui::color_picker::show_color(ui, self.color_32, egui::Vec2::new(10f32, 10f32))
                    // egui::color_picker::color_picker_color32(ui, &mut self.color_32, Alpha::BlendOrAdditive);
                });
            });
        });
    }
}

