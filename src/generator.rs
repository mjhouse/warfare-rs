use noise::{Perlin,NoiseFn,SuperSimplex,Seedable};
use rand::{Rng,rngs::ThreadRng};
use std::collections::HashMap;

/*
    1. Make generator struct
    2. Each method ('temperature', 'moisture' etc) calculates (or returns a cached value) for a SINGLE index
    3. Each method may also use other values, which means an external call will calculate multiple values
    4. To determine if a value has been calculated or not, use HashSet<usize> to check if index exists
       for each type. For example:
            pub moisture_record: HashSet<usize>,
            pub temperature_record: HashSet<usize>,
            ...

*/

struct Context {
    seed: u32,
    width: i32,
    height: i32,
}

struct Resources {
    noise: Box<NoiseFn<[f64; 2]>>,
    random: ThreadRng,
}

struct Values {
    elevation: HashMap<i32,f32>,
    temperature: HashMap<i32,f32>,
    moisture: HashMap<i32,u8>,
}

struct Generator {
    context: Context,
    resources: Resources,
    values: Values,
}

impl Generator {

    pub fn new( seed: u32, width: i32, height: i32 ) -> Self {
        Self {
            context: Context {
                seed: seed,
                width: width,
                height: height,
            },
            resources: Resources {
                noise: Box::new(SuperSimplex::new().set_seed(seed)),
                random: rand::thread_rng(),
            },
            values: Values {
                elevation: HashMap::new(),
                temperature: HashMap::new(),
                moisture: HashMap::new(),
            }
        }
    }

    fn index( &self, mut x: i32, mut y: i32 ) -> i32 {
        x = x + self.context.width / 2;
        y = y + self.context.height / 2;
        (y * self.context.width) + x
    }

    fn index_group( &self, x: i32, y: i32 ) -> Vec<i32> {
        let mut p0 = self.index(x    , y    ); // center
        let mut p1 = self.index(x    , y + 1); // top-left
        let mut p2 = self.index(x + 1, y + 1); // top-right
        let mut p3 = self.index(x + 1, y    ); // mid-right
        let mut p4 = self.index(x + 1, y - 1); // bot-right
        let mut p5 = self.index(x    , y - 1); // bot-left
        let mut p6 = self.index(x - 1, y    ); // mid-left

        let w = self.context.width;
        let h = self.context.height;

        // expected rows
        let r0 = p0 / w + 1;
        let r1 = p0 / w;
        let r2 = p0 / w - 1;

        // account for offset due to hex tiling,
        // MAY NOT BE NECESSARY
        if r1 % 2 != 0 {
            p1 -= 1;
            p2 -= 1;
            p4 -= 1;
            p5 -= 1;
        }

        // actual rows
        let k0 = p1 / w;
        let k1 = p2 / w;
        let k2 = p3 / w;
        let k3 = p4 / w;
        let k4 = p5 / w;
        let k5 = p6 / w;

        let mut group = vec![];

        // only include indices for points in the 
        // expected rows
        if r0 == k0 { group.push(p1); }
        if r0 == k1 { group.push(p2); }
        if r1 == k2 { group.push(p3); }
        if r2 == k3 { group.push(p4); }
        if r2 == k4 { group.push(p5); }
        if r1 == k5 { group.push(p6); }

        group
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

    pub fn elevation( &mut self, x: i32, y: i32 ) -> f32 {
        match self.get_elevation(x,y) {
            Some(v) => *v,
            None => self.make_elevation(x,y),
        }
    }

    pub fn temperature( &mut self, x: i32, y: i32 ) -> f32 {
        match self.get_temperature(x,y) {
            Some(v) => *v,
            None => self.make_temperature(x,y),
        }
    }

    pub fn moisture( &mut self, x: i32, y: i32 ) -> u8 {
        match self.get_moisture(x,y) {
            Some(v) => *v,
            None => self.make_moisture(x,y),
        }
    }

    fn make_elevation( &mut self, x: i32, y: i32 ) -> f32 {
        0.0
    }

    fn make_temperature( &mut self, x: i32, y: i32 ) -> f32 {
        0.0
    }

    fn make_moisture( &mut self, x: i32, y: i32 ) -> u8 {
        0
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn generator_indices_calculated_correctly() {
        // write test to determine whether Generator::index is 
        // finding the correct index given (x,y) points
    }

    #[test]
    fn generator_group_calculated_correctly() {
        // write test to determine whether Generator::index_group is 
        // finding the correct local indices given an (x,y) point
    }
}