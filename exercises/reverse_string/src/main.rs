use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter the string to reverse or press enter to end: ");
        io::stdout().flush().unwrap();
        
        let mut string = String::new();
        match io::stdin().read_line(&mut string) {
            Ok(_) => {
                let new_string = string.trim();
                string = new_string.to_string();
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
        if string.len() == 0 {
            break;
        }
        println!("The result is: {}", reverse(&string));
    }
}

pub fn reverse(input: &str) -> String {
    if input.len() == 0 {
        return input.to_string();
    }

    let mut reversed_string = String::new();
    let vector_chars: Vec<char> = input.chars().collect();
    let mut i = vector_chars.len() - 1;
    loop {
        let char = vector_chars[i];
        reversed_string.push(char);
        if i == 0 {
            break;
        }
        i -= 1;
    }
    return reversed_string;
}