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

            let mn = bounds::MIN_ELEV;
            let mx = bounds::MAX_ELEV;

            let v = (v1/1.0 + v2/2.0 + v3/4.0 + v4/6.0)/4.0;
            let f = factor.min(1.0).max(0.0);

            let v = (v * (mx + mn.abs())) - mn.abs();

            elevation.push(v * f);
        }
    }

    elevation
}

fn update_moisture(moisture: &mut Vec<u8>, index: i32, value: u8, r: i32, k: i32) {
    use std::convert::TryFrom;
    if r == k { // in correct row
        if let Ok(i) = usize::try_from(index){
            if let Some(m) = moisture.get_mut(i) {
                *m = value;
            }
        }
    }
}

fn find_elevation(elevation: &Vec<f32>, index: i32, r: i32, k: i32) -> f32 {
    use std::convert::TryFrom;
    let mut value = f32::MAX;
    if r == k { // in correct row
        if let Ok(i) = usize::try_from(index){ // non-negative
            if let Some(m) = elevation.get(i).cloned() { // position exists
                value = m;
            }
        }
    }
    value
}

fn find_moisture(moisture: &Vec<u8>, index: i32, r: i32, k: i32) -> u8 {
    use std::convert::TryFrom;
    let mut value = 100;
    if r == k { // in correct row
        if let Ok(i) = usize::try_from(index){ // non-negative
            if let Some(m) = moisture.get(i).cloned() { // position exists
                value = m.min(100).max(0);
            }
        }
    }
    value
}

fn generate_moisture(width: i32, height: i32, initial: u8, elevation: &Vec<f32>) -> Vec<u8> {
    use std::convert::TryFrom;
    use std::cmp::Ordering;
    
    let mut moisture = vec![initial;elevation.len()];
    let length = elevation.len();

    let mut cycle = 0;
    let mut moved = 0;

    loop {
        for i in 0..length {
            // surrounding points
            let pt = i as i32;
            let mut tl = pt + width;
            let mut tr = pt + width + 1;
            let mr = pt + 1;
            let mut br = pt - width + 1;
            let mut bl = pt - width;
            let ml = pt - 1;
    
            // expected rows
            let r0 = pt / width + 1;
            let r1 = pt / width;
            let r2 = pt / width - 1;
    
            if r1 % 2 != 0 {
                tl -= 1;
                tr -= 1;
                br -= 1;
                bl -= 1;
            }
    
            // actual rows
            let k0 = tl / width;
            let k1 = tr / width;
            let k2 = mr / width;
            let k3 = br / width;
            let k4 = bl / width;
            let k5 = ml / width;
    
            // create a local group of ( index, elevation, moisture ) triplets
            let mut local = vec![
                ( pt, find_elevation(&elevation,pt,r1,r1), find_moisture(&moisture,pt,r1,r1) ),
                ( tl, find_elevation(&elevation,tl,r0,k0), find_moisture(&moisture,tl,r0,k0) ),
                ( tr, find_elevation(&elevation,tr,r0,k1), find_moisture(&moisture,tr,r0,k1) ),
                ( mr, find_elevation(&elevation,mr,r1,k2), find_moisture(&moisture,mr,r1,k2) ),
                ( br, find_elevation(&elevation,br,r2,k3), find_moisture(&moisture,br,r2,k3) ),
                ( bl, find_elevation(&elevation,bl,r2,k4), find_moisture(&moisture,bl,r2,k4) ),
                ( ml, find_elevation(&elevation,ml,r1,k5), find_moisture(&moisture,ml,r1,k5) ),
            ];
    
            // find the lowest elevation point
            let mut lowest = local
                .iter()
                .min_by(|a,b| a.1
                    .partial_cmp(&b.1)
                    .unwrap())
                .cloned()
                .unwrap();
    
            // keep all triplets that equal the lowest and have < max moisture.
            local.retain(|v| (v.1 - lowest.1) < 0.1 && v.2 <= 100);
    
            // get moisture for the current point
            let mut m = moisture[i];
            let k = m / local.len() as u8;
    
            // divide the current point's moisture among the local
            // group of lowest points
            if local.len() > 0 {
                for point in local.iter() {
                    // find the remaining space in tile
                    let r = 100 - point.2.min(100);

                    // select either the remainder or portion
                    let n = r.min(k);

                    // add the change to the total
                    let t = point.2 + n;

                    // update the moisture for the neighbor
                    update_moisture(&mut moisture,point.0,t,0,0);

                    // subtract from total
                    m -= n;

                    // keep a measure of the amount distributed
                    if point.0 != pt {
                        moved += n as usize;
                    }
                }
            }

            // put the remainder back on the current point
            update_moisture(&mut moisture,pt,m,0,0);
        }

        if cycle > 20 || moved == 0 {
            break;
        }

        cycle += 1;
        moved = 0;
    }

    moisture
}

pub fn weighted(icons: HashMap<Soil,usize>, width: i32, height: i32) -> Vec<Area> {
    let mut results = vec![];
    // let mut rng = rand::thread_rng();
    
    let elevations = generate_elevation(width,height,12345678,1.0);
    let moistures = generate_moisture(width,height,32,&elevations);

    let mut c = 0;

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;

            let location = (x,y);
            let biome = Biome::Grassland;
            let soil = Soil::Loam;
            let moisture = moistures[c];
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