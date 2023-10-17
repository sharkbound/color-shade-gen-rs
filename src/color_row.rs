use eframe::egui as egui;
use crate::stepmode::StepMode;

pub struct ColorRow {
    color_32: ColorValue,
    step_mode: StepMode,
}

