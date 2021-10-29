use macroquad::input::MouseButton;
use macroquad::input::is_mouse_button_down;
use macroquad::input::mouse_wheel;
use macroquad::input::mouse_position;


use macroquad::text::draw_text;
use macroquad::color::colors::{WHITE,BLUE,RED};
use macroquad::prelude::*;

use crate::tile::Tile;

const MIN_ZOOM: f32 = 0.0;
const MAX_ZOOM: f32 = 1.0;

macro_rules! feq {
    ( $a: expr, $b: expr ) => {
        ($a - $b).abs() / ($a + $b) < 0.00001
    }
}

pub struct View {
    px: Option<f32>, // previous x
    py: Option<f32>, // previous y
    ox: f32, // offset x
    oy: f32, // offset y
    oz: f32, // offset z
}

impl View {

    pub fn new() -> Self {
        Self {
            px: None,
            py: None,
            ox: 0.0,
            oy: 0.0,
            oz: 0.5,
        }
    }

    pub fn update(&mut self) {
        self.zoom();
        self.pan();
    }

    // TODO: change this to use a 'Displayable' trait or something
    pub fn display(&self, pos: &Tile, tex: &Texture2D) {
        let p = pos.offset(self.ox,self.oy,self.oz);

        draw_texture(
            *tex,
            p.x - p.r,
            p.y - p.r,
            WHITE
        );

        let (mx,my) = mouse_position();
        if p.overlaps(mx,my) {
            p.draw(BLUE);
        }
        else {
            p.draw(RED);
        }
    }

    fn zoom(&mut self) {
        let ( _,wy) = mouse_wheel();
        let v = self.oz - wy / 100.0;
        let r = (v * 100.0).round() / 100.0;
        if r >= MIN_ZOOM && r <= MAX_ZOOM {
            self.oz = r;
        }
    }

    fn pan(&mut self) {
        let (mx,my) = mouse_position();
        if self.capturing() {
            let px = self.px.unwrap_or(mx);
            let py = self.py.unwrap_or(my);

            self.ox += mx - px;
            self.oy += my - py;

            self.px = Some(mx);
            self.py = Some(my);
        }
        else {
            self.px = None;
            self.py = None;
        }
    }

    fn capturing(&self) -> bool {
        // TODO: change this to use a configured button
        is_mouse_button_down(MouseButton::Right)
    }
    

    pub fn debug(&self) {
        let msg = format!("view: (ox: {}, oy: {}, oz: {:.2})",self.ox,self.oy,self.oz);
        draw_text(msg.as_ref(), 20.0, 20.0, 20.0, WHITE);
    }

    pub fn message(&self, msg: &str) {
        draw_text(msg, 20.0, 40.0, 20.0, WHITE);
    }

}