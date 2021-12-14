use crate::generation::{Biome, Soil};

// TODO: add Debug
#[derive(Clone)]
pub struct Factors {
    pub elevation: u8,
    pub temperature: u8,
    pub moisture: u8,
    pub rockiness: u8,
    pub fertility: u8,
    pub biome: Biome,
    pub soil: Soil,
}

impl Default for Factors {
    fn default() -> Self {
        Self {
            elevation: 50,
            temperature: 50,
            moisture: 50,
            rockiness: 50,
            fertility: 50,
            biome: Biome::None,
            soil: Soil::None,
        }
    }
}
