use eframe::egui as egui;
use crate::stepmode::StepMode;

pub struct ColorRow {
    color_32: egui::Color32,
    step_mode: StepMode,
}

