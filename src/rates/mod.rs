use crate::prelude::*;
use std::error::Error;
use std::fs::{File};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt;
use clap::{ValueEnum};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::prelude::*;

type Record = HashMap<String, String>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, EnumIter)]
pub enum Code {
    EUR,
    CHF,
    SGD,
    HKD,
    INR,
    BRL,
    JPY,
    ISK,
    MXN,
    MYR,
    NOK,
    BGN,
    HUF,
    CZK,
    HRK,
    THB,
    RON,
    USD,
    TRY,
    GBP,
    AUD,
    CAD,
    SEK,
    KRW,
    DKK,
    PHP,
    CNY,
    IDR,
    ILS,
    ZAR,
    NZD,
    PLN,
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Rate {
    pub code: Code,
    pub value: f32,
}

impl Rate {
    fn new(code: Code, value: f32) -> Self {
        Self {
            code,
            value,
        }
    }
}

pub fn load_rates() -> Result<Vec<Rate>, Box<dyn Error>> {
    let mut home_dir: PathBuf = dirs::home_dir().unwrap();
    home_dir.push("eurofxref.csv");
    
    let file = File::open(home_dir)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut rates: Vec<Rate> = vec![];
    rates.push(Rate::new(Code::EUR, 1.0));
    for result in rdr.deserialize() {
        let record: Record = result?;
        for (key, value) in record.iter() {
            // todo record date
            if (key.len() > 1 && !key.eq("Date")) {
                let mut code: Code = Code::EUR;
                for c in Code::iter() {
                    if (c.to_string().eq(key.trim())) {
                        code = c;
                    }
                }
                let rate = Rate::new(code, value.trim().parse::<f32>()?);
                rates.push(rate);
            }
        }
    }
    
    Ok(rates)
}
