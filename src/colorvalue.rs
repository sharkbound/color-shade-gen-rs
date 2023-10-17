use std::rc::Rc;
use eframe::egui as egui;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ColorValue {
    hex: String,
}

impl ColorValue {
    pub fn step(&self, amount: f64) -> Self {}
}
