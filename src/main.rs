use std::io;
use std::iter::repeat;
use std::io::prelude::*;
use std::str::FromStr;

pub mod rustyham;
use rustyham::rustyham::*;

fn main() {
    let title = "A HAMMING CODE GENERATER IN RUST";
    let border: String = repeat('=').take(title.len()).collect::<>();
    println!("{}", border);
    println!("{}", title);
    println!("{}", border);
    
    loop {
        let mut linebuf = String::new();
        println!("(1) Encode ASCII");
        println!("(2) Decode to ASCII");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut linebuf).unwrap();

        let num = i32::from_str(linebuf.trim());
        match num.ok() {
            Some(n) => {
                let mut good = true;
                print!("{}",match n {
                    1 => hamming(Hamming::Encode, prompt("Enter string to encode: ")),
                    2 => hamming(Hamming::Decode, prompt("Enter Hamming code to decode: ")),
                    _ => { good = false; "Invalid input.\n".to_string() }
                });
                if good {break;}
            }
            None => { println!("Invalid input.")}
        }

    }
}

fn prompt(s: &str) -> String {
    let mut input = String::new();
    print!("{}", s);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
