trait Random {
    fn generate() -> Self;
}

impl Random for u8 {
    fn generate() -> Self {
        55
    }
}

impl Random for &str {
    fn generate() -> Self {
        "hola"
    }
}

fn random<T>() -> T
where
    T: Random,
{
    T::generate()
}

fn pick(v: &str) {
    println!("You picked {}", v);
}

fn main() {
    //let v = rand::random();

    let v = random();
    pick(v);
}
