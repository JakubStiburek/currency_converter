use clap::Parser;

#[derive(Parser, Debug)]
#[command(author="Jakub Stiburek", version="0.0.0", about="Simple currency converter.")]
struct Args {
    #[arg(short, long, help="The amount to be converted", default_value_t = 1)]
    amount: u32,
    
    #[arg(short, long, help="First currency code", default_value_t = String::from("eur"))]
    first: String,
    
    #[arg(short, long, help="Second currency code", default_value_t = String::from("usd"))]
    second: String,
    
    #[arg(short, long, help="List available currency codes", default_value_t = false)]
    codes: bool,
}

struct Code {
    
}

enum Codes {
    Eur,
    Usd,
    Czk,
    Hrn,
    Gbp
}

fn main() {
    let args = Args::parse();
    
    println!("Hello {}!", args.first);
    println!("Hello {}!", args.second);
    if args.codes {
        println!("Hello codes")
    }
}
