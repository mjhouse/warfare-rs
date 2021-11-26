use noise::{NoiseFn,Worley,Value,SuperSimplex,Seedable};
use rand_pcg::Pcg64;
use rand::SeedableRng;
use std::collections::HashMap;

use crate::generation::{
    Factors,
    Biome,
    Soil,
    Foliage,
};

use crate::state::{Calendar,Season};
use crate::resources::Textures;
use crate::area::bounds;

#[allow(dead_code)]
#[derive(Default,Clone)]
struct Context {
    seed: u32,
    width: i32,
    height: i32,
    calendar: Calendar,
}

#[allow(dead_code)]
#[derive(Clone)]
struct Resources {
    simplex: SuperSimplex,
    worley: Worley,
    value: Value,
    random: Pcg64,
}

#[derive(Default,Clone)]
struct Values {
    elevation: HashMap<i32,f32>,
    temperature: HashMap<i32,f32>,
    moisture: HashMap<i32,u8>,
    rockiness: HashMap<i32,u8>,
    fertility: HashMap<i32,u8>,
    biome: HashMap<i32,Biome>,
    soil: HashMap<i32,Soil>,
    foliage: HashMap<i32,Foliage>,
}

#[derive(Default,Clone)]
pub struct Generator {
    resources: Resources,
    context: Context,
    factors: Factors,
    values: Values,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            simplex: SuperSimplex::new().set_seed(0),
            worley:  Worley::new().set_seed(0),
            value:   Value::new().set_seed(0),
            random:  Pcg64::seed_from_u64(0),
        }
    }
}

impl Generator {

    pub fn new( seed: u32, width: i32, height: i32, factors: Factors, calendar: Calendar ) -> Self {
        Self {
            resources: Resources {
                simplex: SuperSimplex::new().set_seed(seed),
                worley: Worley::new().set_seed(seed),
                value: Value::new().set_seed(seed),
                random: Pcg64::seed_from_u64(seed as u64),
            },
            context: Context {
                seed: seed,
                width: width,
                height: height,
                calendar: calendar,
            },
            factors: factors,
            values: Values {
                elevation: HashMap::new(),
                temperature: HashMap::new(),
                moisture: HashMap::new(),
                rockiness: HashMap::new(),
                fertility: HashMap::new(),
                biome: HashMap::new(),
                soil: HashMap::new(),
                foliage: HashMap::new(),
            },
        }
    }

    fn index( &self, mut x: i32, mut y: i32 ) -> i32 {
        let w = self.context.width;
        let h = self.context.height;
        
        x = x + w / 2;
        y = y + h / 2;
            
        x + y * w
    }

    #[allow(dead_code)]
    fn index_group( &self, x: i32, y: i32 ) -> Vec<i32> {
        let w = self.context.width;
        let h = self.context.height;
        
        let mut gp = vec![];
        
        // early return if given point is out of bounds
        if x >= (w/2) || 
           x < -(w/2) || 
           y >= (h/2) || 
           y < -(h/2) 
        {
            return gp;
        }

        // get indices from (x,y) coordinates
        let p0 = self.index(x  ,y  ); // center
        let mut p1 = self.index(x-1,y+1); // top-left
        let mut p2 = self.index(x  ,y+1); // top-right
        let mut p3 = self.index(x-1,y-1); // bot-left
        let mut p4 = self.index(x  ,y-1); // bot-right
        let p5 = self.index(x-1,y  ); // mid-left
        let p6 = self.index(x+1,y  ); // mid-right

        // expected rows
        let r0 = p0 / w + 1;
        let r1 = p0 / w;
        let r2 = p0 / w - 1;

        // alternating rows shift
        // right (hex layout)
        if r1 % 2 == 0 {
            p1 += 1;
            p2 += 1;
            p3 += 1;
            p4 += 1;
        }

        // actual rows
        let k0 = p1 / w;
        let k1 = p2 / w;
        let k2 = p3 / w;
        let k3 = p4 / w;
        let k4 = p5 / w;
        let k5 = p6 / w;

        // only include indices for points that are in 
        // the expected rows and are in-bounds
        let c1 = r0 == k0 && p1 >= 0 && p1 < (w*h);
        let c2 = r0 == k1 && p2 >= 0 && p2 < (w*h);
        let c3 = r2 == k2 && p3 >= 0 && p3 < (w*h);
        let c4 = r2 == k3 && p4 >= 0 && p4 < (w*h);
        let c5 = r1 == k4 && p5 >= 0 && p5 < (w*h);
        let c6 = r1 == k5 && p6 >= 0 && p6 < (w*h);

        if c1 { gp.push(p1); } // top-left
        if c2 { gp.push(p2); } // top-right
        if c3 { gp.push(p3); } // bot-left
        if c4 { gp.push(p4); } // bot-right
        if c5 { gp.push(p5); } // mid-left
        if c6 { gp.push(p6); } // mid-right

        gp.sort();
        gp
    }
        
