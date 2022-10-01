use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};
use clap::Parser;
use hyper_tls::HttpsConnector;
use serde::{Serialize, Deserialize};
use serde_json::{Value};

#[derive(Parser, Debug)]
#[command(author = "Jakub Stiburek", version = "0.0.0", about = "Simple currency converter.")]
struct Args {
    #[arg(short, long, help = "The amount to be converted", default_value_t = 1.0)]
    amount: f32,
    
    #[arg(short, long, help = "First currency code", default_value_t = String::from("eur"))]
    first: String,
    
    #[arg(short, long, help = "Second currency code", default_value_t = String::from("usd"))]
    second: String,
    
    #[arg(short, long, help = "List available currency codes", default_value_t = false)]
    codes: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let args = Args::parse();
    
    println!("{}", args.first);
    println!("{}", args.second);
    if args.codes {
        println!("codes")
    }
    
    let uri = format!("https://cdn.jsdelivr.net/gh/fawazahmed0/currency-api@1/latest/currencies/{first}/{second}.json", first = args.first, second = args.second).parse()?;

    let mut resp = client.get(uri).await?;
    
    println!("Response: {}", resp.status());
    
    let mut data = Vec::new();
    while let Some(chunk) = resp.body_mut().data().await {
        data.extend(&chunk?)
    }
    
    let parsed: Value = serde_json::from_slice(&data)?;
    let rate = parsed[&args.second].to_string();
    
    println!("{} {} is {} {}", args.amount, args.first, rate.parse::<f32>()? * args.amount, &args.second);
    
    Ok(())
}
