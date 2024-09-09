mod interpreter;
mod utils;

use std::io::Write;
use term_size;


fn main() {
    println!("Welcome to the sosh!");
    let mut message = "Banner for information".to_string();
    loop {
        // get terminal width
        let (width, _) = term_size::dimensions().unwrap_or((80, 24));

        // write a prompt
        print!("sosh $ \n\x1b[1m\x1b[37m\x1b[100m {}{}\x1b[0m", message, " ".repeat(width as usize - message.len() - 1));

        // move the cursor up a line and 6 characters to the right
        print!("\x1b[1A\x1b[1000D\x1b[7C");
        std::io::stdout().flush().expect("TODO: panic message");

        // read a line from the user
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("TODO: panic message");

        // move the cursor down a line and to the beginning of the line
        print!("\x1b[1B\x1b[1000D");

        // clear the line
        print!("\x1b[0K");
        std::io::stdout().flush().expect("TODO: panic message");

        // parse the input
        let input = input.trim();

        // Tokenize the input
        let tokens: Vec<&str> = input.split_whitespace().collect();

        // dbg!(tokens.clone());

        // Check if the user wants to exit
        if tokens.len() == 1 && tokens[0] == "exit" {
            break;
        }

        // Parse PATH variable
        if tokens.len() != 0 {
            let path = std::env::var("PATH");
            if path.is_ok() {
                let path = path.unwrap();
                let paths: Vec<&str> = path.split(":").collect();

                // Check if the command is in the PATH
                let mut found = false;
                for p in paths {
                    let command = format!("{}/{}", p, tokens[0]);
                    if std::path::Path::new(&command).exists() {
                        found = true;
                        let output = std::process::Command::new(command)
                            .args(&tokens[1..])
                            .output()
                            .expect("failed to execute process");
                        println!("{}", String::from_utf8_lossy(&output.stdout));
                        break;
                    }
                }

                if !found {
                    println!("Error: command {} not found", tokens[0]);
                    continue;
                }

                continue;
            } else {
                println!("Error: PATH variable not set");
                continue;
            }
        }
    }
}
