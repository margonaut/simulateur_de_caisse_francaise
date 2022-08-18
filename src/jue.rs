use colored::Colorize;
use std::collections::HashMap;
use std::io::{self, Write};

use convo::Tree;

pub fn commencer(mut tree: Tree, translations: HashMap<String, String>) {
  let title_ascii = r##"
   _____ _                 _       _                   _         _____      _              
  / ____(_)               | |     | |                 | |       / ____|    (_)             
 | (___  _ _ __ ___  _   _| | __ _| |_ ___  _ __    __| | ___  | |     __ _ _ ___ ___  ___ 
  \___ \| | '_ ` _ \| | | | |/ _` | __/ _ \| '__|  / _` |/ _ \ | |    / _` | / __/ __|/ _ \
  ____) | | | | | | | |_| | | (_| | || (_) | |    | (_| |  __/ | |___| (_| | \__ \__ \  __/
 |_____/|_|_| |_| |_|\__,_|_|\__,_|\__\___/|_|     \__,_|\___|  \_____\__,_|_|___/___/\___|
 |  ____|                       (_)                                                        
 | |__ _ __ __ _ _ __   ___ __ _ _ ___  ___                                                
 |  __| '__/ _` | '_ \ / __/ _` | / __|/ _ \                                               
 | |  | | | (_| | | | | (_| (_| | \__ \  __/                                               
 |_|  |_|  \__,_|_| |_|\___\__,_|_|___/\___|                                               
                        )_)                                                                
"##;

  println!("{}", title_ascii);
  println!("");
  println!("Prepare yourself for...une interaction française! 🇫🇷");
  println!("");
  println!("Your responses are not case sensitive, but accents matter.");
  println!("");

  // Walk the structure
  'walk: while let Some(current) = tree.current_node() {
    // Print node dialogue
    println!("");
    println!(
      "{}{}{}",
      "\"".blue().bold(),
      current.dialogue.blue().bold(),
      "\"".blue().bold(),
    );
    println!("");

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

    // Print hints for valid dialogue options on user request
    if line.trim().contains("uh") {
      println!("");
      println!("Maybe you could try saying...");
      println!("");

      for (_id, link) in current.links.iter().enumerate() {
        match translations.get(&link.dialogue) {
          Some(translation) => println!("'{}'", translation),
          _ => println!("No translation of option found: {}", link.dialogue),
        }
      }
      println!("");
      println!("----------------");

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
      println!("");
      println!("{}", "\"Hmmm. Let's switch to English.\"".blue().bold());
      println!("");
      println!("{}", "JUE TERMINÉ, RÉESSAYEZ".red().bold());
      println!("");
      break 'walk;
    }
  }
}
