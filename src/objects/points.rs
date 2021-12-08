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

#[derive(Copy,Clone,PartialEq,Eq,Hash)]
pub struct Point<T: System = Offset> {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    phantom: PhantomData<T>,
}

impl fmt::Debug for Point<Offset> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({},{})", self.x,self.y)
    }
}

impl fmt::Debug for Point<Axial> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({},{})", self.x,self.y)
    }
}

impl fmt::Debug for Point<Cubic> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({},{},{})", self.x,self.y,self.z)
    }
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
        let (x,y): (f32,f32) = p.into();   // unpack values
        let q = x - (y - (y as i32 & 1) as f32) / 2.;
        let r = y;
        (q,r).into()
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
        let (q,r): (f32,f32) = p.into();     // unpack values
        let x = q + (r - (r as i32 & 1) as f32) / 2.;
        let y = r;
        (x,y).into()
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
        self.to_cubic()
            .neighbors()
            .into_iter()
            .map(Self::from)
            .collect()
    }

    pub fn distance<T: Into<Point<Offset>>>(&self, other: T) -> i32 {
        self.as_cubic()
            .distance(other.into())
    }

    pub fn bounds(&self) -> (Point<Offset>,Point<Offset>) {
        let (w,h) = Context::size();
        (Point::new(-w/2,-h/2),
         Point::new(w/2,h/2))
    }

    pub fn in_bounds(&self) -> bool {
        let (s,e) = self.bounds();
        let (x,y) = self.integers();
        s.x <= x && s.y <= y &&
        e.x > x && e.y > y
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
        self.to_cubic()
            .neighbors()
            .into_iter()
            .map(Self::from)
            .collect()
    }

    pub fn distance<T: Into<Point<Axial>>>(&self, other: T) -> i32 {
        self.as_cubic()
            .distance(other.into())
    }

    pub fn bounds(&self) -> (Point<Axial>,Point<Axial>) {
        let (s,e) = self.as_offset().bounds();
        (s.into(),e.into())
    }

    pub fn in_bounds(&self) -> bool {
        self.to_offset().in_bounds()
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

    pub fn neighbors(&self) -> Vec<Point<Cubic>> {
        let (x,y,z) = self.integers();
        let mut result = Vec::with_capacity(6);
        if self.in_bounds() {
            let p1 = Point::cubic(x-1,y+1,z+0);
            let p2 = Point::cubic(x+0,y+1,z-1);
            let p3 = Point::cubic(x+0,y-1,z+1);
            let p4 = Point::cubic(x+1,y-1,z+0);
            let p5 = Point::cubic(x-1,y+0,z+1);
            let p6 = Point::cubic(x+1,y+0,z-1);
    
            if p1.in_bounds() { result.push(p1); }
            if p2.in_bounds() { result.push(p2); }
            if p3.in_bounds() { result.push(p3); }
            if p4.in_bounds() { result.push(p4); }
            if p5.in_bounds() { result.push(p5); }
            if p6.in_bounds() { result.push(p6); }
        }
        result
    }

    pub fn distance<T: Into<Point<Cubic>>>(&self, other: T) -> i32 {
        let (x1,y1,z1) = self.integers();
        let (x2,y2,z2) = other.into().integers();
        ((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs()) / 2
    }

    pub fn bounds(&self) -> (Point<Cubic>,Point<Cubic>) {
        let (s,e) = self.as_offset().bounds();
        (s.into(),e.into())
    }

    pub fn in_bounds(&self) -> bool {
        self.to_offset().in_bounds()
    }

}

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

    #[test]
    fn offset_to_axial_conversion() {
        initialize!(30,30);
        assert_eq!(Point::offset(-2,2).as_axial(),Point::axial(-3,2));
        assert_eq!(Point::offset(-2,-2).as_axial(),Point::axial(-1,-2));
        assert_eq!(Point::offset(2,-2).as_axial(),Point::axial(3,-2));
    }

    #[test]
    fn axial_to_offset_conversion() {
        initialize!(30,30);
        assert_eq!(Point::axial(-3,2).as_offset(),Point::offset(-2,2));
        assert_eq!(Point::axial(-1,-2).as_offset(),Point::offset(-2,-2));
        assert_eq!(Point::axial(3,-2).as_offset(),Point::offset(2,-2));
    }

    #[test]
    fn offset_to_cubic_conversion() {
        initialize!(30,30);
        assert_eq!(Point::offset(-2,2).as_cubic(),Point::cubic(-3,2,1));
        assert_eq!(Point::offset(-2,-2).as_cubic(),Point::cubic(-1,-2,3));
        assert_eq!(Point::offset(2,-2).as_cubic(),Point::cubic(3,-2,-1));
    }

    #[test]
    fn cubic_to_offset_conversion() {
        initialize!(30,30);
        assert_eq!(Point::cubic(-3,2,1).as_offset(),Point::offset(-2,2));
        assert_eq!(Point::cubic(-1,-2,3).as_offset(),Point::offset(-2,-2));
        assert_eq!(Point::cubic(3,-2,-1).as_offset(),Point::offset(2,-2));
    }

    #[test]
    fn neighbors_offset_compare_cubic_30x30() {
        initialize!(30,30);

        let n1 = point!(0,0)
            .neighbors();
        let n2 = cubic!(0,0)
            .neighbors()
            .into_iter()
            .map(|p| p.to_offset())
            .collect::<Vec<Point>>();

        assert_eq!(n1,n2);
    }

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