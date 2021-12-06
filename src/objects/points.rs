use std::fmt;

use std::marker::PhantomData;
use crate::state::Context;

/// https://gamedevelopment.tutsplus.com/tutorials/introduction-to-axial-coordinates-for-hexagonal-tile-based-games--cms-28820

/// Map tile orientation (warfare uses Horizontal)
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

/// Enum flag as an alternative means of checking
/// coordinate type
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum SystemType {
    Offset,
    Axial,
    Cubic,
}

/// Marker type for offset coordinates
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct Offset;

/// Marker type for axial coordinates
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct Axial;

/// Marker type for cubic coordinates
#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct Cubic;

type Index = i32;

/// Marker trait only implemented for Offset, 
/// Axial and Cubic coordinates
pub trait System {}

pub trait System2D {}
pub trait System3D {}

impl System for Offset {}
impl System for Axial {}
impl System for Cubic {}

impl System2D for Offset {}
impl System2D for Axial {}
impl System3D for Cubic {}

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct Point<T: System = Offset> {
    x: i32,
    y: i32,
    z: i32,
    phantom: PhantomData<T>,
}

impl<T: System + System2D> From<&Point<T>> for (i32,i32) {
    fn from(p: &Point<T>) -> Self { (p.x,p.y) }
}

impl<T: System + System2D> From<Point<T>> for (i32,i32) {
    fn from(p: Point<T>) -> Self { (&p).into() }
}

impl<T: System + System3D> From<&Point<T>> for (i32,i32,i32) {
    fn from(p: &Point<T>) -> Self { (p.x,p.y,p.z) }
}

impl<T: System + System3D> From<Point<T>> for (i32,i32,i32) {
    fn from(p: Point<T>) -> Self { (&p).into() }
}

impl<T: System + System2D> From<&Point<T>> for (f32,f32) {
    fn from(p: &Point<T>) -> Self { (p.x as f32, p.y as f32) }
}

impl<T: System + System2D> From<Point<T>> for (f32,f32) {
    fn from(p: Point<T>) -> Self { (&p).into() }
}

impl<T: System + System3D> From<&Point<T>> for (f32,f32,f32) {
    fn from(p: &Point<T>) -> Self { (p.x as f32, p.y as f32, p.z as f32) }
}

impl<T: System + System3D> From<Point<T>> for (f32,f32,f32) {
    fn from(p: Point<T>) -> Self { (&p).into() }
}

impl<T: System + System2D> From<(i32,i32)> for Point<T> {
    fn from((x,y): (i32,i32)) -> Self {
        Point::deduce(x,y,0)
    }
}

impl<T: System + System2D> From<(f32,f32)> for Point<T> {
    fn from((x,y): (f32,f32)) -> Self {
        Point::deduce(x as i32,y as i32, 0)
    }
}

impl<T: System + System3D> From<(i32,i32,i32)> for Point<T> {
    fn from((x,y,z): (i32,i32,i32)) -> Self {
        Point::deduce(x,y,z)
    }
}

impl<T: System + System3D> From<(f32,f32,f32)> for Point<T> {
    fn from((x,y,z): (f32,f32,f32)) -> Self {
        Point::deduce(x as i32,y as i32, z as i32)
    }
}

// &Offset => Axial
impl From<&Point<Offset>> for Point<Axial>{
    fn from(p: &Point<Offset>) -> Self {
        let (x,y): (f32,f32) = p.into();     // unpack values
        ( x, y - (x / 2.).floor()).into()    // convert to axial
    }
}

// Offset => Axial
impl From<Point<Offset>> for Point<Axial>{
    fn from(p: Point<Offset>) -> Self { (&p).into() }
}

// &Offset => Cubic
impl From<&Point<Offset>> for Point<Cubic>{
    fn from(p: &Point<Offset>) -> Self {
        Point::<Axial>::from(p).into()
    }
}

