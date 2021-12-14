use crate::objects::NameGenerator;
use rand_distr::{Distribution, Normal, WeightedIndex};

macro_rules! weighted {
    ( $weights:expr ) => {
        WeightedIndex::new($weights).expect("Could not make WeightedIndex")
    };
}

#[derive(Debug, Clone, Copy)]
pub enum Sex {
    Male,
    Female,
}

pub struct Demographics {
    pub name: NameGenerator,
    pub sex: Division<'static, Sex>,
    pub age: Demographic,
    pub weight: Demographic,
    pub height: Demographic,
}

pub struct Division<'a, T> {
    weights: &'a [f32],
    results: &'a [T],
}

pub struct Variation {
    mean: f32,
    stdv: f32,
}

pub struct Demographic {
    min: f32,
    max: f32,
    male: Variation,
    female: Variation,
}

impl<T> Division<'_, T> {
    pub fn gen(&self) -> &T {
        self.results
            .get(weighted!(self.weights).sample(&mut rand::thread_rng()))
            .expect("Weight/result mismatch")
    }
}

impl Demographic {
    pub fn gen(&self, sex: &Sex) -> u32 {
        // force: 0 < min < max
        let max = self.max.max(0.);
        let min = self.min.max(0.).min(max);

        // geanerate
        match sex {
            Sex::Male => self.male.gen(),
            Sex::Female => self.female.gen(),
        }
        .clamp(min, max)
        .round() as u32
    }
}

impl Variation {
    pub fn gen(&self) -> f32 {
        let d = Normal::new(self.mean, self.stdv).expect("Could not create normal distribution");
        d.sample(&mut rand::thread_rng())
    }
}

// normal distributions:
//      https://www.simplypsychology.org/normal-distribution.html
// height:
//      https://ourworldindata.org/human-height
//      https://dhoroty.applebutterexpress.com/what-is-standard-deviation-height
// weight:
//      https://www.cdc.gov/nchs/data/series/sr_03/sr03-046-508.pdf
//          (std-dev calculated from (std_error) * sqrt(sample_size))
// military:
//      https://www.cfr.org/backgrounder/demographics-us-military
impl Default for Demographics {
    fn default() -> Self {
        Self {
            name: NameGenerator::new(),
            sex: Division {
                weights: &[0.8, 0.2],
                results: &[Sex::Male, Sex::Female],
            },
            age: Demographic {
                min: 18.,
                max: 40.,
                male: Variation {
                    mean: 21.,
                    stdv: 4.,
                },
                female: Variation {
                    mean: 21.,
                    stdv: 4.,
                },
            },
            weight: Demographic {
                min: 36.,  // ~80lb
                max: 136., // ~300lb
                male: Variation {
                    mean: 85.5,
                    stdv: 40.78 / 2., // div b/c military flattens distribution
                },
                female: Variation {
                    mean: 74.9,
                    stdv: 32.49 / 2., // div b/c military flattens distribution
                },
            },
            height: Demographic {
                min: 121., // ~4ft
                max: 243., // ~8ft
                male: Variation {
                    mean: 178.4,
                    stdv: 10.16,
                },
                female: Variation {
                    mean: 164.7,
                    stdv: 8.89,
                },
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn male_female_distribution() {
        let demographics = Demographics::default();
        let mut male = 0;
        let mut female = 0;
        for _ in 0..100 {
            match demographics.sex.gen() {
                Sex::Male => male += 1,
                Sex::Female => female += 1,
            };
        }
        // dbg!(male);
        // dbg!(female);
    }

    #[test]
    fn male_age_distribution() {
        let demographics = Demographics::default();
        for _ in 0..100 {
            let age = demographics.age.gen(&Sex::Male);
            // dbg!(age);
        }
    }

    #[test]
    fn female_age_distribution() {
        let demographics = Demographics::default();
        for _ in 0..100 {
            let age = demographics.age.gen(&Sex::Female);
            // dbg!(age);
        }
    }

    #[test]
    fn male_weight_distribution() {
        let demographics = Demographics::default();
        for _ in 0..100 {
            let weight = demographics.weight.gen(&Sex::Male);
            // dbg!(weight);
        }
    }

    #[test]
    fn male_height_distribution() {
        let demographics = Demographics::default();
        for _ in 0..100 {
            let height = demographics.height.gen(&Sex::Male);
            // dbg!(height);
        }
    }

    #[test]
    fn female_height_distribution() {
        let demographics = Demographics::default();
        for _ in 0..100 {
            let height = demographics.height.gen(&Sex::Female);
            // dbg!(height);
        }
    }
}
