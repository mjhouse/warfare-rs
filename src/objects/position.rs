use std::fmt;
use crate::state::Context;

#[derive(Clone,PartialEq,Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// calculate an index from position
    fn to_index( &self ) -> i32 {
        let (w,h) = Context::size();
        let x = self.x + w / 2;
        let y = self.y + h / 2;
        x + y * w
    }

    /// create a position from an index
    fn from_index( i: i32 ) -> Self {
        let (w,h) = Context::size();
        let x = (i % w) - (w / 2);
        let y = (i / w) - (h / 2);
        Self::new(x,y)
    }

    pub fn neighbors( &self ) -> Vec<Self> {
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
        let p0 = Self::new(x,y).to_index();         // center    
        let mut p1 = Self::new(x-1,y+1).to_index(); // top-left  
        let mut p2 = Self::new(x,y+1).to_index();   // top-right 
        let mut p3 = Self::new(x-1,y-1).to_index(); // bot-left  
        let mut p4 = Self::new(x,y-1).to_index();   // bot-right 
        let p5 = Self::new(x-1,y).to_index();       // mid-left  
        let p6 = Self::new(x+1,y).to_index();       // mid-right 

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

        if c1 { result.push(Self::from_index(p1)); } // top-left
        if c2 { result.push(Self::from_index(p2)); } // top-right
        if c3 { result.push(Self::from_index(p3)); } // bot-left
        if c4 { result.push(Self::from_index(p4)); } // bot-right
        if c5 { result.push(Self::from_index(p5)); } // mid-left
        if c6 { result.push(Self::from_index(p6)); } // mid-right

        result
    }

}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x,self.y)
    }
}