    fn get_noise( &self, f: &dyn NoiseFn<[f64; 2]>, x: f32, y: f32) -> f32 {
        f.get([
            x as f64 * std::f64::consts::PI,
            y as f64 * std::f64::consts::PI,
        ]) as f32
    }

    fn get_simplex( &self, x: f32, y: f32) -> f32 {
        self.get_noise(&self.resources.simplex,x,y)
    }

    fn get_worley( &self, x: f32, y: f32) -> f32 {
        self.get_noise(&self.resources.worley,x,y)
    }

    fn get_value( &self, x: f32, y: f32) -> f32 {
        self.get_noise(&self.resources.value,x,y)
    }

    fn get_elevation( &self, x: i32, y: i32 ) -> Option<&f32> {
        self.values.elevation.get(&self.index(x,y))
    }

    fn get_temperature( &self, x: i32, y: i32 ) -> Option<&f32> {
        self.values.temperature.get(&self.index(x,y))
    }

    fn get_moisture( &self, x: i32, y: i32 ) -> Option<&u8> {
        self.values.moisture.get(&self.index(x,y))
    }

    fn get_rockiness( &self, x: i32, y: i32 ) -> Option<&u8> {
        self.values.rockiness.get(&self.index(x,y))
    }

    fn get_fertility( &self, x: i32, y: i32 ) -> Option<&u8> {
        self.values.fertility.get(&self.index(x,y))
    }

    fn get_biome( &self, x: i32, y: i32 ) -> Option<&Biome> {
        self.values.biome.get(&self.index(x,y))
    }

    fn get_soil( &self, x: i32, y: i32 ) -> Option<&Soil> {
        self.values.soil.get(&self.index(x,y))
    }

    fn get_foliage( &self, x: i32, y: i32 ) -> Option<&Foliage> {
        self.values.foliage.get(&self.index(x,y))
    }

    pub fn elevation( &mut self, x: i32, y: i32 ) -> f32 {
        match self.get_elevation(x,y) {
            Some(v) => *v,
            None => {
                let i = self.index(x,y);
                let v = self.make_elevation(x,y);
                self.values.elevation.insert(i,v);
                v
            }
        }
    }

    pub fn temperature( &mut self, x: i32, y: i32 ) -> f32 {
        match self.get_temperature(x,y) {
            Some(v) => *v,
            None => {
                let i = self.index(x,y);
                let v = self.make_temperature(x,y);
                self.values.temperature.insert(i,v);
                v
            }
        }
    }

    pub fn moisture( &mut self, x: i32, y: i32 ) -> u8 {
        match self.get_moisture(x,y) {
            Some(v) => *v,
            None => {
                let i = self.index(x,y);
                let v = self.make_moisture(x,y);
                self.values.moisture.insert(i,v);
                v
            }
        }
    }

    pub fn rockiness( &mut self, x: i32, y: i32 ) -> u8 {
        match self.get_rockiness(x,y) {
            Some(v) => *v,
            None => {
                let i = self.index(x,y);
                let v = self.make_rockiness(x,y);
                self.values.rockiness.insert(i,v);
                v
            }
        }
    }

    pub fn fertility( &mut self, x: i32, y: i32 ) -> u8 {
        match self.get_fertility(x,y) {
            Some(v) => *v,
            None => {
                let i = self.index(x,y);
                let v = self.make_fertility(x,y);
                self.values.fertility.insert(i,v);
                v
            }
        }
    }

