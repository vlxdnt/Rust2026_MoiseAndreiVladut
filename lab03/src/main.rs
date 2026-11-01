fn is_prime(n: u16) -> bool {
    if n < 2 {
        return false;
    }

    let n = n as u32;
    let mut d: u32 = 2;
    while d * d <= n {
        if n.is_multiple_of(d) {
            return false;
        }
        d += 1;
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut n = x;
    loop {
        if n == u16::MAX {
            return None;
        }
        n += 1;
        if is_prime(n) {
            return Some(n);
        }
    }
}

fn problem1() {
    println!("=============================================");
    // success cases
    assert_eq!(next_prime(0), Some(2));
    assert_eq!(next_prime(10), Some(11));
    assert_eq!(next_prime(13), Some(17));
    assert_eq!(next_prime(65520), Some(65521));

    // fail cases
    assert_eq!(next_prime(65521), None);
    assert_eq!(next_prime(u16::MAX), None);

    let mut count = 0;
    let mut current: u16 = 0;
    while let Some(p) = next_prime(current) {
        count += 1;
        if count <= 10 || p > 65500 {
            println!("prime #{count}: {p}");
        } else if count == 11 {
            println!("...");
        }
        current = p;
    }
    println!("total primes in u16: {count}");
}

///////////////////////////////////////////////////
fn add_or_panic(a: u32, b: u32) -> u32 {
    match a.checked_add(b) {
        Some(r) => r,
        None => panic!("Addition overflow: {a} + {b} does not fit in an u32"),
    }
}

fn mul_or_panic(a: u32, b: u32) -> u32 {
    match a.checked_mul(b) {
        Some(r) => r,
        None => panic!("Multiplication overflow: {a} * {b} does not fit in an u32"),
    }
}

fn add_overflow() -> u32 {
    add_or_panic(u32::MAX, 1)
}

fn mul_overflow() -> u32 {
    mul_or_panic(65536, 65536)
}

fn problem2() {
    println!("=============================================");
    // success cases
    assert_eq!(add_or_panic(2, 3), 5);
    assert_eq!(add_or_panic(u32::MAX, 0), u32::MAX);
    assert_eq!(mul_or_panic(1000, 1000), 1_000_000);
    assert_eq!(mul_or_panic(u32::MAX, 1), u32::MAX);

    let r = std::panic::catch_unwind(add_overflow);
    assert!(r.is_err());
    let r = std::panic::catch_unwind(mul_overflow);
    assert!(r.is_err());
    println!("panic cases behaved as expected");
}

////////////////////////////////////////////
#[derive(Debug, PartialEq)]
enum MathError {
    AdditionOverflow,
    MultiplicationOverflow,
}

fn add_checked(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_add(b) {
        Some(r) => Ok(r),
        None => Err(MathError::AdditionOverflow),
    }
}

fn mul_checked(a: u32, b: u32) -> Result<u32, MathError> {
    match a.checked_mul(b) {
        Some(r) => Ok(r),
        None => Err(MathError::MultiplicationOverflow),
    }
}

fn compute(a: u32, b: u32, c: u32) -> Result<u32, MathError> {
    let sum = add_checked(a, b)?;
    let result = mul_checked(sum, c)?;
    Ok(result)
}

fn problem3() {
    println!("=============================================");

    // success
    assert_eq!(add_checked(2, 3), Ok(5));
    assert_eq!(mul_checked(6, 7), Ok(42));
    assert_eq!(compute(2, 3, 4), Ok(20));

    // fail
    assert_eq!(add_checked(u32::MAX, 1), Err(MathError::AdditionOverflow));
    assert_eq!(
        mul_checked(1 << 16, 1 << 16),
        Err(MathError::MultiplicationOverflow)
    );
    assert_eq!(compute(u32::MAX, 1, 2), Err(MathError::AdditionOverflow));
    assert_eq!(compute(1 << 16, 0, 1), Ok(65536));
    assert_eq!(
        compute(1 << 15, 1 << 15, 1 << 16),
        Err(MathError::MultiplicationOverflow)
    );

    println!("compute(2, 3, 4)          = {:?}", compute(2, 3, 4));
    println!("compute(MAX, 1, 2)        = {:?}", compute(u32::MAX, 1, 2));
    println!(
        "compute(32768, 32768, 65536) = {:?}",
        compute(1 << 15, 1 << 15, 1 << 16)
    );
}

//////////////////////////////////////////////
#[derive(Debug, PartialEq)]
enum CharError {
    NotAscii,
    NotDigit,
    NotBase16Digit,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if c.is_ascii_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(CharError::NotLetter)
    }
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if c.is_ascii_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(CharError::NotLetter)
    }
}

fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        Err(CharError::NotPrintable)
    } else {
        println!("{c}");
        Ok(())
    }
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    if !c.is_ascii_digit() {
        return Err(CharError::NotDigit);
    }
    Ok(c as u32 - '0' as u32)
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotAscii);
    }
    match c.to_digit(16) {
        Some(v) => Ok(v),
        None => Err(CharError::NotBase16Digit),
    }
}

fn print_error(e: &CharError) {
    match e {
        CharError::NotAscii => println!("Error: the character is not an ASCII character"),
        CharError::NotDigit => println!("Error: the character is not a digit"),
        CharError::NotBase16Digit => println!("Error: the character is not a base16 digit"),
        CharError::NotLetter => println!("Error: the character is not a letter"),
        CharError::NotPrintable => println!("Error: the character is not printable"),
    }
}

fn problem4() {
    println!("=============================================");

    // success
    assert_eq!(to_uppercase('a'), Ok('A'));
    assert_eq!(to_lowercase('Z'), Ok('z'));
    assert_eq!(print_char('x'), Ok(()));
    assert_eq!(char_to_number('7'), Ok(7));
    assert_eq!(char_to_number_hex('f'), Ok(15));
    assert_eq!(char_to_number_hex('B'), Ok(11));
    assert_eq!(char_to_number_hex('3'), Ok(3));
    // fail
    assert_eq!(to_uppercase('5'), Err(CharError::NotLetter));
    assert_eq!(to_lowercase('#'), Err(CharError::NotLetter));
    assert_eq!(print_char('\n'), Err(CharError::NotPrintable));
    assert_eq!(char_to_number('é'), Err(CharError::NotAscii));
    assert_eq!(char_to_number('a'), Err(CharError::NotDigit));
    assert_eq!(char_to_number_hex('ă'), Err(CharError::NotAscii));
    assert_eq!(char_to_number_hex('g'), Err(CharError::NotBase16Digit));

    print_error(&CharError::NotAscii);
    print_error(&CharError::NotDigit);
    print_error(&CharError::NotBase16Digit);
    print_error(&CharError::NotLetter);
    print_error(&CharError::NotPrintable);

    match char_to_number('x') {
        Ok(v) => println!("value = {v}"),
        Err(e) => print_error(&e),
    }
}

////////////////////////////////////////////
#[derive(Debug, PartialEq)]
enum CalcError {
    Overflow,
    DivisionByZero,
    UnknownOperator,
}

fn calculate(a: u32, op: char, b: u32) -> Result<u32, CalcError> {
    match op {
        '+' => match a.checked_add(b) {
            Some(r) => Ok(r),
            None => Err(CalcError::Overflow),
        },
        '*' => match a.checked_mul(b) {
            Some(r) => Ok(r),
            None => Err(CalcError::Overflow),
        },
        '/' => match a.checked_div(b) {
            Some(r) => Ok(r),
            None => Err(CalcError::DivisionByZero),
        },
        _ => Err(CalcError::UnknownOperator),
    }
}

fn show(a: u32, op: char, b: u32) {
    match calculate(a, op, b) {
        Ok(r) => println!("{a} {op} {b} = {r}"),
        Err(e) => println!("{a} {op} {b} -> error: {e:?}"),
    }
}

fn problem5() {
    println!("=============================================");
    // success
    assert_eq!(calculate(2, '+', 3), Ok(5));
    assert_eq!(calculate(6, '*', 7), Ok(42));
    assert_eq!(calculate(10, '/', 2), Ok(5));
    // fail
    assert_eq!(calculate(u32::MAX, '+', 1), Err(CalcError::Overflow));
    assert_eq!(calculate(65536, '*', 65536), Err(CalcError::Overflow));
    assert_eq!(calculate(5, '/', 0), Err(CalcError::DivisionByZero));
    assert_eq!(calculate(5, '-', 1), Err(CalcError::UnknownOperator));

    show(2, '+', 3);
    show(6, '*', 7);
    show(10, '/', 0);
    show(5, '%', 2);
    show(u32::MAX, '+', 1);
}

fn main() {
    problem1();
    problem2();
    problem3();
    problem4();
    problem5();
}
