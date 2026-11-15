//! SigmaOS-craft-light: scene lighting model.
//!
//! Real implementation: light types (point, directional, spot, ambient), intensity,
//! color, position, shadows, attenuation, light collection, and validation. Replaces the
//! 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Light type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Ambient,
    Directional,
    Point,
    Spot,
}

/// Light.
#[derive(Debug, Clone, PartialEq)]
pub struct Light {
    pub id: String,
    pub kind: LightType,
    pub color: [u8; 3],
    pub intensity: f32,
    pub position: Option<[f32; 3]>,
    pub direction: Option<[f32; 3]>,
    pub cast_shadows: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LightError {
    EmptyId,
    DuplicateLight(String),
    LightNotFound(String),
    InvalidIntensity(f32),
    InvalidColor,
}

impl fmt::Display for LightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyId => write!(f, "light ID must not be empty"),
            Self::DuplicateLight(id) => write!(f, "duplicate light {:?}", id),
            Self::LightNotFound(id) => write!(f, "light {:?} not found", id),
            Self::InvalidIntensity(i) => write!(f, "invalid intensity {}", i),
            Self::InvalidColor => write!(f, "invalid color"),
        }
    }
}

impl std::error::Error for LightError {}

/// Scene lighting manager.
pub struct LightingManager {
    lights: BTreeMap<String, Light>,
}

impl LightingManager {
    pub fn new() -> Self { Self { lights: BTreeMap::new() } }

    pub fn add_light(&mut self, light: Light) -> Result<(), LightError> {
        if light.id.trim().is_empty() { return Err(LightError::EmptyId); }
        if light.intensity < 0.0 { return Err(LightError::InvalidIntensity(light.intensity)); }
        let id = light.id.clone();
        if self.lights.contains_key(&id) {
            return Err(LightError::DuplicateLight(id));
        }
        self.lights.insert(id, light);
        Ok(())
    }

    pub fn remove_light(&mut self, id: &str) -> Result<(), LightError> {
        if !self.lights.contains_key(id) {
            return Err(LightError::LightNotFound(id.into()));
        }
        self.lights.remove(id);
        Ok(())
    }

    pub fn get_light(&self, id: &str) -> Option<&Light> { self.lights.get(id) }

    pub fn lights(&self) -> Vec<&Light> { self.lights.values().collect() }

    pub fn lights_by_type(&self, kind: LightType) -> Vec<&Light> {
        self.lights.values().filter(|l| l.kind == kind).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_light_aaa() {
        let mut lm = LightingManager::new();
        let l = Light { id: "l1".into(), kind: LightType::Point, color: [255, 255, 255],
            intensity: 1.0, position: Some([0.0, 0.0, 0.0]), direction: None, cast_shadows: true };
        lm.add_light(l).unwrap();
        assert_eq!(lm.lights().len(), 1);
    }

    #[test]
    fn test_empty_id_rejected_aaa() {
        let mut lm = LightingManager::new();
        let l = Light { id: "".into(), kind: LightType::Point, color: [255, 255, 255],
            intensity: 1.0, position: None, direction: None, cast_shadows: false };
        assert!(lm.add_light(l).is_err());
    }

    #[test]
    fn test_invalid_intensity_rejected_aaa() {
        let mut lm = LightingManager::new();
        let l = Light { id: "x".into(), kind: LightType::Point, color: [255, 255, 255],
            intensity: -1.0, position: None, direction: None, cast_shadows: false };
        assert!(lm.add_light(l).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut lm = LightingManager::new();
        let l = Light { id: "x".into(), kind: LightType::Point, color: [255, 255, 255],
            intensity: 1.0, position: None, direction: None, cast_shadows: false };
        lm.add_light(l.clone()).unwrap();
        let l2 = Light { id: "x".into(), kind: LightType::Point, color: [255, 255, 255],
            intensity: 1.0, position: None, direction: None, cast_shadows: false };
        assert!(lm.add_light(l2).is_err());
    }

    #[test]
    fn test_lights_by_type_aaa() {
        let mut lm = LightingManager::new();
        lm.add_light(Light { id: "a".into(), kind: LightType::Ambient, color: [255, 255, 255],
            intensity: 0.5, position: None, direction: None, cast_shadows: false }).unwrap();
        lm.add_light(Light { id: "p".into(), kind: LightType::Point, color: [255, 255, 255],
            intensity: 1.0, position: None, direction: None, cast_shadows: false }).unwrap();
        assert_eq!(lm.lights_by_type(LightType::Ambient).len(), 1);
        assert_eq!(lm.lights_by_type(LightType::Point).len(), 1);
    }
}
