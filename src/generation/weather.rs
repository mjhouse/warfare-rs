use rand::distributions::{WeightedIndex,Distribution};

#[derive(Clone,Copy,PartialEq)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Storm,
    Snowstorm,
    Rain,
    Snow,
}

pub struct WeatherState {
    pub start:  WeatherType,
    pub finish: WeatherType,
    pub chance: f32,
}

pub struct Weather {
    current: WeatherType,
    states: Vec<WeatherState>,
}

impl WeatherState {

    pub fn new(start: WeatherType, finish: WeatherType, chance: f32) -> Self {
        Self { start, finish, chance }
    }

}

impl Weather {

    pub fn new() -> Self {
        use WeatherType::*;

        Self {
            current: Clear,
            states: vec![
                // clear
                WeatherState::new(Clear,Clear,0.4),
                WeatherState::new(Clear,Cloudy,0.3),

                // cloudy
                WeatherState::new(Cloudy,Cloudy,0.3),
                WeatherState::new(Cloudy,Clear,0.3),
                WeatherState::new(Cloudy,Rain,0.2),
                WeatherState::new(Cloudy,Storm,0.2),
                
                // rainy
                WeatherState::new(Rain,Rain,0.4),
                WeatherState::new(Rain,Storm,0.2),
                WeatherState::new(Rain,Cloudy,0.4),

                // stormy
                WeatherState::new(Storm,Storm,0.1),
                WeatherState::new(Storm,Rain,0.4),
                WeatherState::new(Storm,Cloudy,0.5),
            ]
        }
    }

    pub fn update( &mut self ) {
        // filter for states starting at current, split into next states
        // and percentage odds.
        let (states,chances): (Vec<WeatherType>,Vec<f32>) = self.states
            .iter()
            .filter(|s| s.start == self.current)
            .map(|s| (s.finish,s.chance))
            .unzip();

        // build a weighted index that biases the outcome
        // based on `chances`.
        let dst = WeightedIndex::new(&chances).unwrap();
        let mut rng = rand::thread_rng();

        // select next state
        self.current = states[dst.sample(&mut rng)];
    }

    pub fn current( &self, t: f32 ) -> WeatherType {
        use WeatherType::*;
        match self.current {
            Storm if t <= 0. => Snowstorm,
            Rain  if t <= 0. => Snow,
            v => v,
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_succeeds() {
        let mut weather = Weather::new();
        for _ in 0..100 {
            use WeatherType::*;

            // match weather.current(0.) {
            //     Clear => println!("Clear"),
            //     Cloudy => println!("Cloudy"),
            //     Storm => println!("Storm"),
            //     Snowstorm => println!("Snowstorm"),
            //     Rain => println!("Rain"),
            //     Snow => println!("Snow"),
            // };

            weather.update();
        }
    }
}