    pub fn biome( &mut self, x: i32, y: i32 ) -> Biome {
        match self.get_biome(x,y) {
            Some(v) => *v,
            None => {
                let i = self.index(x,y);
                let v = self.make_biome(x,y);
                self.values.biome.insert(i,v);
                v
            }
        }
    }

    pub fn soil( &mut self, x: i32, y: i32 ) -> Soil {
        match self.get_soil(x,y) {
            Some(v) => *v,
            None => {
                let i = self.index(x,y);
                let v = self.make_soil(x,y);
                self.values.soil.insert(i,v);
                v
            }
        }
    }

    pub fn foliage( &mut self, x: i32, y: i32 ) -> Foliage {
        match self.get_foliage(x,y) {
            Some(v) => *v,
            None => {
                let i = self.index(x,y);
                let v = self.make_foliage(x,y);
                self.values.foliage.insert(i,v);
                v
            }
        }
    }

    pub fn textures( &mut self, textures: &Textures, x: i32, y: i32 ) -> Vec<usize> {
        let mut result = vec![
            textures.soil(&self.soil(x,y)),
        ];


        let m = self.moisture(x,y);
        let f = self.fertility(x,y);
        let t = self.foliage(x,y);
        let j = self.temperature(x,y);

        if m > 99 {
            result.push(textures.get("water"));
        }
        else {
            if j < 0. {
                result.push(textures.get("snow"));
            }
            else {
                if f > 75 {
                    result.push(textures.get("grass1"));
                }
                else if f > 50 {
                    result.push(textures.get("grass2"));
                }
                else if f > 25 {
                    result.push(textures.get("grass3"));
                }
                else {
                    result.push(textures.get("grass4"));
                }
            }

            if t == Foliage::Trees {
                result.push(textures.get("trees"));
            }
        }

        result
    }

    fn make_elevation( &self, x: i32, y: i32 ) -> f32 {
        let max = bounds::MAX_ELEV;
        let min = bounds::MIN_ELEV;

        let factor = self.factors.elevation as f32;

        let i = x as f32 * 0.015;
        let j = y as f32 * 0.015;

        let n1 = (self.get_simplex(i * 1.0, j * 1.0) + 1.0) / 2.0;
        let n2 = (self.get_simplex(i * 2.0, j * 2.0) + 1.0) / 2.0;
        let n3 = (self.get_simplex(i * 4.0, j * 4.0) + 1.0) / 2.0;

        let v1 = 1.00 * n1;
        let v2 = 0.50 * n2;
        let v3 = 0.25 * n3;

        let mut v;
        let mut f;

        f = factor / 100.0;

        // average multiple levels of noise
        v = (v1 + v2 + v3) / (1.0 + 0.5 + 0.25);

        // raise elev to factor and invert
        v = 1.0 - v.powf(f);

        // scale between min and max elevation
        v = (v * (max - min)) + min;

        v
    }

    fn make_temperature( &mut self, x: i32, y: i32 ) -> f32 {
        let emax = bounds::MAX_ELEV;
        let tmax = bounds::MAX_TEMP;
        let tmin = bounds::MIN_TEMP;

        let i = x as f32;
        let j = y as f32;

        // get seasonal weight and regional variation
        let w = 10.0;
        let k = ((self.get_value(i,j) + 1.) / 2.0) * -1.;

        // adjust 'k' based on the season and multiply by
        // the weight
        let s = match self.context.calendar.season() {
            Season::Winter => k * 1.0,
            Season::Autumn => k * 0.5,
            Season::Spring => k * 0.2,
            _ => 0.0,
        } * w;

        let mut f;
        let mut e;
        let mut v;

        f = self.factors.temperature as f32;
        e = self.elevation(x,y);

        // scale 0-1
        f = f / 100.;
        e = e / emax;

        // inverse elevation and add factor
        v = (1.0 - e)/2.0 + f/2.0;

        // scale into temperature range
        v * (tmax + tmin.abs()) - tmin.abs() + s
    }

