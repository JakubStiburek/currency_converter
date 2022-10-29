mod rates;

mod prelude {
    pub use crate::rates::*;
}

use prelude::*;

fn main() {
    let rates = rates::load_rates().expect("Failed to load rates");
    
    for rate in rates.iter() {
        println!("{}: {}", rate.code.to_string(), rate.value)
    }
}
