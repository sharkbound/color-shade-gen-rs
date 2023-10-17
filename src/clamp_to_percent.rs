trait ClampToPercent<T> {
    fn clamp_to_percent(&self) -> T;
    fn to_f64(&self) -> f64;
}

// region Impls for ClampToPercent
impl ClampToPercent<f64> for f64 {
    fn clamp_to_percent(&self) -> f64 {
        self.min(1.0).max(0.0)
    }

    fn to_f64(&self) -> f64 {
        return *self;
    }
}

impl ClampToPercent<f32> for f32 {
    fn clamp_to_percent(&self) -> f32 {
        self.min(1.0).max(0.0)
    }
    fn to_f64(&self) -> f64 {
        return *self as f64;
    }
}

impl ClampToPercent<i32> for i32 {
    fn clamp_to_percent(&self) -> i32 {
        *self.min(&100).max(&0)
    }

    fn to_f64(&self) -> f64 {
        return (*self as f64 / 100.0).clamp_to_percent();
    }
}
// endregion