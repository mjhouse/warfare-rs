use chrono::naive::NaiveDate;
use chrono::Datelike;
use std::fmt::{ Display, Formatter, Debug, Result as FmtResult};

/// default year
const YEAR: i32 = 2021;

#[derive(Clone)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Clone)]
pub struct Calendar {
    turn: u32,
    date: NaiveDate,
    season: Season,
}

impl Season {
    pub fn from(turn: u32) -> Self {
        match turn % 365 {
            v if v <= 079 => Self::Winter,
            v if v <= 171 => Self::Spring,
            v if v <= 265 => Self::Summer,
            v if v <= 355 => Self::Autumn,
            v if v <= 365 => Self::Winter,
            _ => unreachable!(),
        }
    }
}

impl Calendar {

    pub fn from_date(year: i32, month: u32, day: u32) -> Self {
        let date = NaiveDate::from_ymd(year,month,day);
        let turn = date.ordinal();
        let season = Season::from(turn);

        Self { turn, date, season }
    }

    pub fn from_turn(turn: u32) -> Self {
        let date = NaiveDate::from_yo(YEAR,turn);
        let season = Season::from(turn);

        Self { turn, date, season }
    }

    pub fn advance(&mut self) {
        self.turn += 1;
        self.date = self.date.succ();
        self.season = Season::from(self.turn);
    }

    pub fn season(&self) -> Season {
        self.season.clone()
    }

}

impl Default for Calendar {
    fn default() -> Self {
        Self::from_turn(1)
    }
}

impl Display for Calendar {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}: {} ({})", 
            self.turn, 
            self.date, 
            self.season )
    }
}

impl Display for Season {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use Season::*;
        match self {
            Spring => write!(f, "Spring"),
            Summer => write!(f, "Summer"),
            Autumn => write!(f, "Autumn"),
            Winter => write!(f, "Winter"),
        }
    }
}

impl Debug for Calendar {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(self,f)
    }
}