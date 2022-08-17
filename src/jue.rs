use super::translations;
// #[macro_use]
// extern crate text_io;
use std::io::{self, Write};

use convo::Tree;

pub fn commencer(mut tree: Tree) {
  println!("START GAME?");
  println!("\n{:#?}", tree);
  let translations = translations::lib();

  // Walk the structure
  'walk: while let Some(current) = tree.current_node() {
    // Print node dialogue
    println!("\"{}\"", current.dialogue);

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
      println!("Maybe you could try saying...");

      for (_id, link) in current.links.iter().enumerate() {
        match translations.get(&link.dialogue) {
          Some(&translation) => println!("'{}'", translation),
          _ => println!("No translation of option found: {}", link.dialogue),
          // _ => {}
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
      println!("Oh, je vois. Let's switch to English.");
      println!("");
      println!("JUE TERMINÉ, RÉESSAYEZ");
      println!("");
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

// TODOS
// create dialougue tree
// create translations
// polish command line interactions
// FINI!
