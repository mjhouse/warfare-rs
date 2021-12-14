use crate::state::demographics::Sex;
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

const LAST_NAMES: &str = "assets/data/last_names.txt";
const MALE_NAMES: &str = "assets/data/male_names.txt";
const FEMALE_NAMES: &str = "assets/data/female_names.txt";

static LAST_INDEX: AtomicUsize = AtomicUsize::new(0);
static MALE_INDEX: AtomicUsize = AtomicUsize::new(0);
static FEMALE_INDEX: AtomicUsize = AtomicUsize::new(0);

static LAST_MAX_INDEX: AtomicUsize = AtomicUsize::new(0);
static MALE_MAX_INDEX: AtomicUsize = AtomicUsize::new(0);
static FEMALE_MAX_INDEX: AtomicUsize = AtomicUsize::new(0);

// https://github.com/arineng/arincli/blob/master/lib/male-first-names.txt
static MALE: Lazy<Vec<String>> = Lazy::new(|| {
    let mut result: Vec<String> =
        BufReader::new(File::open(MALE_NAMES).expect("Cannot find male names"))
            .lines()
            .into_iter()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
    MALE_MAX_INDEX.store(result.len(), Ordering::SeqCst);
    result.shuffle(&mut thread_rng());
    log::info!("Loaded male first names");
    result
});

// https://github.com/arineng/arincli/blob/master/lib/female-first-names.txt
static FEMALE: Lazy<Vec<String>> = Lazy::new(|| {
    let mut result: Vec<String> =
        BufReader::new(File::open(FEMALE_NAMES).expect("Cannot find female names"))
            .lines()
            .into_iter()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
    FEMALE_MAX_INDEX.store(result.len(), Ordering::SeqCst);
    result.shuffle(&mut thread_rng());
    log::info!("Loaded female first names");
    result
});

// https://github.com/arineng/arincli/blob/master/lib/last-names.txt
static LAST: Lazy<Vec<String>> = Lazy::new(|| {
    let mut result: Vec<String> =
        BufReader::new(File::open(LAST_NAMES).expect("Cannot find last names"))
            .lines()
            .into_iter()
            .filter(Result::is_ok)
            .map(Result::unwrap)
            .collect();
    LAST_MAX_INDEX.store(result.len(), Ordering::SeqCst);
    result.shuffle(&mut thread_rng());
    log::info!("Loaded last names");
    result
});

// do not call until after LAST names list has
// been accessed for the first time
fn last_index() -> usize {
    let v = LAST_INDEX.fetch_add(1, Ordering::SeqCst);
    let m = LAST_MAX_INDEX.load(Ordering::SeqCst);
    if v >= m {
        LAST_INDEX.store(0, Ordering::SeqCst);
        0
    } else {
        v
    }
}

// do not call until after MALE names list has
// been accessed for the first time
fn male_index() -> usize {
    let v = MALE_INDEX.fetch_add(1, Ordering::SeqCst);
    let m = MALE_MAX_INDEX.load(Ordering::SeqCst);
    if v >= m {
        MALE_INDEX.store(0, Ordering::SeqCst);
        0
    } else {
        v
    }
}

// do not call until after FEMALE names list has
// been accessed for the first time
fn female_index() -> usize {
    let v = FEMALE_INDEX.fetch_add(1, Ordering::SeqCst);
    let m = FEMALE_MAX_INDEX.load(Ordering::SeqCst);
    if v >= m {
        FEMALE_INDEX.store(0, Ordering::SeqCst);
        0
    } else {
        v
    }
}

#[derive(Debug, Clone)]
pub struct Name(String);
pub struct NameGenerator;

impl NameGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn male(&self) -> Name {
        let first = Lazy::force(&MALE); // trigger load
        let last = Lazy::force(&LAST); // trigger load
        let mi = male_index();
        let li = last_index();
        Name(format!("{} {}", first[mi], last[li]))
    }

    pub fn female(&self) -> Name {
        let first = Lazy::force(&FEMALE); // trigger load
        let last = Lazy::force(&LAST); // trigger load
        let fi = female_index();
        let li = last_index();
        Name(format!("{} {}", first[fi], last[li]))
    }

    pub fn gen(&self, sex: &Sex) -> Name {
        match sex {
            Sex::Male => self.male(),
            Sex::Female => self.female(),
        }
    }
}

impl Name {
    pub fn inner(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn male_name_generation() {
        let name = NameGenerator::new().male();
        // dbg!(name);
    }

    #[test]
    fn female_name_generation() {
        let name = NameGenerator::new().female();
        // dbg!(name);
    }
}
