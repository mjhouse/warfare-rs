use serde::{Serialize,Deserialize};
use std::ops::{Add,Sub,AddAssign,SubAssign};
use std::fmt::Debug;

#[derive(Serialize,Deserialize,Clone,Copy,Debug)]
pub struct Property {
    sta: u8,
    val: u8,
    min: u8,
    max: u8,
}

fn up<T>(val: T) -> i64
where
    T: Into<i64>
{
    val.into()
}

fn bound<T,C>(val: T, min: T, max: T, chng: C) -> i64 
where 
    T: Into<i64>, 
    C: Into<i64>
{
    up(val)
        .saturating_add(up(chng))
        .max(up(min))
        .min(up(max))
}

impl Property {

    pub fn new(val: u8, min: u8, max: u8) -> Self {
        assert!(min <= max);
        assert!(val <= max);
        assert!(val >= min);
        Self { 
            sta: val,
            val: val, 
            min: min,
            max: max
        }
    }

    pub fn inner(&self) -> (u8,u8,u8) {
        ( self.val, self.min, self.max )
    }

    pub fn val(&self) -> u8 {
        self.val
    }

    pub fn min(&self) -> u8 {
        self.min
    }

    pub fn max(&self) -> u8 {
        self.max
    }

    pub fn reset(&mut self) {
        self.val = self.sta;
    }

    pub fn set<T: Into<i64>>(&mut self, value: T) {
        self.val = bound(
            0,
            self.min,
            self.max,
            value) as u8;
    }

    pub fn update<T: Into<i64>>(&mut self, change: T) {
        self.val = bound(
            self.val,
            self.min,
            self.max,
            change) as u8;
    }

    fn updated<T: Into<i64>>(mut self, change: T) -> Self {
        self.update(change);
        self
    }
}

impl<T: Into<i64>> Add<T> for Property {
    type Output = Self;
    fn add(self, other: T) -> Self {
        self.updated(other)
    }
}

impl<T: Into<i64>> Sub<T> for Property {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        self.updated(-1 * up(other))
    }
}

impl<T: Into<i64>> AddAssign<T> for Property {
    fn add_assign(&mut self, other: T) {
        self.update(other);
    }
}

impl<T: Into<i64>> SubAssign<T> for Property {
    fn sub_assign(&mut self, other: T) {
        self.update(-1 * up(other));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_works_sanity_check() {
        let prop = Property::new(0,0,100);
        assert_eq!(prop.val(),0);
        assert_eq!(prop.min(),0);
        assert_eq!(prop.max(),100);
    }

    #[test]
    fn test_update_stays_in_bounds() {
        let mut prop = Property::new(0,0,100);
        for _ in 0..110 {
            prop.update(1);
            assert!(prop.val() <= 100);
        }

        for _ in 0..120 {
            prop.update(-1);
            assert!(prop.val() >= 0);
        }
    }

    #[test]
    fn test_set_stays_in_bounds() {
        let mut prop = Property::new(0,0,100);
        for i in -10..110 {
            prop.set(i);
            assert!(prop.val() <= 100);
        }

        for i in -10..110 {
            prop.set(i);
            assert!(prop.val() >= 0);
        }
    }

    #[test]
    fn test_implicit_ops() {
        let mut prop = Property::new(0,0,100).updated(50);
        assert_eq!(prop.val(),50);

        prop = prop + 10;
        assert_eq!(prop.val(),60);

        prop += 10;
        assert_eq!(prop.val(),70);

        prop = prop - 10;
        assert_eq!(prop.val(),60);

        prop -= 10;
        assert_eq!(prop.val(),50);
    }
}