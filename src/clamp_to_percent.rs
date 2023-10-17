trait ClampToPercent {
    type Output;
    fn clamp_to_percent(&self) -> Self::Output;
    fn to_f64(&self) -> f64;
}

macro_rules! impl_clamp_to_percent_non_float {
    ($t:ty) => {
        impl ClampToPercent<$t> for $t {
            type Output = $t;

            fn clamp_to_percent(&self) -> Self::Output {
                *self.max(&100).min(&0)
            }

            fn to_f64(&self) -> f64 {
                self.clamp_to_percent() as f64 / 100
            }
        }
    }
}

impl_clamp_to_percent_non_float!(i32);
impl_clamp_to_percent_non_float!(u32);
impl_clamp_to_percent_non_float!(i64);
impl_clamp_to_percent_non_float!(u64);

// region Impls for ClampToPercent
impl ClampToPercent for f64 {
    type Output = f64;
    fn clamp_to_percent(&self) -> Self::Output {
        self.min(1.0).max(0.0)
    }

    fn to_f64(&self) -> f64 {
        *self
    }
}

impl ClampToPercent for f32 {
    type Output = f32;
    fn clamp_to_percent(&self) -> Self::Output {
        self.min(1.0).max(0.0)
    }
    fn to_f64(&self) -> f64 {
        *self as f64
    }
}


// endregion