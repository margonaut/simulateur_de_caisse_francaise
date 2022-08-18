// use super::translations;
use std::collections::HashMap;
// #[macro_use]
// extern crate text_io;
use std::io::{self, Write};

use convo::Tree;

pub fn commencer(mut tree: Tree, translations: HashMap<String, String>) {
  println!("");
  println!("");
  println!("Prepare yourself for...une interaction française!");
  println!("");

  // Walk the structure
  'walk: while let Some(current) = tree.current_node() {
    // Print node dialogue
    println!("");
    println!("\"{}\"", current.dialogue);
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
          // _ => {}
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
      println!("\"Hmmm. Let's switch to English.\"");
      println!("");
      println!("JUE TERMINÉ, RÉESSAYEZ");
      println!("");
      break 'walk;
    }
  }
}
