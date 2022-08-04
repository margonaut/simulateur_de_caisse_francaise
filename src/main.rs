#[macro_use]
extern crate text_io;
use std::{
    collections::HashMap,
    io::{self, Write},
    path::Path,
};

use convo::Tree;

fn main() {
    // todo: Allow for different conversation selections (chatty vs. straight to business cashier)
    // todo: Allow for selection of hard vs. easy mode, where easy mode will ignore accent requirements
    // Select convo file to walk
    let path_in = Path::new("src/dialogue_files/english.yml");
    println!("Selected input path: {}", path_in.to_str().unwrap());

    // Import convo file
    print!("Importing...");
    let tree = convo::importer::import(path_in).unwrap();
    println!("Complete.");

    // Walk the Tree
    println!("Starting...\nYou may enter 'Q' to quit anytime.\n");
    walk(tree);
}

fn walk(mut tree: Tree) {
    // Walk the structure
    'walk: while let Some(current) = tree.current_node() {
        // Print node dialogue
        println!("{}", current.dialogue);

        // End if there's no links to choose
        if current.links.is_empty() {
            break 'walk; // Dead end
        }

        // Get user input
        print!(" > "); // User input prompt
        io::stdout().flush().unwrap(); // Flush before input capture
        let line: String = read!("{}\n"); // Capture

        // Quit if the user requests
        if line.trim().eq_ignore_ascii_case("q") {
            break 'walk;
        }
        let mut translations: HashMap<String, &str> = HashMap::new();

        translations.insert("Yes".to_string(), "Oui");
        translations.insert("No".to_string(), "Non");

        if line.trim().contains("uh") {
            println!("Oh, would you like a hint?");
            println!("Maybe you could try saying...");

            for (_id, link) in current.links.iter().enumerate() {
                match translations.get(&link.dialogue) {
                    Some(&translation) => println!("'{}'", translation),
                    // _ => println!("No translation of option found"),
                    _ => {}
                }
            }

            continue 'walk;
        }

        // Check to see if the user input matches any valid options, ignoring case
        let matching_link = current
            .links
            .iter()
            .find(|&x| x.dialogue.trim().eq_ignore_ascii_case(&line));

        if let Some(link) = matching_link {
            // a link was matched, get the next node identifier and move through the tree
            let link_key = link.to_key.clone();
            tree.set_current_key(&link_key).unwrap();
        } else {
            // todo: allow trying again instead of quitting
            println!("you suck at french!");
            break 'walk;
        }
    }
}

// Print node links
// for (id, link) in current.links.iter().enumerate() {
//     println!("[{}] {}", id, link.dialogue);
// }
// println!("The user input is: {line}");
// println!("The dialogue options are: {:?}", current.links);

// Replace accented characters: https://crates.io/crates/unidecode

// if line.trim().eq_ignore_ascii_case("") {
//     println!("Nothing? Really?");
//     continue 'walk;
// }
