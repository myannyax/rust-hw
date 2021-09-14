fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn read_int() -> u8 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn main() {
    println!("Hi! You can start guessing now");
    let number: u8 = (random_int() % 100 + 1) as u8;
    loop {
        let guess = read_int();
        if guess != number {
            println!("Nope, guessed number is {} than this", if guess > number { "lesser" } else { "greater" });
        } else {
            break;
        }
    }
    println!("Right, it's {}!", number);
}