    fn make_moisture( &mut self, x: i32, y: i32 ) -> u8 {
        let emax = bounds::MAX_ELEV;

        // adjust moisture based on the season
        let s = match self.context.calendar.season() {
            Season::Winter => 1,
            Season::Autumn => 2,
            Season::Spring => 4,
            _ => 0,
        } as u8;

        let mut f;
        let mut e;
        let mut v;

        f = self.factors.moisture as f32 / 50.;
        e = self.elevation(x,y);

        // normalize and invert elevation
        e = 1.0 - (e / emax);

        // scale double max X moisture, divide
        v = (f * e * 200.0) / 2.0;

        (v as u8 + s).min(100)
    }

    fn make_rockiness( &mut self, x: i32, y: i32 ) -> u8 {
        let emax = bounds::MAX_ELEV;

        let factor = self.factors.rockiness as f32;

        let e;
        let mut m;
        let mut v;

        // get related and normalize
        e = self.elevation(x,y) / emax;
        m = self.moisture(x,y) as f32;

        // scale and inverse moisture
        m = 1.0 - (m / 100.0);

        // scale and add factor
        v = (e * 100.0) + (e * factor);

        // account for moisture
        v = v * m;

        v as u8
    }

    fn make_fertility( &mut self, x: i32, y: i32 ) -> u8 {
        let tmax = bounds::MAX_TEMP;
        let tmin = bounds::MIN_TEMP;

        let factor = self.factors.fertility as f32;

        let mut t;
        let mut m;
        let mut r;
        let mut f;

        // get related and normalize
        t = self.temperature(x,y);
        r = self.rockiness(x,y) as f32;
        m = self.moisture(x,y) as f32;

        // ideal growing temperature is 20deg celsius
        let k = (20.0 + tmin.abs()) / (tmax + tmin.abs());

        // scale temperature
        t = (t + tmin.abs()) / (tmax + tmin.abs());

        // t is not distance from k
        t = (k - t).abs();

        // scale rockiness
        r = r / 100.0;

        // scale factor
        f = factor / 100.0;

        // scale and exaggerate moisture
        m = (m / 100.0).powf(2.0);

        (((f + m) - (r + t)) / 2.0 * 100.0).round() as u8
    }

    fn make_biome( &self, _x: i32, _y: i32 ) -> Biome {
        let factor = self.factors.biome;
        factor.into()
    }

    fn make_soil( &mut self, x: i32, y: i32 ) -> Soil {
        use Soil::*;

        if self.factors.soil != Soil::None {
            return self.factors.soil
        }

        // moisture on x axis, fertility on -y axis
        let spectrum = [
            Sand,   Sand, Chalk, Silt, Clay, Clay,
            Sand,  Chalk, Chalk, Silt, Clay, Clay,
            Chalk, Chalk,  Silt, Clay, Clay, Clay,
            Chalk,  Silt,  Silt, Loam, Peat, Peat,
            Silt,   Silt,  Loam, Loam, Peat, Peat,
            Silt,   Loam,  Loam, Loam, Peat, Peat,
        ];


        // biased because fertility is skewed low (70 rather than 100)
        let m = (self.moisture(x,y) as f32 / 100.).min(1.);
        let f = (self.fertility(x,y) as f32 / 100.).min(1.);

        let n = ((m * 5.) + (f * 30.)).min(35.).max(0.) as usize;

        spectrum[n]
    }

