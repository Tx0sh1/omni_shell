use cli_prompts::{prompts::{Confirmation, Input, Multiselect}, DisplayPrompt};
use chrono::{DateTime, Utc};

fn main() {

    let now: DateTime<Utc> = Utc::now();

    loop {
        let input_prompt = Input::new("Omni_shell >", |s| Ok(s.to_string())).help_message("enter command");

        let command = input_prompt.display().unwrap();
        let command = command.trim();

        match command {
            "exit" => break,
            "date" => println!("UTC now is: {}", now),
            "help" => println!("Available commands: \n - help: Show this message \n - clear: Clear the screen \n - date: Show current date and time \n - exit: Exit the shell"),
            _ => println!("{}", command)
        }
        
    }
    
}
