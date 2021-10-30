use std::collections::HashMap;
use crate::area::{Area,Soil,Biome};

use rand::Rng;

pub fn random(icons: HashMap<Soil,usize>, width: i32, height: i32) -> Vec<Area> {
    let mut results = vec![];
    let mut rng = rand::thread_rng();

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;

            let location = (x,y);
            let biome = rng.gen();
            let soil = rng.gen();
            let moisture = rng.gen::<u8>().min(100);
            let rocks = rng.gen::<u8>().min(100);
            let fertility = rng.gen::<u8>().min(100);
            let elevation = rng.gen_range::<f32,_>(0.0..1000.0);
            let temperature = rng.gen_range::<f32,_>(0.0..50.0);
            let texture = icons[&soil];

            let area = Area::create()
                .with_texture(texture)
                .with_location(location)
                .with_biome(biome)
                .with_soil(soil)
                .with_moisture(moisture)
                .with_rocks(rocks)
                .with_fertility(fertility)
                .with_elevation(elevation)
                .with_temperature(temperature)
                .build();

            results.push(area);
        }
    }

    results
}