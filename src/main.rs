use cli_prompts::{prompts::{Confirmation, Input, Multiselect}, DisplayPrompt};

fn main() {

    loop {
        let input_prompt = Input::new("Omni_shell >", |s| Ok(s.to_string())).help_message("enter command");

        let command = input_prompt.display().unwrap();

        if command == "exit"{
            break;
        } else {
            println!("Omni_shell > {}", command)
        }
        
    }
    
}
