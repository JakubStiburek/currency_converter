use std::error::Error;
use std::fmt::format;
use std::fs::{File};
use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt;
use hyper::{Client};
use hyper::body::HttpBody as _;
use clap::{Parser, ValueEnum};
use hyper_tls::HttpsConnector;
use serde_json::{Value};
use tokio::time::{timeout, Duration};

type Record = HashMap<String, String>;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Code {
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

struct Rate {
    code: Code,
    value: f32,
}

fn main() {
    load_rates();
}

fn load_rates() -> Result<(), Box<dyn Error>> {
    let mut home_dir: PathBuf = dirs::home_dir().unwrap();
    home_dir.push("eurofxref.csv");
    
    let file = File::open(home_dir)?;
    let mut rdr = csv::Reader::from_reader(file);
    let rates: Vec<String> = vec![];
    for result in rdr.deserialize() {
        let record: Record = result?;
        for (key, value) in record.iter() {
            // todo record date
            if (key.len() > 1 && !key.eq("Date")) {
                println!("{:?} - {:?}", key.trim(), value.trim().parse::<f32>()?)
            }
        }
    }
    
    Ok(())
}
