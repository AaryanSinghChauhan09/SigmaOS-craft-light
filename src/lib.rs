#[derive(Debug, Clone)]
pub struct ExposureConfig { pub ev: f32 }
impl ExposureConfig {
    pub fn new(ev: f32) -> Self { Self { ev } }
    pub fn scale(&self) -> f32 { 2.0_f32.powf(self.ev) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_exposure() {
        let e = ExposureConfig::new(2.0);
        assert_eq!(e.scale(), 4.0);
    }
}
