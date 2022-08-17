mod jue;
#[macro_use]
extern crate text_io;
use std::path::Path;
mod conversation_generator;
mod translations;

fn main() {
    let tree = conversation_generator::generate_conversation();
    // todo: Allow for different conversation selections (chatty vs. straight to business cashier)
    // todo: Allow for selection of hard vs. easy mode, where easy mode will ignore accent requirements
    // // Select convo file to walk
    // let path_in = Path::new("src/dialogue_files/english.yml");
    // println!("Selected input path: {}", path_in.to_str().unwrap());

    // // Import convo file
    // print!("Importing...");
    // let tree = convo::importer::import(path_in).unwrap();
    // println!("Complete.");

    // // Walk the Tree
    // println!("Starting...\nYou may enter 'Q' to quit anytime.\n");
    jue::commencer(tree);
}
