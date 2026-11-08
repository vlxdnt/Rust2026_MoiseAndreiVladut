use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::time::Instant;

// P1

fn p1(path: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(path)?;
    let mut longest_bytes = "";
    let mut longest_chars = "";

    for line in text.lines() {
        if line.len() > longest_bytes.len() {
            longest_bytes = line;
        }
        if line.chars().count() > longest_chars.chars().count() {
            longest_chars = line;
        }
    }

    println!(
        "Longest (bytes): {} ({} bytes)",
        longest_bytes,
        longest_bytes.len()
    );
    println!(
        "Longest (chars): {} ({} chars)",
        longest_chars,
        longest_chars.chars().count()
    );
    Ok(())
}

// P2

fn rot13(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_lowercase() {
            result.push(((c as u8 - b'a' + 13) % 26 + b'a') as char);
        } else if c.is_ascii_uppercase() {
            result.push(((c as u8 - b'A' + 13) % 26 + b'A') as char);
        } else {
            result.push(c);
        }
    }
    result
}

fn p2(path: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(path)?;
    if !text.is_ascii() {
        println!("Error: the text contains non-ASCII characters");
        return Ok(());
    }
    print!("{}", rot13(&text));
    Ok(())
}

// P3

fn expand(word: &str) -> &str {
    if word == "pt" || word == "ptr" {
        "pentru"
    } else if word == "dl" {
        "domnul"
    } else if word == "dna" {
        "doamna"
    } else {
        word
    }
}

fn p3(path: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        let mut result = String::new();
        for word in line.split(" ") {
            result.push_str(expand(word));
            result.push(' ');
        }
        println!("{}", result.trim_end());
    }
    Ok(())
}

// P4

fn p4(path: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(path)?;
    for line in text.lines() {
        if line.trim().starts_with('#') {
            continue;
        }
        let mut cols = line.split_whitespace();
        let Some(ip) = cols.next() else {
            continue;
        };
        let Some(host) = cols.next() else {
            continue;
        };
        println!("{} => {}", host, ip);
    }
    Ok(())
}

// Bonus
fn generate(path: &str, mib: u32) -> Result<(), io::Error> {
    let logo = r"########
  ########
    ########
      ########                      ########
        ########                  ########
          ########              ########
            ########          ########
              ########      ########
                ########  ########
                  ##############
                    ##########
                  VALORANT
";
    let block = logo.repeat(1000);
    let target = mib as u64 * 1024 * 1024;

    let mut file = File::create(path)?;
    let mut written: u64 = 0;
    while written < target {
        file.write_all(block.as_bytes())?;
        written += block.len() as u64;
    }
    Ok(())
}

// the whole file in memory, char by char
fn naive(input: &str, output: &str) -> Result<(), io::Error> {
    let text = fs::read_to_string(input)?;
    if !text.is_ascii() {
        println!("Error: the text contains non-ASCII characters");
        return Ok(());
    }
    fs::write(output, rot13(&text))
}

// only a 1 MiB buffer in memory, works directly on bytes
fn fast(input: &str, output: &str) -> Result<(), io::Error> {
    let mut reader = File::open(input)?;
    let mut writer = File::create(output)?;
    let mut buf = vec![0u8; 1024 * 1024];

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let chunk = &mut buf[..n];
        if !chunk.is_ascii() {
            println!("Error: the text contains non-ASCII characters");
            return Ok(());
        }
        for b in chunk.iter_mut() {
            if *b >= b'a' && *b <= b'z' {
                *b = (*b - b'a' + 13) % 26 + b'a';
            } else if *b >= b'A' && *b <= b'Z' {
                *b = (*b - b'A' + 13) % 26 + b'A';
            }
        }
        writer.write_all(chunk)?;
    }
    Ok(())
}

// generates the file then times both versions of ROT13 on it
fn bonus() -> Result<(), io::Error> {
    generate("big.txt", 4096)?;

    let start = Instant::now();
    naive("big.txt", "big_naive.txt")?;
    println!("naive: {:?}", start.elapsed());

    let start = Instant::now();
    fast("big.txt", "big_fast.txt")?;
    println!("fast: {:?}", start.elapsed());

    Ok(())
}

fn main() -> Result<(), io::Error> {
    p1("input.txt")?;
    p2("p2.txt")?;
    p3("p3.txt")?;
    p4("/etc/hosts")?;
    bonus()?;
    Ok(())
}
