mod jue;
#[macro_use]
extern crate text_io;
mod conversation_generator;
mod translations;

fn main() {
    let tree = conversation_generator::generate_conversation();
    // todo: Allow for different conversation selections (chatty vs. straight to business cashier)
    // todo: Allow for selection of hard vs. easy mode, where easy mode will ignore accent requirements
    jue::commencer(tree);
}