// Offset => Cubic
impl From<Point<Offset>> for Point<Cubic>{
    fn from(p: Point<Offset>) -> Self { (&p).into() }
}

// &Axial => Offset
impl From<&Point<Axial>> for Point<Offset>{
    fn from(p: &Point<Axial>) -> Self {
        let (x,y): (f32,f32) = p.into();     // unpack values
        ( x, y + (x / 2.).floor()).into()    // convert to offset
    }
}

// Axial => Offset
impl From<Point<Axial>> for Point<Offset>{
    fn from(p: Point<Axial>) -> Self { (&p).into() }
}

// &Axial => Cubic
impl From<&Point<Axial>> for Point<Cubic>{
    fn from(p: &Point<Axial>) -> Self {
        let (x,y): (f32,f32) = p.into();     // unpack values
        ( x, y, -x-y).into()                 // calculate `z`
    }
}

// Axial => Cubic
impl From<Point<Axial>> for Point<Cubic>{
    fn from(p: Point<Axial>) -> Self { (&p).into() }
}

// &Cubic => Offset
impl From<&Point<Cubic>> for Point<Offset>{
    fn from(p: &Point<Cubic>) -> Self {
        Point::<Axial>::from(p).into()
    }
}

// Cubic => Offset
impl From<Point<Cubic>> for Point<Offset>{
    fn from(p: Point<Cubic>) -> Self { (&p).into() }
}

// &Cubic => Axial
impl From<&Point<Cubic>> for Point<Axial>{
    fn from(p: &Point<Cubic>) -> Self {
        let (x,y,_): (f32,f32,_) = p.into(); // unpack values
        (x,y).into()                         // convert to axial
    }
}

// Cubic => Axial
impl From<Point<Cubic>> for Point<Axial>{
    fn from(p: Point<Cubic>) -> Self { (&p).into() }
}

impl Point<Offset> {
    /// Deduce the type for the new point
    fn deduce<T: System>(x: i32, y: i32, z: i32) -> Point<T> {
        Point { x, y, z, phantom: PhantomData }
    }

    /// Create a new offset, axial or cubic points from values/indices
    pub fn new(x: i32,y: i32) -> Point<Offset> { (x,y).into() }
    pub fn offset(x: i32,y: i32) -> Point<Offset> { (x,y).into() }
    pub fn axial(x: i32, y: i32) -> Point<Axial> { (x,y).into() }
    pub fn cubic(x: i32, y: i32, z:i32) -> Point<Cubic> { (x,y,z).into() }
    pub fn index(i: Index) -> Point<Offset> { Point::from_index(i) }

    pub fn subtype(&self) -> SystemType { SystemType::Offset }

    pub fn floats(&self) -> (f32,f32) { self.into() }
    pub fn integers(&self) -> (i32,i32) { self.into() }

    pub fn to_index(self) -> Index { self.as_index() }
    pub fn to_axial(self) -> Point<Axial> { self.into() }
    pub fn to_cubic(self) -> Point<Cubic> { self.into() }

    pub fn as_axial(&self) -> Point<Axial> { self.into() }
    pub fn as_cubic(&self) -> Point<Cubic> { self.into() }

    pub fn as_index(&self) -> Index { 
        let (w,h) = Context::size();
        let x = self.x + w / 2;
        let y = self.y + h / 2;
        x + y * w
    }

    pub fn from_index(i: Index) -> Point<Offset> {
        let (w,h) = Context::size();
        ((i % w) - (w / 2),
         (i / w) - (h / 2)).into()
    }

