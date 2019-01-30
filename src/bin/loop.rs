use std::io::{self, Write}; // `Write` trait for flush

fn main(){
    loop {
        let mut s: String = String::new();
        print!("Please input your something(type 'q' to quit): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut s).expect("Failed to read line");
        let s = s.trim();
        println!("read: {}", s);
        if s == "q" {
            println!("bye");
            break;
        } else {
            println!("continue");
        }
    }
}
