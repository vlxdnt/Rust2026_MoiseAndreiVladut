fn is_prime(x: u32) -> bool {
    if x < 2 {
        return false;
    }

    let y = (x as f64).sqrt() as u32;
    for i in 2..=y {
        if x.is_multiple_of(i) {
            return false;
        }
    }

    true
}

fn is_coprime(mut x: u32, mut y: u32) -> bool {
    while y != 0 {
        let tmp: u32 = y;
        y = x % y;
        x = tmp;
    }

    x == 1
}

fn sing(x: u32) {
    if x < 1 {
        return;
    }

    for i in (2..=x).rev() {
        println!("{i} bottles of beer on the wall,");
        println!("{i} bottles of beer.");
        println!("Take one down, pass it around,");
        if i == 2 {
            println!("1 bottle of beer on the wall.");
        } else {
            println!("{} bottles of beer on the wall.", i - 1)
        }
        println!();
    }

    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer.");
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
}

fn main() {
    for i in 0..=100 {
        if is_prime(i) {
            println!("{i} is a prime");
        }
    }

    for i in 0..=100 {
        let mut coprimes = Vec::new();
        for j in (i + 1)..=100 {
            if is_coprime(i, j) {
                coprimes.push(j.to_string());
            }
        }

        if !coprimes.is_empty() {
            println!("{i} is coprime with: {}", coprimes.join(", "));
        }
    }

    sing(99);
}