    fn make_foliage( &mut self, x: i32, y: i32 ) -> Foliage {
        let emax = bounds::MAX_ELEV;

        let i = x as f32 * 0.05;
        let j = y as f32 * 0.05;

        let v = self.get_simplex(i,j);
        let e = self.elevation(x,y) / emax;

        if v > 0. && e < 0.5 {
            Foliage::Trees
        }
        else {
            Foliage::Grass
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generator_indices_30x30() {
        let gen = Generator::new(0,30,30,Default::default(),0);
        assert_eq!(gen.index(-15,-15),0);
        assert_eq!(gen.index(0,-8),225);
        assert_eq!(gen.index(-11,-4),334);
        assert_eq!(gen.index(14,0),479);
        assert_eq!(gen.index(-15,4),570);
        assert_eq!(gen.index( 14, 14),899);
    }

    #[test]
    fn generator_indices_30x42() {
        let gen = Generator::new(0,30,42,Default::default(),0);
        assert_eq!(gen.index(-15,-21),0);
        assert_eq!(gen.index(14,-21),29);
        assert_eq!(gen.index(14,20),1259);
        assert_eq!(gen.index(-15,20),1230);
        assert_eq!(gen.index(10,16),1135);
        assert_eq!(gen.index(0,-6),465);
        assert_eq!(gen.index(8,-17),143);
    }

    #[test]
    fn generator_group_indices_30x30_corners() {
        let gen = Generator::new(0,30,30,Default::default(),0);
        let mut grp;

        // bottom-left corner
        grp = gen.index_group(-15,-15);
        assert_eq!(grp, vec![1,30,31]);

        // bottom-right corner
        grp = gen.index_group(14,-15);
        assert_eq!(grp, vec![28,59]);

        // top-left corner
        grp = gen.index_group(-15,14);
        assert_eq!(grp, vec![840,871]);

        // top-right corner
        grp = gen.index_group(14,14);
        assert_eq!(grp, vec![868,869,898]);
    }

    #[test]
    fn generator_group_indices_30x30_quadrants() {
        let gen = Generator::new(0,30,30,Default::default(),0);
        let mut grp;

        // top-left quadrant
        grp = gen.index_group(-10,10);
        assert_eq!(grp, vec![724,725,754,756,784,785]);

        // top-right quadrant
        grp = gen.index_group(10,10);
        assert_eq!(grp, vec![744,745,774,776,804,805]);

        // bottom-left quadrant
        grp = gen.index_group(-10,-10);
        assert_eq!(grp, vec![124,125,154,156,184,185]);

        // bottom-right quadrant
        grp = gen.index_group(10,-10);
        assert_eq!(grp, vec![144,145,174,176,204,205]);
    }

    #[test]
    fn generator_group_indices_30x30_edges() {
        let gen = Generator::new(0,30,30,Default::default(),0);
        let mut grp;

        // top-left quadrant top-edge
        grp = gen.index_group(-7,14);
        assert_eq!(grp, vec![847,848,877,879]);

        // top-left quadrant left edge
        grp = gen.index_group(-15,9);
        assert_eq!(grp, vec![690,691,721,750,751]);

        // bot-left quadrant bot-edge
        grp = gen.index_group(-9,-15);
        assert_eq!(grp, vec![5,7,36,37]);

        // bot-left quadrant left-edge
        grp = gen.index_group(-15,-10);
        assert_eq!(grp, vec![120,151,180]);

        // bot-right quadrant bot-edge
        grp = gen.index_group(8,-15);
        assert_eq!(grp, vec![22,24,53,54]);

        // bot-right quadrant right-edge
        grp = gen.index_group(14,-8);
        assert_eq!(grp, vec![208,209,238,268,269]);

        // top-right quadrant right-edge
        grp = gen.index_group(14,10);
        assert_eq!(grp, vec![748,749,778,808,809]);

        // top-right quadrant top-edge
        grp = gen.index_group(6,14);
        assert_eq!(grp, vec![860,861,890,892]);
    }

    #[test]
    fn generator_group_indices_30x30_negative() {
        let gen = Generator::new(0,30,30,Default::default(),0);
        let mut grp;

        // top-left quadrant top-edge
        grp = gen.index_group(-7,15);
        assert!(grp.is_empty());

        // top-left quadrant left edge
        grp = gen.index_group(-16,9);
        assert!(grp.is_empty());

        // bot-left quadrant bot-edge
        grp = gen.index_group(-9,-16);
        assert!(grp.is_empty());

        // bot-left quadrant left-edge
        grp = gen.index_group(-16,-10);
        assert!(grp.is_empty());

        // bot-right quadrant bot-edge
        grp = gen.index_group(8,-16);
        assert!(grp.is_empty());

        // bot-right quadrant right-edge
        grp = gen.index_group(15,-8);
        assert!(grp.is_empty());

        // top-right quadrant right-edge
        grp = gen.index_group(15,10);
        assert!(grp.is_empty());

        // top-right quadrant top-edge
        grp = gen.index_group(6,15);
        assert!(grp.is_empty());
    }
}