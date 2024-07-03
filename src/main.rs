use rust_for_everyone::{Amount, Currency};

#[derive(Debug, Clone, Copy)]
pub struct USD;

impl Currency for USD {
    const CODE: &'static str = "USD";
    const SYMBOL: &'static str = "$";
    const RATIO: u8 = 100;
}

#[derive(Debug, Clone, Copy)]
pub struct GBP;

impl Currency for GBP {
    const CODE: &'static str = "GBP";
    const SYMBOL: &'static str = "£";
    const RATIO: u8 = 100;
}

fn main() {
    let a = Amount::new(100, USD);
    let z = Amount::new(100, USD);

    let b = Amount::new(88, GBP);

    println!("Amount of a={:#}", a);
    println!("Amount of b={}", b);

    println!("Amount of a+z={}", a + z);
    println!("Amount of a+b={}", a + b);
}
