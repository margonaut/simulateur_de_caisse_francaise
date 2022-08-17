use convo::{Link, Node, Tree};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::fs;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
struct Response {
  anglais: String,
  francais: String,
}

#[derive(Serialize, Deserialize)]
struct Exchange {
  name: String,
  chance_of_happening: i32,
  // chance_of_happening: u8,
  prompts: Vec<String>,
  valid_responses: Vec<Response>,
}

#[derive(Serialize, Deserialize)]
struct Conversation {
  exchanges: Vec<Exchange>,
}

pub fn generate_conversation() -> convo::Tree {
  let mut tree = Tree::new();
  let root_key = "greeting";

  // Create some nodes
  let mut node1 = Node::new(root_key, "I am the root node!");
  let node2 = Node::new("end", "I am the last node!");

  // Link node1 -> node 2
  Link::link(&mut node1, &node2, "I link start to end!");

  // Populate the tree
  tree.nodes.insert(node1.key.clone(), node1);
  tree.nodes.insert(node2.key.clone(), node2);

  /* BEGIN ACTUAL IMPLEMENTATION */
  let mut conversation_tree = Tree::new();
  let mut conversation_nodes: Vec<Node> = Vec::new();
  let mut rng = rand::thread_rng();
  let mut exchange_data: Vec<Exchange> = Vec::new();

  // Fetch basic conversation data
  match get_typed_json_from_file("src/convo.json") {
    Ok(conversation) => {
      exchange_data = conversation.exchanges;
    }
    Err(err) => println!("Error: {}", err),
  }

  // create list of conversation nodes
  for exchange in &exchange_data {
    let prompt_num: usize = rng.gen_range(0..exchange.prompts.len());

    // Factor an exchanges chance_of_happening into whether a node should be created
    let chance_to_happen_roll = rng.gen_range(0..100);
    if chance_to_happen_roll > exchange.chance_of_happening {
      continue;
    }

    let node = Node::new(&exchange.name, &exchange.prompts[prompt_num]);
    println!("Node created: {}", node.key);
    conversation_nodes.push(node)
  }

  // TODO: Fix this

  let mut iterator = conversation_nodes.iter_mut().enumerate().peekable();
  // iterate over nodes to create response links

  while let Some((i, node)) = iterator.next() {
    // for (i, node) in iterator {
    // let next_node = &mut conversation_nodes[i + 1];
    let maybe_next_node = iterator.peek();

    match maybe_next_node {
      Some((_, next_node)) => {
        let exchange = &exchange_data[i];
        let responses = &exchange.valid_responses;

        for response in responses {
          Link::link(node, &next_node, &response.francais);
        }
      }
      None => continue,
    }
  }

  // add all nodes to tree
  for node in conversation_nodes {
    conversation_tree.nodes.insert(node.key.clone(), node);
  }

  // conversation_tree.set_root_key("greeting").unwrap();
  let root_key = "greeting";
  conversation_tree.set_root_key(root_key).unwrap();

  // println!("\n{:#?}", conversation_tree);
  return conversation_tree;
  // return tree;
}

// fn get_typed_json_from_file() -> Result<conversation_generator::Conversation> {
fn get_typed_json_from_file(path: &str) -> Result<Conversation> {
  // fn get_json_from_file() -> serde_json::Value {
  let data = fs::read_to_string(path).expect("Unable to read file");

  let c: Conversation = serde_json::from_str(&data).expect("JSON does not have correct format.");
  return Ok(c);

  // Can be used like:
  // match get_json_from_file() {
  //   Ok(data) => {
  //     println!("{}", data);
  //     println!("{}", data["structure"][0]);
  //   }
  //   Err(err) => println!("Error: {}", err),
  // }
}

// fn get_json_from_file() -> Result<serde_json::Value> {
//   // fn get_json_from_file() -> serde_json::Value {
//   let data = fs::read_to_string("src/convo.json").expect("Unable to read file");

//   let json: serde_json::Value =
//     serde_json::from_str(&data).expect("JSON does not have correct format.");

//   return Ok(json);

//   // Can be used like:
//   // match get_json_from_file() {
//   //   Ok(data) => {
//   //     println!("{}", data);
//   //     println!("{}", data["structure"][0]);
//   //   }
//   //   Err(err) => println!("Error: {}", err),
//   // }
// }

// fn get_json_values() -> Result<serde_json::Value> {
//   // Some JSON input data as a &str. Maybe this comes from the user.
//   let data = r#"
//         {
//             "name": "John Doe",
//             "age": 43,
//             "phones": [
//                 "+44 1234567",
//                 "+44 2345678"
//             ]
//         }"#;

//   // Parse the string of data into serde_json::Value.
//   let v: Value = serde_json::from_str(data)?;

//   // Access parts of the data by indexing with square brackets.
//   // println!("Please call {} at the number {}", v["name"], v["phones"][0]);

//   Ok(v)
// }

// having a lot of trouble creating and referencing dynamic variables
// fucks up later usage of tree somehow
fn get_node_by_name(tree: &'static convo::Tree, name: &str) -> &'static convo::Node {
  let index_num = tree.nodes.get_index_of(name).unwrap();
  return &tree.nodes[index_num];
}

// fn calculate_length(s: &String) -> usize {
//   s.len()
// }

// fn create_node_and_add_translation_to_list() {}

// {
//     key: "greeting",
//     dialogue: "I am the root node!",
//     links: [
//         Link {
//             to_key: "end",
//             dialogue: "I link start to end!",
//         },
//     ],
// }

// you can't write

// struct ConvoNode {
//     key: String,
//     prompts:  Vec<String>,
//     valid_responses: Vec<Struct {
//       anglais: String,
//       francais: String,
//     }>
//     // exchanges: String,
// }

// struct ConversationMap {
//     exchanges: String,
//     // exchanges: String,
//     age: u8,
//     phones: Vec<String>,
// }

// #[derive(Serialize, Deserialize)]
// struct Person {
//   name: String,
//   age: u8,
//   phones: Vec<String>,
// }

// fn typed_example() -> Result<()> {
//   // Some JSON input data as a &str. Maybe this comes from the user.
//   let data = r#"
//         {
//             "name": "John Doe",
//             "age": 43,
//             "phones": [
//                 "+44 1234567",
//                 "+44 2345678"
//             ]
//         }"#;

//   // Parse the string of data into a Person object. This is exactly the
//   // same function as the one that produced serde_json::Value above, but
//   // now we are asking it for a Person as output.
//   let p: Person = serde_json::from_str(data)?;

//   // Do things just like with any other Rust data structure.
//   println!("Please call {} at the number {}", p.name, p.phones[0]);

//   Ok(())
// }

// let mut curveball_data: Vec<Exchange> = Vec::new();

// // TODO: Create random curveball nodes and insert into node list
// let mut curveball_nodes: Vec<Node> = Vec::new();
// for curveball in &curveball_data {
//   let prompt_num: usize = rng.gen_range(0..curveball.prompts.len());

//   let node = Node::new(&curveball.name, &curveball.prompts[prompt_num]);
//   println!("Node created: {}", node.key);
//   curveball_nodes.push(node)
// }

// Fetch potential curveball exchange data
// match get_typed_json_from_file("src/curveballs.json") {
//   Ok(curveballs) => {
//     curveball_data = curveballs.exchanges;
//   }
//   Err(err) => println!("Error: {}", err),
// }