    pub fn neighbors(&self) -> Vec<Point<Offset>> {
        let (w,h) = Context::size();

        let x = self.x;
        let y = self.y;
        
        let mut result = vec![];
        
        // early return if given point is out of bounds
        if x >= (w/2) || 
           x < -(w/2) || 
           y >= (h/2) || 
           y < -(h/2) 
        {
            return result;
        }

        // get indices from (x,y) coordinates
        let p0 = Point::offset(x,y).to_index();         // center    
        let mut p1 = Point::offset(x-1,y+1).to_index(); // top-left  
        let mut p2 = Point::offset(x,y+1).to_index();   // top-right 
        let mut p3 = Point::offset(x-1,y-1).to_index(); // bot-left  
        let mut p4 = Point::offset(x,y-1).to_index();   // bot-right 
        let p5 = Point::offset(x-1,y).to_index();       // mid-left  
        let p6 = Point::offset(x+1,y).to_index();       // mid-right 

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

        if c1 { result.push(Point::index(p1)); } // top-left
        if c2 { result.push(Point::index(p2)); } // top-right
        if c3 { result.push(Point::index(p3)); } // bot-left
        if c4 { result.push(Point::index(p4)); } // bot-right
        if c5 { result.push(Point::index(p5)); } // mid-left
        if c6 { result.push(Point::index(p6)); } // mid-right

        result
    }

    pub fn distance<T: Into<Point<Offset>>>(&self, other: T) -> i32 {
        self.as_cubic().distance(other.into())
    }
}

impl Point<Axial> {
    pub fn subtype(&self) -> SystemType { SystemType::Axial }

    pub fn floats(&self) -> (f32,f32) { self.into() }
    pub fn integers(&self) -> (i32,i32) { self.into() }

    pub fn to_index(self) -> Index { self.to_offset().as_index() }
    pub fn to_offset(self) -> Point<Offset> { self.into() }
    pub fn to_cubic(self) -> Point<Cubic> { self.into() }

    pub fn as_index(&self) -> Index { self.as_offset().as_index() }
    pub fn as_offset(&self) -> Point<Offset> { self.into() }
    pub fn as_cubic(&self) -> Point<Cubic> { self.into() }

    pub fn neighbors(&self) -> Vec<Point<Axial>> {
        self.as_offset()
            .neighbors()
            .into_iter()
            .map(|v| Self::from(v))
            .collect()
    }

    pub fn distance<T: Into<Point<Axial>>>(&self, other: T) -> i32 {
        self.as_cubic().distance(other.into())
    }
}

impl Point<Cubic> {
    pub fn subtype(&self) -> SystemType { SystemType::Cubic }

    pub fn floats(&self) -> (f32,f32,f32) { self.into() }
    pub fn integers(&self) -> (i32,i32,i32) { self.into() }

    pub fn to_index(self) -> Index { self.to_offset().as_index() }
    pub fn to_offset(self) -> Point<Offset> { self.into() }
    pub fn to_axial(self) -> Point<Axial> { self.into() }

    pub fn as_index(&self) -> Index { self.as_offset().as_index() }
    pub fn as_offset(&self) -> Point<Offset> { self.into() }
    pub fn as_axial(&self) -> Point<Axial> { self.into() }

    // pub fn neighbors(&self) -> Vec<Point<Cubic>> {
    //     self.as_offset()
    //         .neighbors()
    //         .into_iter()
    //         .map(|v| Self::from(v))
    //         .collect()
    // }

    /*
        re-implement neighbors:
            1. calculate max and min points in row
            2. convert max and min points to cubic
            3. verify that neighboring points are inside bounds
            3. do not convert to index to calculate
    */

    pub fn distance<T: Into<Point<Cubic>>>(&self, other: T) -> i32 {
        let (x1,y1,z1) = self.integers();
        let (x2,y2,z2) = other.into().integers();
        ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) / 2
    }

}

/*
    IMPLEMENT Eq/PartialEq for offset/axial/cubic

*/

