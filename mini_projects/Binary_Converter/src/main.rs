use std::io;

const BASE: i32 = 2;

fn binary_addition(binary_1: &str, binary_2: &str) -> String {
    // Pad both to 8 bits so lengths match
    let bin_1 = to_bits(binary_1);
    let bin_2 = to_bits(binary_2);

    // Reverse both so index 0 = LSB (rightmost bit)
    let b1: Vec<char> = bin_1.chars().rev().collect();
    let b2: Vec<char> = bin_2.chars().rev().collect();

    let mut result = String::new();
    let mut carry: u8 = 0;

    // Walk both strings in parallel using one index
    let mut i = 0;
    while i < 8 {
        let x = b1[i].to_digit(10).unwrap() as u8;
        let y = b2[i].to_digit(10).unwrap() as u8;

        let (sum, new_carry) = bits_add(x, y, carry);
        carry = new_carry;

        result.push(char::from_digit(sum as u32, 10).unwrap());
        i += 1;
    }

    // If carry remains after all 8 bits, there is overflow
    if carry == 1 {
        result.push('1');
    }

    // Reverse to restore MSB-first order, then trim leading zeros
    let reversed: String = result.chars().rev().collect();
    let trimmed = reversed.trim_start_matches('0');

    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}

fn bits_add(x: u8, y: u8, carry: u8) -> (u8, u8) {
    let total = x + y + carry;
    let sum = total % 2;
    let new_carry = total / 2;
    (sum, new_carry)
}

fn to_bits(binary: &str) -> String {
    if binary.len() >= 8 {
        return binary.to_string();
    }

    let padding = "0".repeat(8 - binary.len());
    format!("{}{}", padding, binary)
}

fn flip_bits(bits: &str) -> String {
    let mut flipped = String::new();

    for ch in bits.chars() {
        if ch == '0' {
            flipped.push('1');
        } else if ch == '1' {
            flipped.push('0');
        } else {
            println!("Invalid bit: {}", ch);
        }
    }

    flipped
}

fn to_binary(n: i32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut bin_conv = String::new();
    let mut result = n / BASE;
    let mut rem = n % BASE;

    while result > 0 {
        bin_conv.push(if rem > 0 { '1' } else { '0' });
        rem = result % BASE;
        result /= BASE;
    }
    bin_conv.push('1');

    bin_conv.chars().rev().collect()
}

fn main() {
    println!("Enter a number: ");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: i32 = number
        .trim()
        .parse()
        .expect("Please enter a valid integer");

    let binary = to_binary(number);
    let bit_format = to_bits(&binary);
    let flipped = flip_bits(&bit_format);

    println!("Decimal        : {}", number);
    println!("Binary         : {}", binary);
    println!("8-bit format   : {}", bit_format);
    println!("Flipped bits   : {}", flipped);
    println!();
    println!("Addition test  : {} + {} = {}", "101", "11", binary_addition("101", "11"));
    println!("Expected       : 1000  (5 + 3 = 8)");
}