impl From<(i32,i32)> for Position {
    fn from((x,y): (i32,i32)) -> Self {
        Position::new(x,y)
    }
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     macro_rules! initialize {
//         ( $w:expr, $h:expr ) => { 
//             crate::state::State::default().set_map_size($w,$h) 
//         }
//     }

//     macro_rules! index {
//         ( $i:expr ) => { Position::from_index($i) }
//     }

//     macro_rules! position {
//         ( $x:expr, $y:expr ) => { Position::new($x,$y) }
//     }

//     macro_rules! positions {
//         ($( ($x:expr,$y:expr) ),*) => {
//             vec![$( 
//                 position!($x, $y) 
//             ),*]
//         }
//     }

//     #[test]
//     fn position_to_index_30x30() {
//         initialize!(30,30);
//         assert_eq!(position!(-15,-15).to_index(), 0);
//         assert_eq!(position!(0,-8).to_index(), 225);
//         assert_eq!(position!(-11,-4).to_index(), 334);
//         assert_eq!(position!(14,0).to_index(), 479);
//         assert_eq!(position!(-15,4).to_index(), 570);
//         assert_eq!(position!( 14, 14).to_index(), 899);
//     }

//     #[test]
//     fn position_to_index_30x42() {
//         initialize!(30,42);
//         assert_eq!(position!(-15,-21).to_index(),0);
//         assert_eq!(position!(14,-21).to_index(),29);
//         assert_eq!(position!(14,20).to_index(),1259);
//         assert_eq!(position!(-15,20).to_index(),1230);
//         assert_eq!(position!(10,16).to_index(),1135);
//         assert_eq!(position!(0,-6).to_index(),465);
//         assert_eq!(position!(8,-17).to_index(),143);
//     }

//     #[test]
//     fn index_to_position_30x30() {
//         initialize!(30,30);
//         assert_eq!(index!(0),position!(-15,-15));
//         assert_eq!(index!(225),position!(0,-8));
//         assert_eq!(index!(334),position!(-11,-4));
//         assert_eq!(index!(479),position!(14,0));
//         assert_eq!(index!(570),position!(-15,4));
//         assert_eq!(index!(899),position!(14,14));
//     }

//     #[test]
//     fn index_to_position_30x42() {
//         initialize!(30,42);
//         assert_eq!(index!(0),position!(-15,-21));
//         assert_eq!(index!(29),position!(14,-21));
//         assert_eq!(index!(1259),position!(14,20));
//         assert_eq!(index!(1230),position!(-15,20));
//         assert_eq!(index!(1135),position!(10,16));
//         assert_eq!(index!(465),position!(0,-6));
//         assert_eq!(index!(143),position!(8,-17));
//     }

//     #[test]
//     fn position_neighbors_30x30_corners() {
//         initialize!(30,30);

//         // bottom-left corner
//         assert_eq!(
//             position!(-15,-15).neighbors(), 
//             positions![(-15,-14),(-14,-14),(-14,-15)],
//         );

//         // bottom-right corner
//         assert_eq!(
//             position!(14,-15).neighbors(), 
//             positions![(14,-14),(13,-15)],
//         );

//         // top-left corner
//         assert_eq!(
//             position!(-15,14).neighbors(), 
//             positions![(-15,13),(-14,14)],
//         );

//         // top-right corner
//         assert_eq!(
//             position!(14,14).neighbors(), 
//             positions![(13,13),(14,13),(13,14)],
//         );
//     }

//     #[test]
//     fn generator_group_30x30_quadrants() {
//         initialize!(30,30);

//         // top-left quadrant
//         assert_eq!(
//             position!(-10,10).neighbors(),
//             positions![(-11,11),(-10,11),(-11,9),(-10,9),(-11,10),(-9,10)],
//         );

//         // top-right quadrant
//         assert_eq!(
//             position!(10,10).neighbors(),
//             positions![(9,11),(10,11),(9,9),(10,9),(9,10),(11,10)],
//         );

//         // bottom-left quadrant
//         assert_eq!(
//             position!(-10,-10).neighbors(),
//             positions![(-11,-9),(-10,-9),(-11,-11),(-10,-11),(-11,-10),(-9,-10)],
//         );

//         // bottom-right quadrant
//         assert_eq!(
//             position!(10,-10).neighbors(),
//             positions![(9,-9),(10,-9),(9,-11),(10,-11),(9,-10),(11,-10)],
//         );
//     }

//     #[test]
//     fn generator_group_30x30_edges() {
//         initialize!(30,30);

//         // top-left quadrant top-edge
//         assert_eq!(
//             position!(-7,14).neighbors(),
//             positions![(-8,13),(-7,13),(-8,14),(-6,14)],
//         );

//         // top-left quadrant left edge
//         assert_eq!(
//             position!(-15,9).neighbors(),
//             positions![(-15,10), (-14,10), (-15,8), (-14,8), (-14,9)],
//         );

//         // bot-left quadrant bot-edge
//         assert_eq!(
//             position!(-9,-15).neighbors(),
//             positions![(-9,-14), (-8,-14), (-10,-15), (-8,-15)],
//         );

//         // bot-left quadrant left-edge
//         assert_eq!(
//             position!(-15,-10).neighbors(),
//             positions![(-15,-9), (-15,-11), (-14,-10)],
//         );

//         // bot-right quadrant bot-edge
//         assert_eq!(
//             position!(8,-15).neighbors(),
//             positions![(8,-14), (9,-14), (7,-15), (9,-15)],
//         );

//         // bot-right quadrant right-edge
//         assert_eq!(
//             position!(14,-8).neighbors(),
//             positions![(13,-7), (14,-7), (13,-9), (14,-9), (13,-8)],
//         );


//         // top-right quadrant right-edge
//         assert_eq!(
//             position!(14,10).neighbors(),
//             positions![(13,11), (14,11), (13,9), (14,9), (13,10)],
//         );

//         // top-right quadrant top-edge
//         assert_eq!(
//             position!(6,14).neighbors(),
//             positions![(5,13), (6,13), (5,14), (7,14)],
//         );

//     }

//     #[test]
//     fn generator_group_30x30_negative() {
//         initialize!(30,30);

//         // top-left quadrant top-edge
//         assert!(position!(-7,15).neighbors().is_empty());

//         // top-left quadrant left edge
//         assert!(position!(-16,9).neighbors().is_empty());

//         // bot-left quadrant bot-edge
//         assert!(position!(-9,-16).neighbors().is_empty());

//         // bot-left quadrant left-edge
//         assert!(position!(-16,-10).neighbors().is_empty());

//         // bot-right quadrant bot-edge
//         assert!(position!(8,-16).neighbors().is_empty());

//         // bot-right quadrant right-edge
//         assert!(position!(15,-8).neighbors().is_empty());

//         // top-right quadrant right-edge
//         assert!(position!(15,10).neighbors().is_empty());

//         // top-right quadrant top-edge
//         assert!(position!(6,15).neighbors().is_empty());
//     }
// }