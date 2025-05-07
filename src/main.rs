use cli_prompts::{prompts::{Confirmation, Input, Multiselect}, DisplayPrompt};
use chrono::{DateTime, Utc};

fn main() {


    loop {
        
        let input_prompt = Input::new("Omni_shell >", |s| Ok(s.to_string())).help_message("enter command");

        let command = input_prompt.display().unwrap();
        let command = command.trim();

        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.get(0) {
            Some(&"exit") | Some(&"stop") => break,
            Some(&"echo") | Some(&"e") => echo_command(&parts),
            Some(&"clear") | Some(&"c") => println!("\x1B[2J\x1B[H"),
            Some(&"date") | Some(&"time") => {
                let now: DateTime<Utc> = Utc::now();
                println!("UTC now is: {}", now);
            }
            Some(&"help") | Some(&"h") => println!(
                "Available commands:\n\
                 - help: Show this message\n\
                 - clear: Clear the screen\n\
                 - date: Show current date and time\n\
                 - exit: Exit the shell"
            ),
            _ => println!("command unknown"),
        }
        
    }
    }
    


fn echo_command(parts: &[&str]) {
    if parts.len() > 1 {
        let message = parts[1..].join(" ");
        println!("{}", message);
    } else {
        println!("Usage: echo <message>");
    }
}
    