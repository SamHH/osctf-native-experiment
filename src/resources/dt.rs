use std::default::Default;

pub struct DeltaTime(pub f64);

impl Default for DeltaTime {
    fn default() -> DeltaTime { DeltaTime(0f64) }
}
