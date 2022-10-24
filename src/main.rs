use hyper::{Client};
use hyper::body::HttpBody as _;
use clap::{Parser, ValueEnum};
use hyper_tls::HttpsConnector;
use serde_json::{Value};
use std::fmt;
use tokio::time::{timeout, Duration};

#[derive(Parser, Debug)]
#[command(author = "Jakub Stiburek", version = "0.0.0", about = "Simple currency converter.")]
struct Args {
    amount: Option<f32>,
    
    first: Option<Code>,
    
    second: Option<Code>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Code {
    Czk,
    Eur,
    Usd,
    Gbp,
    Pln
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

static CODES: [&str; 5] = ["czk", "eur", "usd", "gbp", "pln"];

struct Pair {
    first: String,
    second: String
}

impl Pair {
    fn new(first: String, second: String) -> Self {
        Self {
            first,
            second
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let args = Args::parse();
    
    let first = match args.first {
        Some(c) => c.to_string().to_lowercase(),
        None => Code::Eur.to_string().to_lowercase()
    };
    let second = match args.second {
        Some(c) => c.to_string().to_lowercase(),
        None => Code::Czk.to_string().to_lowercase()
    };
    let amount = match args.amount {
        Some(a) => a,
        None => 1.0
    };
    
    let mut pairs: Vec<Pair> = vec![];
    
    for i in CODES.iter() {
        for j in  CODES.iter(){
            pairs.push(Pair::new(i.to_string(), j.to_string()))
        }
    }
    
    let mut uris: Vec<String> = vec![];
    
    for i in pairs.iter() {
        uris.push(format!("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/{}/{}.json", &i.first, &i.second));
    }
    
    let mut rates: Vec<f32> = vec![];
    
    for uri in uris.iter() {
        let mut resp = client.get(uri.parse()?).await?;
        
        println!("{}", &uri);
    
        let mut data = Vec::new();
        while let Some(chunk) = resp.body_mut().data().await {
            data.extend(&chunk?)
        }
    
        let parsed: Value = serde_json::from_slice(&data)?;
        let rate = parsed[&second].to_string();
        
        println!("{}", &rate);
        
        rates.push(rate.parse::<f32>()? * &amount);
    }
    
    for r in rates {
        println!("{}", r);
    }
    
    Ok(())
}
