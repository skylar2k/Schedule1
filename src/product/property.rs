use core::todo;
use vector2::Vector2;

#[derive(Debug)]
pub struct Property {
    name: String,
    tier: u32,
    addictiveness: f32,
    pub value_change: i32,
    pub value_multiplier: f32,
    pub add_base_value_multiple: f32,
    mix_direction: Vector2,
    mix_magnitude: f32,
}

impl Property {
    pub fn new(
        name: String,
        tier: u32,
        addictiveness: f32,
        add_base_value_multiple: f32,
        mix_direction: Vector2,
        mix_magnitude: f32,
    ) -> Self {
        Self {
            name,
            tier,
            addictiveness,
            add_base_value_multiple,
            mix_direction,
            mix_magnitude,
            ..Property::default()
        }
    }
}

impl Default for Property {
    fn default() -> Property {
        Property {
            name: "".to_string(),
            tier: 1,
            addictiveness: 0.1,
            value_change: 0,
            value_multiplier: 1.0,
            add_base_value_multiple: 0.0,
            mix_direction: Vector2::default(),
            mix_magnitude: 1.0,
        }
    }
}

#[test]
fn test_property() {
    let property = Property::new(
        "Anti-gravity".to_string(),
        5,
        0.611,
        0.54,
        Vector2::new(0.3085047, -0.95122284),
        3.111784,
    );
    println!("{property:?}");
}
