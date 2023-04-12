use std::io;

use pronouner::*;

const CAST: &'static str = include_str!("assets/characters.json");
const DICT: &'static str = include_str!("assets/dictionary.json");
const CONVERSATION: &'static str = include_str!("assets/conversation.xyr");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's load the already known cast from the characters file and the verb dictionary from dictionary.json.
    let mut cast: CharacterCast = serde_json::from_str(CAST)?;
    let dict = serde_json::from_str(DICT)?;

    // Ask the user their name and pronouns.
    let (name, pronouns) = ask_name_and_pronouns()?;

    // Add the user as a new character with the identifier "player".
    let player = GrammaticalCharacter::new(name, pronouns, None, None);
    cast.insert("player".to_string(), player);

    // Instantiate a new compiler with the cast and dictionary as context.
    let compiler = DialogMacroCompiler::new(cast, dict);

    // Compile the conversation into the final message.
    let message = compiler.parse_and_compile(CONVERSATION)?;

    // Print out the message.
    println!("\n\n\n=========================================\n{message}\n=========================================");

    Ok(())
}

fn ask_name_and_pronouns() -> Result<(String, Pronouns), Box<dyn std::error::Error>> {
    // Ask the user their name and pronouns
    let mut lines = io::stdin().lines();

    println!("Well hello friend! What's your name?");

    let name = {
        let Some(name_res) = lines.next() else {
            println!("Something went wrong :(");
            return Err(Box::new(std::io::Error::from(std::io::ErrorKind::Other)));
        };

        name_res?
    };

    println!("{}? That's a nice name!", &name);
    println!("One more thing: What are your pronouns?");
    println!("\the/him -> 1\n\tshe/her -> 2\n\tthey/them -> 3\n\txe/xyr -> 4");

    let pronouns = loop {
        let Some(option_res) = lines.next() else {
            println!("Something went wrong :(");
            return Err(Box::new(std::io::Error::from(std::io::ErrorKind::Other)));
        };

        let option = option_res?;

        let Ok(value) = option.parse::<i32>() else {
            println!("That's not a number. Please answer with 1, 2, 3, or 4.");
            continue;
        };

        if value < 1 || value > 4 {
            println!("Please answer with 1, 2, 3, or 4.");
            continue;
        }

        break match value {
            1 => Pronouns::HeHim,
            2 => Pronouns::SheHer,
            3 => Pronouns::TheyThem,
            4 => Pronouns::XeXyr,
            _ => panic!(),
        };
    };

    Ok((name, pronouns))
}
