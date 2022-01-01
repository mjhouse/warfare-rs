use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Mutex;

const MESSAGE: &str = "Could not lock flag mutex";
const INITIAL: usize = 10;

#[macro_export]
macro_rules! flags {
    ( ) => { Flags::new() };
    ( $( $f: expr ),* ) => {{
        let mut flags = Flags::new();
        $( flags.set($f); )*
        flags
    }}
}

macro_rules! access {
    ( $s: ident ) => { $s.0.lock().expect(MESSAGE) }
}

macro_rules! create {
    ( $s: expr ) => { Mutex::new(HashSet::with_capacity($s)) }
}

/// thread-safe collection of flags
pub struct Flags<T>(Mutex<HashSet<T>>);

impl<T> Flags<T>
where 
    T: Eq + Hash + Copy
{
    /// create new collection of flags
    pub fn new() -> Self {
        Self(create!(INITIAL))
    }

    /// set or unset the flag, return true if already set
    pub fn update(&mut self, flag: T, value: bool) -> bool {
        if value { self.set(flag) }
        else { self.unset(flag) }
    }

    /// toggle the flag, return true if already set
    pub fn toggle(&mut self, flag: T) -> bool {
        self.update(flag,!self.get(flag))
    }

    /// set the flag, return true if already set
    pub fn set(&mut self, flag: T) -> bool {
        !access!(self).insert(flag)
    }

    /// unset the flag, return true if already set
    pub fn unset(&mut self, flag: T) -> bool {
        access!(self).remove(&flag)
    }

    /// check if flag is set
    pub fn get(&self, flag: T) -> bool {
        access!(self).contains(&flag)
    }

    /// clear all flags from set
    pub fn clear(&mut self) {
        access!(self).clear();
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    enum TestFlags {
        First,
        Second,
        Third,
    }

    #[test]
    fn flag_init_test() {
        let mut flags: Flags<TestFlags> = Flags::new();
        assert!(!flags.set(TestFlags::Second));
        assert!(flags.get(TestFlags::Second));

        flags.clear();
        assert!(!flags.get(TestFlags::Second));
    }

    #[test]
    fn flag_macro_test() {
        let mut flags = flags![
            TestFlags::Second,
            TestFlags::Third
        ];

        assert!(!flags.get(TestFlags::First));
        assert!(flags.get(TestFlags::Second));
        assert!(flags.get(TestFlags::Third));
    }

    #[test]
    fn flag_toggle_test() {
        let mut flags = flags![];

        assert!(!flags.toggle(TestFlags::First));
        assert!(flags.get(TestFlags::First));
    }
}