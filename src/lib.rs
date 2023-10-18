pub mod color_ramp;
mod clamp_to_percent;

use std::default::Default;
use eframe::{App, Frame};
use eframe::egui::Context;
use eframe::egui as egui;
use crate::color_ramp::ColorRamp;

pub struct MainApp {
    color_rows: Vec<ColorRamp>,
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
            // debug
            color_rows: vec![Default::default(), Default::default()],
        }
    }
}

impl App for MainApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                for row in self.color_rows.iter() {
                    ui.horizontal(|ui| {
                        row.render(ui);
                    });
                }
            });
        });
    }
}

