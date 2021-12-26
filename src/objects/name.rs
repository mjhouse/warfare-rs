use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use crate::state::demographics::Sex;

macro_rules! read {
    ( $p:expr ) => {
        BufReader::new(
            File::open($p)
                .expect("Cannot read name file"))
            .lines()
            .into_iter()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect()
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct NameBound {
    index: usize,
    max:   usize,
}

struct NameData {
    names: Vec<String>,
    bound: NameBound,
}

type NameSet = Lazy<Mutex<NameData>>;

// https://github.com/arineng/arincli/blob/master/lib/male-first-names.txt
static MALE: NameSet = Lazy::new(|| {
    let mut names: Vec<String> = read!("assets/data/male_names.txt");
    names.shuffle(&mut rand::thread_rng());

    let index = 0;
    let max = names.len();
    Mutex::new(NameData { names, bound: NameBound{ index, max } })
});

// https://github.com/arineng/arincli/blob/master/lib/female-first-names.txt
static FEMALE: NameSet = Lazy::new(|| {
    let mut names: Vec<String> = read!("assets/data/female_names.txt");
    names.shuffle(&mut rand::thread_rng());

    let index = 0;
    let max = names.len();
    Mutex::new(NameData { names, bound: NameBound{ index, max } })
});

// https://github.com/arineng/arincli/blob/master/lib/last-names.txt
static LAST: NameSet = Lazy::new(|| {
    let mut names: Vec<String> = read!("assets/data/last_names.txt");
    names.shuffle(&mut rand::thread_rng());

    let index = 0;
    let max = names.len();
    Mutex::new(NameData { names, bound: NameBound{ index, max } })
});

const COUNT: usize = 3;
static SETS: [&NameSet;COUNT] = [&MALE,&FEMALE,&LAST];

fn next(idx: usize) -> String {
    let set = &mut SETS[idx].lock().unwrap();
    let mut v = set.bound.index + 1;
    if v >= set.bound.max {
        v = 0;
    }
    set.bound.index = v;
    set.names[v].clone()
}

pub struct NameGenerator;

#[derive(Deserialize, Serialize, Default, Debug, Clone, Eq, PartialEq)]
pub struct Names {
    pub bounds: [NameBound;COUNT],
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct Name {
    first: String,
    last:  String,
}

impl Name {
    pub fn male() -> Self {
        Self {
            first: next(0),
            last:  next(2),
        }
    }

    pub fn female() -> Self {
        Self {
            first: next(1),
            last:  next(2),
        }
    }
    
    pub fn first(&self) -> &str {
        self.first.as_str()
    }

    pub fn last(&self) -> &str {
        self.last.as_str()
    }

    pub fn full(&self) -> String {
        format!("{} {}",self.first,self.last)
    }
}

impl From<Name> for String {
    fn from(name: Name) -> String {
        name.full()
    }
}

impl NameGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn male(&self) -> Name {
        Name::male()
    }

    pub fn female(&self) -> Name {
        Name::female()
    }

    pub fn gen(&self, sex: &Sex) -> Name {
        match sex {
            Sex::Male => Name::male(),
            Sex::Female => Name::female(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // #[test]
    // fn male_name_generation() {
    //     let name = NameGenerator::new().male();
    //     // dbg!(name);
    // }

    // #[test]
    // fn female_name_generation() {
    //     let name = NameGenerator::new().female();
    //     // dbg!(name);
    // }

    // #[test]
    // fn name_gen_preseeded() {
    //     // manually change seed and verify
    //     NameGenerator::init(Names::new(12345));
    //     let gen = NameGenerator::new();
    //     for _ in 0..10 {
    //         dbg!(gen1.male());
    //     }
    // }
}
