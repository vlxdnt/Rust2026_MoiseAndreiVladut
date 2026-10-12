fn add_chars_n(mut s: String, c: char, n: u32) -> String {
    for _ in 1..=n {
        s.push(c);
    }
    s
}

fn add_chars_n_2(s: &mut String, c: char, n: u32) {
    for _ in 1..=n {
        s.push(c);
    }
}

fn add_space(s: &mut String, n: u32) {
    for _ in 1..=n {
        s.push(' ')
    }
}

fn add_str(s: &mut String, s2: &str) {
    s.push_str(s2);
}

fn add_integer(s: &mut String, num: i32) {
    if num == 0 {
        s.push('0');
        return;
    }

    let mut n = num.unsigned_abs();
    let mut buffer = [0u8; 32];
    let mut index = 0;
    let mut digit_count = 0;

    while n > 0 {
        if digit_count > 0 && digit_count % 3 == 0 {
            buffer[index] = b'_';
            index += 1;
        }

        let digit = (n % 10) as u8;
        buffer[index] = b'0' + digit;
        index += 1;
        digit_count += 1;

        n /= 10;
    }

    if num < 0 {
        s.push('-');
    }

    for i in (0..index).rev() {
        s.push(buffer[i] as char);
    }
}

fn add_float(s: &mut String, mut num: f64, precision: u8) {
    if num < 0.0 {
        s.push('-');
        num = -num;
    }

    let int_part = num as u64;
    let mut frac_part = num - (int_part as f64);

    if int_part == 0 {
        s.push('0');
    } else {
        let mut n = int_part;
        let mut buffer = [0u8; 32];
        let mut index = 0;

        while n > 0 {
            buffer[index] = b'0' + (n % 10) as u8;
            index += 1;
            n /= 10;
        }

        for i in (0..index).rev() {
            s.push(buffer[i] as char);
        }
    }

    if precision > 0 {
        s.push('.');

        for _ in 0..precision {
            frac_part *= 10.0;
            let digit = frac_part as u8;

            s.push((b'0' + digit) as char);

            frac_part -= digit as f64;
        }
    }
}

/////////////////////////////////////////////////////////// main funcs
fn p_1() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

fn p_2() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        add_chars_n_2(&mut s, c, 26 - i);

        i += 1;
    }

    print!("{}", s);
}

fn p_3() {
    let mut s = String::new();

    add_space(&mut s, 40);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_str(&mut s, "💚\n");

    add_space(&mut s, 40);
    add_str(&mut s, "RUST.\n\n");

    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 4);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 10);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 9);
    add_str(&mut s, "is\n");

    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 7);
    add_str(&mut s, "has");
    add_space(&mut s, 12);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 7);
    add_str(&mut s, "version");
    add_space(&mut s, 5);
    add_float(&mut s, 2.038, 3);
    add_str(&mut s, ".\n");

    print!("{}", s);
}

fn main() {
    p_1();
    println!();
    p_2();
    println!();
    p_3();
    println!();
}
