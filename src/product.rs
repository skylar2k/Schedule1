mod property;
use property::Property;

pub struct Product {
    base_price: f32,
    properties: [Property; 8],
}

impl Product {
    pub fn calculate_value(&self) -> f32 {
        let mut value_multiplier = 0.0;
        for property in &self.properties {
            
        }
        0.0
    }
}
