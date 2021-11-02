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

fn generate_perlin(noise: &Perlin, x: i32, y: i32, scale: f64) -> f32 {
    let mut v = noise.get([
        (x as f64 * std::f64::consts::PI) * scale,
        (y as f64 * std::f64::consts::PI) * scale,
    ]) as f32;

    // normalize between 0.0 and 1.0
    v = (v + 1.0) / 2.0;

    v
}

fn generate_elevation(width: i32, height: i32, seed: u32, mut factor: f32) -> Vec<f32> {
    let mut elevation = vec![];
    let mut png = Perlin::new()
        .set_seed(seed);

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;

            let v1 = generate_perlin(&png,x,y,1.0/50.0);
            let v2 = generate_perlin(&png,x,y,1.0/25.0);
            let v3 = generate_perlin(&png,x,y,1.0/12.5);
            let v4 = generate_perlin(&png,x,y,1.0/6.25);

            let m1 = bounds::MIN_ELEV;
            let m2 = bounds::MAX_ELEV;

            let v = (v1/1.0 + v2/2.0 + v3/4.0 + v4/8.0)/4.0;
            let f = factor.min(100.0).max(0.0) / 100.0;
            elevation.push(v * f * m2);
        }
    }

    elevation
}

pub fn weighted(icons: HashMap<Soil,usize>, width: i32, height: i32) -> Vec<Area> {
    let mut results = vec![];
    // let mut rng = rand::thread_rng();
    
    let elevations = generate_elevation(width,height,12345678,100.0);

    let mut c = 0;

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;

            let location = (x,y);
            let biome = Biome::Grassland;
            let soil = Soil::Loam;
            let moisture = 0;
            let rocks = 0;
            let fertility = 0;
            let elevation = elevations[c];
            let temperature = 0.0;
            let texture = icons[&soil];

            c += 1;

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