// impl<T: Into<Point<Cubic>>> PartialEq<T> for Point<Cubic> {
//     fn eq(&self, other: &T) -> bool {
//         let p = other.into();
//         self.x == p.x && self.y == p.y && self.z == p.z
//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! initialize {
        ( $w:expr, $h:expr ) => { 
            crate::state::State::default().set_map_size($w,$h) 
        }
    }

    macro_rules! index {
        ( $i:expr ) => { Point::index($i) }
    }
    
    macro_rules! point {
        ( $x:expr, $y:expr ) => { Point::offset($x,$y) }
    }

    macro_rules! axial {
        ( $x:expr, $y:expr ) => { Point::offset($x,$y).to_axial() }
    }

    macro_rules! cubic {
        ( $x:expr, $y:expr ) => { Point::offset($x,$y).to_cubic() }
    }
    
    macro_rules! points {
        ($( ($x:expr,$y:expr) ),*) => {
            vec![$( 
                point!($x, $y) 
            ),*]
        }
    }

    // #[test]
    // fn neighbors_offset_compare_cubic_30x30() {
    //     initialize!(30,30);

    //     assert_eq!(
    //         point!(0,0).neighbors(),
    //         cubic!(0,0).neighbors())
 
    // }

    #[test]
    fn offset_point_as_index_30x30() {
        initialize!(30,30);
        assert_eq!(point!(-15,-15).as_index(), 0);
        assert_eq!(point!(0,-8).as_index(), 225);
        assert_eq!(point!(-11,-4).as_index(), 334);
        assert_eq!(point!(14,0).as_index(), 479);
        assert_eq!(point!(-15,4).as_index(), 570);
        assert_eq!(point!( 14, 14).as_index(), 899);
    }

    #[test]
    fn axial_point_as_index_30x30() {
        initialize!(30,30);
        assert_eq!(axial!(-15,-15).as_index(), 0);
        assert_eq!(axial!(0,-8).as_index(), 225);
        assert_eq!(axial!(-11,-4).as_index(), 334);
        assert_eq!(axial!(14,0).as_index(), 479);
        assert_eq!(axial!(-15,4).as_index(), 570);
        assert_eq!(axial!( 14, 14).as_index(), 899);
    }

    #[test]
    fn cubic_point_as_index_30x30() {
        initialize!(30,30);
        assert_eq!(cubic!(-15,-15).as_index(), 0);
        assert_eq!(cubic!(0,-8).as_index(), 225);
        assert_eq!(cubic!(-11,-4).as_index(), 334);
        assert_eq!(cubic!(14,0).as_index(), 479);
        assert_eq!(cubic!(-15,4).as_index(), 570);
        assert_eq!(cubic!( 14, 14).as_index(), 899);
    }

    #[test]
    fn point_as_index_30x42() {
        initialize!(30,42);
        assert_eq!(point!(-15,-21).as_index(),0);
        assert_eq!(point!(14,-21).as_index(),29);
        assert_eq!(point!(14,20).as_index(),1259);
        assert_eq!(point!(-15,20).as_index(),1230);
        assert_eq!(point!(10,16).as_index(),1135);
        assert_eq!(point!(0,-6).as_index(),465);
        assert_eq!(point!(8,-17).as_index(),143);
    }

    #[test]
    fn index_to_point_30x30() {
        initialize!(30,30);
        assert_eq!(index!(0),point!(-15,-15));
        assert_eq!(index!(225),point!(0,-8));
        assert_eq!(index!(334),point!(-11,-4));
        assert_eq!(index!(479),point!(14,0));
        assert_eq!(index!(570),point!(-15,4));
        assert_eq!(index!(899),point!(14,14));
    }

    #[test]
    fn index_to_point_30x42() {
        initialize!(30,42);
        assert_eq!(index!(0),point!(-15,-21));
        assert_eq!(index!(29),point!(14,-21));
        assert_eq!(index!(1259),point!(14,20));
        assert_eq!(index!(1230),point!(-15,20));
        assert_eq!(index!(1135),point!(10,16));
        assert_eq!(index!(465),point!(0,-6));
        assert_eq!(index!(143),point!(8,-17));
    }

    #[test]
    fn point_neighbors_30x30_corners() {
        initialize!(30,30);

        // bottom-left corner
        assert_eq!(
            point!(-15,-15).neighbors(), 
            points![(-15,-14),(-14,-14),(-14,-15)],
        );

        // bottom-right corner
        assert_eq!(
            point!(14,-15).neighbors(), 
            points![(14,-14),(13,-15)],
        );

        // top-left corner
        assert_eq!(
            point!(-15,14).neighbors(), 
            points![(-15,13),(-14,14)],
        );

        // top-right corner
        assert_eq!(
            point!(14,14).neighbors(), 
            points![(13,13),(14,13),(13,14)],
        );
    }

    #[test]
    fn point_neighbors_30x30_quadrants() {
        initialize!(30,30);

        // top-left quadrant
        assert_eq!(
            point!(-10,10).neighbors(),
            points![(-11,11),(-10,11),(-11,9),(-10,9),(-11,10),(-9,10)],
        );

        // top-right quadrant
        assert_eq!(
            point!(10,10).neighbors(),
            points![(9,11),(10,11),(9,9),(10,9),(9,10),(11,10)],
        );

        // bottom-left quadrant
        assert_eq!(
            point!(-10,-10).neighbors(),
            points![(-11,-9),(-10,-9),(-11,-11),(-10,-11),(-11,-10),(-9,-10)],
        );

        // bottom-right quadrant
        assert_eq!(
            point!(10,-10).neighbors(),
            points![(9,-9),(10,-9),(9,-11),(10,-11),(9,-10),(11,-10)],
        );
    }

    #[test]
    fn point_neighbors_30x30_edges() {
        initialize!(30,30);

        // top-left quadrant top-edge
        assert_eq!(
            point!(-7,14).neighbors(),
            points![(-8,13),(-7,13),(-8,14),(-6,14)],
        );

        // top-left quadrant left edge
        assert_eq!(
            point!(-15,9).neighbors(),
            points![(-15,10), (-14,10), (-15,8), (-14,8), (-14,9)],
        );

        // bot-left quadrant bot-edge
        assert_eq!(
            point!(-9,-15).neighbors(),
            points![(-9,-14), (-8,-14), (-10,-15), (-8,-15)],
        );

        // bot-left quadrant left-edge
        assert_eq!(
            point!(-15,-10).neighbors(),
            points![(-15,-9), (-15,-11), (-14,-10)],
        );

        // bot-right quadrant bot-edge
        assert_eq!(
            point!(8,-15).neighbors(),
            points![(8,-14), (9,-14), (7,-15), (9,-15)],
        );

        // bot-right quadrant right-edge
        assert_eq!(
            point!(14,-8).neighbors(),
            points![(13,-7), (14,-7), (13,-9), (14,-9), (13,-8)],
        );


        // top-right quadrant right-edge
        assert_eq!(
            point!(14,10).neighbors(),
            points![(13,11), (14,11), (13,9), (14,9), (13,10)],
        );

        // top-right quadrant top-edge
        assert_eq!(
            point!(6,14).neighbors(),
            points![(5,13), (6,13), (5,14), (7,14)],
        );

    }

    #[test]
    fn point_neighbors_30x30_negative() {
        initialize!(30,30);

        // top-left quadrant top-edge
        assert!(point!(-7,15).neighbors().is_empty());

        // top-left quadrant left edge
        assert!(point!(-16,9).neighbors().is_empty());

        // bot-left quadrant bot-edge
        assert!(point!(-9,-16).neighbors().is_empty());

        // bot-left quadrant left-edge
        assert!(point!(-16,-10).neighbors().is_empty());

        // bot-right quadrant bot-edge
        assert!(point!(8,-16).neighbors().is_empty());

        // bot-right quadrant right-edge
        assert!(point!(15,-8).neighbors().is_empty());

        // top-right quadrant right-edge
        assert!(point!(15,10).neighbors().is_empty());

        // top-right quadrant top-edge
        assert!(point!(6,15).neighbors().is_empty());
    }
}