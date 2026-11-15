use sigmaos_craft_light::{Light, LightType, LightingManager};

#[test]
fn lighting_manager_integration_aaa() {
    let mut lm = LightingManager::new();
    let l = Light { id: "int1".into(), kind: LightType::Directional, color: [255, 255, 255],
        intensity: 1.0, position: None, direction: Some([0.0, -1.0, 0.0]), cast_shadows: true };
    lm.add_light(l).unwrap();
    assert_eq!(lm.lights().len(), 1);
    assert_eq!(lm.lights_by_type(LightType::Directional).len(), 1);
}
