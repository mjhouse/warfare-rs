use std::collections::HashMap;
use crate::area::{Area,Soil,Biome,bounds};

use rand::Rng;
use noise::{Perlin,NoiseFn,Clamp,Seedable};

fn generate_elevation(width: i32, height: i32, seed: u32, factor: f32) -> Vec<f32> {
    let w = width as f64;
    let h = height as f64;

    // offset values from [-1,1] to [1,2]
    let offset = noise::Constant::new(1.0);

    // set up ridges/mountains
    let base1 = noise::RidgedMulti::new().set_seed(seed);

    let ridges = noise::ScalePoint::new(
        noise::Add::new(&base1,&offset))
        .set_x_scale(0.015 / (w / 30.0))
        .set_y_scale(0.015 / (h / 30.0));


    // set up rolling hills
    let base2 = noise::SuperSimplex::new().set_seed(seed);

    let hills = noise::ScalePoint::new(
        noise::Add::new(&base2,&offset))
        .set_x_scale(0.015)
        .set_y_scale(0.015);

    // multiply noise together for final generator
    let noise = noise::Multiply::new(&ridges,&hills);

    let mut elevation = vec![];

    let max = bounds::MAX_ELEV;
    let min = bounds::MIN_ELEV;

    for y in 0..height {
        for x in 0..width {
            let mut v = noise.get([
                x as f64 * std::f64::consts::PI,
                y as f64 * std::f64::consts::PI,
            ]) as f32;

            // normalize between 0 and 1
            v = v / 4.0;

            // scale between min and max
            v = (v * (max + min.abs())) - min.abs();

            // scale by given factor and add
            elevation.push(v * factor);
        }
    }

    elevation
}

fn get_elevation(elevation: &Vec<f32>, index: i32) -> f32 {
    use std::convert::TryFrom;
    let mut value = f32::MAX;
    if let Ok(i) = usize::try_from(index){ // non-negative
        if let Some(m) = elevation.get(i).cloned() { // position exists
            value = m;
        }
    }
    value
}

fn get_moisture(moisture: &Vec<u8>, index: i32) -> u8 {
    use std::convert::TryFrom;
    let mut value = 100;
    if let Ok(i) = usize::try_from(index){ // non-negative
        if let Some(m) = moisture.get(i).cloned() { // position exists
            value = m.min(100).max(0);
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
            let mut local = vec![];
            
            // only include triplet if the point (tl, tr etc) is on the board (in expected row)
            if r0 == k0 { local.push(( tl, get_elevation(&elevation,tl), get_moisture(&moisture,tl) )); }
            if r0 == k1 { local.push(( tr, get_elevation(&elevation,tr), get_moisture(&moisture,tr) )); }
            if r1 == k2 { local.push(( mr, get_elevation(&elevation,mr), get_moisture(&moisture,mr) )); }
            if r2 == k3 { local.push(( br, get_elevation(&elevation,br), get_moisture(&moisture,br) )); }
            if r2 == k4 { local.push(( bl, get_elevation(&elevation,bl), get_moisture(&moisture,bl) )); }
            if r1 == k5 { local.push(( ml, get_elevation(&elevation,ml), get_moisture(&moisture,ml) )); }

            // get elevation and moisture for current point
            let e = get_elevation(&elevation,pt);
            let mut m = get_moisture(&moisture,pt);

            // filter out triplets that are above the current tile
            // or are 100% water.
            local = local
                .into_iter()
                .filter(|l| l.1 < e && l.2 < 100)
                .collect::<Vec<(i32,f32,u8)>>();

            // sort lowest elevation to highest
            local.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

            for p in local.iter_mut() {
                // find the remaining space in tile
                let r = 100 - p.2.min(100);

                // select either the remainder or all
                let n = r.min(m);

                // add the change to the total
                let t = p.2 + n;

                // update water for the local point
                if let Ok(j) = usize::try_from(p.0){
                    if let Some(v) = moisture.get_mut(j) {
                        // subtract from total
                        m -= n;

                        // add to value for point
                        *v = t;

                        // update metrics
                        moved += n as usize;
                    }
                }

                // exit early if all moisture has
                // been distributed
                if m == 0 { 
                    break; 
                }
            }

            // set the remainder back to the current point
            if let Some(v) = moisture.get_mut(i) {
                *v = m;
            }
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
    
    let elevations = generate_elevation(width,height,789654,0.5);
    let moistures = generate_moisture(width,height,8,&elevations);

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