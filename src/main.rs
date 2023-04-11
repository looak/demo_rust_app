use std::{io::{self, Write}, fmt::format};

fn read_mask_from_user() -> u64 {
    loop {
        println!("Input chess bitboard mask: ");
        // io::stdout().flush();

        let mut input_str = String::new();
        match io::stdin().read_line(&mut input_str) {
            Ok(_) => (),
            Err(_) => println!("Invalid input, try again!"),
        }

        match u64::from_str_radix(input_str.trim(), 10) {
            Ok(mask) => return mask,
            Err(_) => println!("Invalid input, try again!"),
        }
    }
}

fn main() {
    let args:Vec<_> = std::env::args().collect();
    let mut mask:u64 = 0;
    if args.len() > 1 {
        mask = args[1].parse::<u64>().unwrap();
    }
    else {
        mask = read_mask_from_user();
    }
    
    // Print the number in decimal (base 10)
    println!("Decimal (base 10): {}", mask);

    // Print the number in binary (base 2)
    let binary = format!("{:b}", mask);
    println!("Binary (base 2): {}", binary);

    for i in 0..64 {
        let bit_str = if mask & (1 << i) != 0 { "x" } else { "." };
        print!("[ {} ]", bit_str);
        if (i % 8) == 7 {
            println!(""); // new line
        }
    }
       
}