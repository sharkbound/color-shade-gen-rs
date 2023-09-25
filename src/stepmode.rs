#[derive(Default)]
pub enum StepMode {
    #[default]
    Luminosity,
    Hue,
    Saturation,
    Value,
}
