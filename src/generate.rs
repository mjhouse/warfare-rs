use std::collections::HashMap;
use crate::area::{Area,Soil,Biome,bounds};

use rand::Rng;
use noise::{Perlin,NoiseFn,Clamp,Seedable};

pub fn flattened(icons: HashMap<Soil,usize>, width: i32, height: i32) -> Vec<Area> {
    let mut results = vec![];
    let mut rng = rand::thread_rng();

    let kind = Soil::Clay;

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;

            let location = (x,y);
            let biome = rng.gen();
            let soil = kind.clone();
            let moisture = rng.gen::<u8>().min(100);
            let rocks = rng.gen::<u8>().min(100);
            let fertility = rng.gen::<u8>().min(100);
            let elevation = rng.gen_range::<f32,_>(bounds::MIN_ELEV..bounds::MAX_ELEV);
            let temperature = rng.gen_range::<f32,_>(bounds::MIN_TEMP..bounds::MAX_TEMP);
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
            let elevation = rng.gen_range::<f32,_>(bounds::MIN_ELEV..bounds::MAX_ELEV);
            let temperature = rng.gen_range::<f32,_>(bounds::MIN_TEMP..bounds::MAX_TEMP);
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

pub fn noise(icons: HashMap<Soil,usize>, width: i32, height: i32) -> Vec<Area> {
    let mut results = vec![];
    let mut rng = rand::thread_rng();

    let mut png = Perlin::new();
    let mut noise = vec![];

    png.set_seed(rng.gen());

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;
            let v = png.get([
                x as f64 * std::f64::consts::PI,
                y as f64 * std::f64::consts::PI
            ]);

            noise.push((v * 5.0).round().abs() as u8);
        }
    }

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;

            let k = noise.pop().unwrap_or(0);

            let location = (x,y);
            let biome = rng.gen();
            let soil = k.into();
            let moisture = rng.gen::<u8>().min(100);
            let rocks = rng.gen::<u8>().min(100);
            let fertility = rng.gen::<u8>().min(100);
            let elevation = rng.gen_range::<f32,_>(bounds::MIN_ELEV..bounds::MAX_ELEV);
            let temperature = rng.gen_range::<f32,_>(bounds::MIN_TEMP..bounds::MAX_TEMP);
            let texture = icons.get(&soil).expect("icon doesn't exist");

            let area = Area::create()
                .with_texture(*texture)
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