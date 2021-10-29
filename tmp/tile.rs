use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::ops::Add;

use macroquad::color::Color;
use macroquad::prelude::draw_rectangle_lines;

static ID: Lazy<Mutex<u64>> = Lazy::new(|| Mutex::new(0));

fn id() -> u64 {
    let v = ID.lock().unwrap().clone();
    ID.lock().unwrap().add(1);
    v
}

pub struct Tile {
    pub id: u64,
    pub r: f32,
    pub x: f32,
    pub y: f32,
}

impl Tile {

    pub fn new(x: f32, y: f32) -> Self {
        let id = id();
        let r = 1.0;
        Self { id, r, x, y }
    }

    pub fn all(w: u32, h: u32) -> Vec<Self> {
        let mut tiles = vec![];
        for y in 0..h {
            for x in 0..w {
                tiles.push(
                    Self::new(x as f32,y as f32));
            }
        }
        tiles
    }

    pub fn offset(&self, ox: f32, oy: f32, oz: f32) -> Self {
        let s = oz * 100.0;
        let r = s / 2.0;
        let x = (self.x * s) + ox;
        let y = (self.y * s) + oy;
        Self { id: self.id, r, x, y }
    }

    pub fn draw(&self, color: Color) {
        draw_rectangle_lines(
            self.x - self.r, 
            self.y - self.r, 
            self.r * 2.0, 
            self.r * 2.0, 
            self.r / 10.0,
            color
        )
    }

    pub fn overlaps(&self, x: f32, y: f32) -> bool {
        let lx = self.x - self.r;
        let gx = self.x + self.r;
        let ly = self.y - self.r;
        let gy = self.y + self.r;

        x > lx && x < gx && y > ly && y < gy
    }

    pub fn distance(&self, x: f32, y: f32) -> f32 {
        ((self.x - x).powf(2.0) + (self.y - y).powf(2.0)).sqrt()
    }

}