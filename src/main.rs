mod rates;

use std::fmt;
use hyper::{Client};
use hyper::body::HttpBody as _;
use clap::{Parser, ValueEnum};
use hyper_tls::HttpsConnector;
use serde_json::{Value};

mod prelude {
    pub use crate::rates::*;
}

use prelude::*;

#[derive(Parser, Debug)]
#[command(author = "Jakub Stiburek", version = "0.0.0", about = "Simple currency converter.")]
struct Args {
    amount: Option<f32>,
    
    first: Option<Code>,
    
    second: Option<Code>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let args = Args::parse();
    
    let first = match args.first {
        Some(c) => c.to_string().to_lowercase(),
        None => Code::EUR.to_string().to_lowercase()
    };
    let second = match args.second {
        Some(c) => c.to_string().to_lowercase(),
        None => Code::CZK.to_string().to_lowercase()
    };
    let amount = match args.amount {
        Some(a) => a,
        None => 1.0
    };
    
    let uri = format!("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/{first}/{second}.json", first = &first, second = &second).parse()?;
    
    let mut resp = client.get(uri).await?;
    
    let mut data = Vec::new();
    while let Some(chunk) = resp.body_mut().data().await {
        data.extend(&chunk?)
    }
    
    let parsed: Value = serde_json::from_slice(&data)?;
    let rate = parsed[&second].to_string();
    
    println!("{} {} is {} {}", &amount, &first, rate.parse::<f32>()? * &amount, &second);
    
    Ok(())
}
