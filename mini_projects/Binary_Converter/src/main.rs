use std::io;
const BASE: i32 = 2;

fn binary_additon(binary_1: &str, binary_2: &str) -> String {
    let mut result = String::new();
    
    if binary_1.len() == 1 && binary_2.len() == 1 {
        let x = binary_1.chars().next().unwrap().to_digit(2).unwrap() as u8;
        let y = binary_2.chars().next().unwrap().to_digit(2).unwrap() as u8;
    
        let (sum, carry) = bits_add(x, y, 0);

        result.push(char::from_digit(carry as u32, 10).unwrap());
    
        result.push(char::from_digit(sum as u32, 10).unwrap());
        
    } else {
        // Assumption : binary nos are  8 bits or less
        let mut bin_1 = to_bits(&binary_1);
        let mut bin_2 = to_bits(&binary_2);
        bin_1 = bin_1.chars().rev().collect();
        bin_2 = bin_2.chars().rev().collect();
  
        let mut len =

        for i in bin_1.chars() {
            let j = i;
            let mut carry_val = 0;
            for j in bin_2.chars() {
                println!("I: {i}  J: {j}");
                let (sum, carry)  = bits_add(i.to_digit(10).unwrap() as u8, 
                                        j.to_digit(10).unwrap() as u8, carry_val);
                result.push_str(&sum.to_string());
                carry_val = carry;
                break;
            };
        };

    };

    if result.len() == 9 {
        result.pop();
    };
    result.chars().rev().collect()
}

fn bits_add(x: u8, y: u8, z: u8) -> (u8, u8) {
    let total = x + y + z;
    let sum = total % 2;
    let carry = total / 2;
    (sum, carry)
}

// fn twos_complemet(flipped_bits: &str) -> String {

// }

fn to_bits(binary: &str) -> String {
    if binary.len() >= 9 {
        return binary.to_string()
    }

    let mut bits = String::new();
    let mut deficit = 9 - binary.len();
    while deficit > 1 {
        bits.push('0');
        deficit -= 1;
    };
    bits.push_str(&binary);
    bits
}

fn flip_bits(bits: &str) -> String {
    let mut flipped = String::new();

    for ch in bits.chars() {
        if ch == '0' {
            flipped.push('1');
        } else if ch == '1' {
            flipped.push('0');
        } else{
            println!("Invalid Bit number");
        };
    };

    flipped
}


fn to_binary(n: i32) -> String {
    if n == 0 {
        return "0".to_string()
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

    // Input Validation
    let number: i32 = number.trim().parse()
        .expect("Please enter a valid integer");

    // let binary = to_binary(number);
    // let bit_format = to_bits(&binary);
    // let flipped = flip_bits(&bit_format);

    // let bits_additon = bits_add(1, 0);


    // println!("{} carry {}", bits_additon.0, bits_additon.1);
    // println!("Converted {} number to binary is: {}", number, binary);
    // println!("To 8 bit format: {}", bit_format);
    // println!("flipped bit: {}", flipped);

    
    println!("Binary addition: {} + {} = {}", "11", "1", binary_additon("11", "1"));



}
