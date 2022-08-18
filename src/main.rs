mod jue;
#[macro_use]
extern crate text_io;
mod conversation_generator;

fn main() {
    let (tree, translations) = conversation_generator::generate_conversation();
    jue::commencer(tree, translations);
}
