use bevy::prelude::Color;

#[derive(Default, Debug, Clone)]
pub struct Shade {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
}

#[derive(Default, Debug, Clone)]
pub struct Spectrum {
    start: Shade,
    end: Shade,
}

impl Spectrum {
    pub fn empty() -> Self {
        Self {
            start: Shade {
                h: 0.0,
                s: 1.0,
                l: 1.0,
                a: 0.0,
            },
            end: Shade {
                h: 0.0,
                s: 1.0,
                l: 1.0,
                a: 0.0,
            },
        }
    }

    pub fn with_start_color(mut self, h: f32, s: f32, l: f32, a: f32) -> Self {
        self.start = Shade { h, s, l, a };
        self
    }

    pub fn with_end_color(mut self, h: f32, s: f32, l: f32, a: f32) -> Self {
        self.end = Shade { h, s, l, a };
        self
    }

    pub fn finish(self) -> Self {
        self
    }

    pub fn get(&self, value: f32) -> Color {
        let h = self.interp_hue(&self.start, &self.end, value);
        let s = self.interpolate(self.start.s, self.end.s, value);
        let l = self.interpolate(self.start.l, self.end.l, value);
        let a = self.interpolate(self.start.a, self.end.a, value);
        let (r, g, b) = self.convert_color(h, s, l);
        Color::rgba(r, g, b, a)
    }

    fn interp_hue(&self, start: &Shade, end: &Shade, mut v: f32) -> f32 {
        let mut a = start.clone();
        let mut b = end.clone();

        // Hue interpolation
        let h;
        let mut d = b.h - a.h;

        if a.h > b.h {
            // swap b.h and a.h
            let k = b.h;
            b.h = a.h;
            a.h = k;

            d = -d;
            v = 1. - v;
        }

        if d > 0.5 {
            a.h = a.h + 1.;
            h = (a.h + v * (b.h - a.h)) % 1.;
        } else {
            h = a.h + v * d;
        }

        h
    }

    fn interpolate(&self, start: f32, end: f32, v: f32) -> f32 {
        start * (1.0 - v) + end * v
    }

    fn convert_color(&self, h: f32, s: f32, l: f32) -> (f32, f32, f32) {
        let r;
        let g;
        let b;

        if s == 0.0 {
            r = l;
            g = l;
            b = l;
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;
            r = self.convert_value(p, q, h + 1.0 / 3.0);
            g = self.convert_value(p, q, h);
            b = self.convert_value(p, q, h - 1.0 / 3.0);
        }

        (r, g, b)
    }

    fn convert_value(&self, p: f32, q: f32, mut t: f32) -> f32 {
        if t < 0.0 {
            t += 1.0
        };
        if t > 1.0 {
            t -= 1.0
        };
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        };
        if t < 1.0 / 2.0 {
            return q;
        };
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        };
        p
    }
}

/**
 * Converts an HSL color value to RGB. Conversion formula
 * adapted from http://en.wikipedia.org/wiki/HSL_color_space.
 * Assumes h, s, and l are contained in the set [0, 1] and
 * returns r, g, and b in the set [0, 255].
 *
 * @param   {number}  h       The hue
 * @param   {number}  s       The saturation
 * @param   {number}  l       The lightness
 * @return  {Array}           The RGB representation
 */
//  function hslToRgb(h, s, l){
//     var r, g, b;

//     if(s == 0){
//         r = g = b = l; // achromatic
//     }else{
//         var hue2rgb = function hue2rgb(p, q, t){
//             if(t < 0) t += 1;
//             if(t > 1) t -= 1;
//             if(t < 1/6) return p + (q - p) * 6 * t;
//             if(t < 1/2) return q;
//             if(t < 2/3) return p + (q - p) * (2/3 - t) * 6;
//             return p;
//         }

//         var q = l < 0.5 ? l * (1 + s) : l + s - l * s;
//         var p = 2 * l - q;
//         r = hue2rgb(p, q, h + 1/3);
//         g = hue2rgb(p, q, h);
//         b = hue2rgb(p, q, h - 1/3);
//     }

//     return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)];
// }

/**
 * Converts an RGB color value to HSL. Conversion formula
 * adapted from http://en.wikipedia.org/wiki/HSL_color_space.
 * Assumes r, g, and b are contained in the set [0, 255] and
 * returns h, s, and l in the set [0, 1].
 *
 * @param   {number}  r       The red color value
 * @param   {number}  g       The green color value
 * @param   {number}  b       The blue color value
 * @return  {Array}           The HSL representation
 */
//  function rgbToHsl(r, g, b){
//     r /= 255, g /= 255, b /= 255;
//     var max = Math.max(r, g, b), min = Math.min(r, g, b);
//     var h, s, l = (max + min) / 2;

//     if(max == min){
//         h = s = 0; // achromatic
//     }else{
//         var d = max - min;
//         s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
//         switch(max){
//             case r: h = (g - b) / d + (g < b ? 6 : 0); break;
//             case g: h = (b - r) / d + 2; break;
//             case b: h = (r - g) / d + 4; break;
//         }
//         h /= 6;
//     }

//     return [h, s, l];
// }

impl From<Color> for Shade {
    fn from(color: Color) -> Self {
        match color.as_hsla() {
            Color::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } => Self {
                h: hue,
                s: saturation,
                l: lightness,
                a: alpha,
            },
            _ => unreachable!(),
        }
    }
}
