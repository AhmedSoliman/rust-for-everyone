fn pick<T>(value: u8, even: T, odd: T) -> T {
    if value % 2 == 0 {
        even
    } else {
        odd
    }
}

fn main() {
    let v = rand::random();

    println!("You picked {}, it's {}", v, pick(v, "even", "odd"));
    println!("You picked {}, it's {}", v, pick(v, true, false));
    println!("You picked {}, it's {}", v, pick(v, 100, 200));
}
