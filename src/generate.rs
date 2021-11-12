use std::collections::HashMap;
use std::iter::FromIterator;
use std::array::IntoIter;

use bevy::prelude::*;
use bevy::{
    asset::LoadState,
    sprite::{TextureAtlas, TextureAtlasBuilder},
    window::WindowMode,
};


use bevy_tilemap::prelude::*;

use noise::{Perlin,NoiseFn,Clamp,Seedable};
use rand::Rng;

use crate::area::{Area,Soil,Biome,bounds};
use crate::controls::Controls;
use crate::state::State;

pub struct GeneratorPlugin;

#[derive(Default,Debug,Clone)]
pub struct Generator {
    // add map features
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

fn generate_temperature(width: i32, height: i32, seed: u32, factor: f32) -> Vec<f32> {
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

    let mut temperature = vec![];

    let max = bounds::MAX_TEMP;
    let min = bounds::MIN_TEMP;

    for y in 0..height {
        for x in 0..width {
            let mut v = noise.get([
                x as f64 * std::f64::consts::PI,
                y as f64 * std::f64::consts::PI,
            ]) as f32;

            // normalize between 0 and 1
            v = 1.0 - (v / 4.0);

            // scale between min and max
            v = (v * (max + min.abs())) - min.abs();

            // scale by given factor and add
            temperature.push(v * factor);
        }
    }

    temperature
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

mod gen {

    use crate::area::bounds;

    #[derive(Default,Debug,Clone)]
    pub struct Context {
        pub seed: u32,
        pub width: i32,
        pub height: i32,
        pub elevation: f32,
        pub moisture: f32,
        pub temperature: f32,
    }

    pub fn elevation(context: &Context, x: f32, y: f32, initial: f32) -> f32 {
        0.0
    }

    pub fn temperature(context: &Context, x: f32, y: f32, initial: f32) -> f32 {
        // use rand::Rng;
        // let mut rng = rand::thread_rng();

        let mut max_e = bounds::MAX_ELEV;
        let mut min_e = bounds::MIN_ELEV;
        let mut mid_e = max_e - min_e;

        let mut max_t = bounds::MAX_TEMP;
        let mut min_t = bounds::MIN_TEMP;
        let mut mid_t = max_t - min_t;
        
        let mut e = context.elevation;
        let mut w = context.moisture;
        
        let mut t = initial;
        let mut v = 0.0;

        // shift range to be positive
        if min_e < 0.0 { 
            let m = min_e.abs();
            max_e += m;
            min_e += m;
            mid_e = max_e - min_e;
            e += m;
        }

        // shift range to be positive
        if min_t < 0.0 { 
            let m = min_t.abs();
            max_t += m;
            min_t += m;
            mid_t = max_t - min_t;
        }

        // normalize elevation (0.0,1.0)
        v = (e - min_e) / (max_e - min_e);

        // invert elevation
        v = 1.0 - v;

        // scale temperature by elevation
        t = t + (t * (v - 0.5)) * 2.0;

        // scale temperature by water
        t = t + ((-t * 0.25) * w);

        // add random variation (NOT TESTED)
        //t = t + (t * 0.25 * (rng.gen::<f32>() - 0.5) ); // rng = [-.5,.5] 

        t
    }

    pub fn moisture(context: &Context, x: f32, y: f32, initial: f32) -> f32 {
        0.0
    }
}

fn generate(icons: HashMap<Soil,usize>, width: i32, height: i32, seed:u32, efactor: f32, wfactor: u8) -> Vec<Area> {
    let mut results = vec![];
    
    let elevations = generate_elevation(width,height,seed,efactor);
    let moistures = generate_moisture(width,height,wfactor,&elevations);
    // let temperatures = generate_temperature(width,height,seed,efactor);

    let mut c = 0;

    for y in 0..height {
        for x in 0..width {
            let y = y - height / 2;
            let x = x - width / 2;

            println!("({},{}): {}",x,y,c);

            // ========================================================
            // TEST
            let mut context = gen::Context::default();
            context.elevation = elevations[c];
            context.width = width;
            context.height = height;
            context.seed = seed;
            context.moisture = moistures[c] as f32 / 100.0;

            let fx = x as f32;
            let fy = y as f32;

            let initial = 20.0; // room temp in c

            context.temperature = gen::temperature(&context,fx,fy,initial);
            // ========================================================

            let location = (x,y);
            let biome = Biome::Grassland;
            let soil = Soil::Loam;
            let moisture = moistures[c];
            let rocks = 0;
            let fertility = 0;
            let elevation = elevations[c];
            let temperature = context.temperature;
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

fn generator_initialize_system(
    mut commands: Commands,
    mut resources: ResMut<crate::WarfareResources>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if !resources.loaded_textures {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();

        if let LoadState::Loaded =
            asset_server.get_group_load_state(resources.textures.iter().map(|h| h.id))
        {
            for handle in resources.textures.iter() {
                let texture = textures.get(handle).unwrap();
                texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
            }
    
            let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
            let atlas_handle = texture_atlases.add(texture_atlas);
    
            let tilemap = Tilemap::builder()
                .topology(GridTopology::HexOddRows)
                .dimensions(1, 1)
                .chunk_dimensions(30, 30, 1)
                .texture_dimensions(175, 200)
                .add_layer(
                    TilemapLayer {
                        kind: LayerKind::Dense,
                        ..Default::default()
                    },
                    0,
                )
                .add_layer(
                    TilemapLayer {
                        kind: LayerKind::Sparse,
                        ..Default::default()
                    },
                    1,
                )
                .texture_atlas(atlas_handle)
                .finish()
                .unwrap();
    
            let tilemap_components = TilemapBundle {
                tilemap,
                visible: Visible {
                    is_visible: true,
                    is_transparent: true,
                },
                transform: Default::default(),
                global_transform: Default::default(),
            };
    
            commands
                .spawn()
                .insert_bundle(tilemap_components)
                .insert(Timer::from_seconds(0.075, true));
    
            resources.loaded_textures = true;
        }
    }
}

fn generator_configure_system(
    mut state: ResMut<State>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut map_query: Query<&mut Tilemap>,
    mut ctl_query: Query<&mut Controls>,
) {
    if state.terrain.update {
        if let Ok(mut map) = map_query.single_mut() {

            let seed = state.terrain.seed.parse::<u32>().unwrap_or(0);
            let efactor = state.terrain.mountains;
            let wfactor = state.terrain.water;

            if !map.contains_chunk((0, 0)) {
                map.insert_chunk((0, 0)).unwrap();
            }

            let width = (map.width().unwrap() * map.chunk_width()) as i32;
            let height = (map.height().unwrap() * map.chunk_height()) as i32;

            // Then we need to find out what the handles were to our textures we are going to use.
            let clay: Handle<Texture> = asset_server.get_handle("textures/clay.png");
            let sand: Handle<Texture> = asset_server.get_handle("textures/sand.png");
            let silt: Handle<Texture> = asset_server.get_handle("textures/silt.png");
            let peat: Handle<Texture> = asset_server.get_handle("textures/peat.png");
            let chalk: Handle<Texture> = asset_server.get_handle("textures/chalk.png");
            let loam: Handle<Texture> = asset_server.get_handle("textures/loam.png");
            let blank: Handle<Texture> = asset_server.get_handle("textures/blank.png");
            let marker: Handle<Texture> = asset_server.get_handle("textures/marker.png");

            let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();

            let icons = HashMap::<_, _>::from_iter(IntoIter::new([
                (Soil::Clay, texture_atlas.get_texture_index(&clay).unwrap()),
                (Soil::Sand, texture_atlas.get_texture_index(&sand).unwrap()),
                (Soil::Silt, texture_atlas.get_texture_index(&silt).unwrap()),
                (Soil::Peat, texture_atlas.get_texture_index(&peat).unwrap()),
                (Soil::Chalk, texture_atlas.get_texture_index(&chalk).unwrap()),
                (Soil::Loam, texture_atlas.get_texture_index(&loam).unwrap()), 
            ]));

            let areas = generate(icons,width,height,seed,efactor,wfactor);

            let mut tiles = areas
                .iter()
                .map(|a| a.tile())
                .collect::<Vec<Tile<_>>>();

            state.icons.blank = texture_atlas.get_texture_index(&blank).unwrap();
            state.icons.mark = texture_atlas.get_texture_index(&marker).unwrap();

            state.add_all(areas);
            map.insert_tiles(tiles).unwrap();

            map.spawn_chunk((0, 0)).unwrap();

            state.terrain.update = false;
            state.loaded = true;

            // update overlay
            if let Ok(mut c) = ctl_query.single_mut() {
                c.update = true;
            }
        }
    }
}

impl Plugin for GeneratorPlugin {
	fn build(&self, app: &mut AppBuilder) {
        app
            .add_system(generator_initialize_system.system())
            .add_system(generator_configure_system.system());
	}
}

pub fn setup(mut commands: Commands) {
	commands
		.spawn()
		.insert(Generator::default());
}