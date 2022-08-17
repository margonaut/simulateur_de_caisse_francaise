use convo::{Link, Node, Tree};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Response {
  anglais: String,
  francais: String,
}

#[derive(Serialize, Deserialize)]
struct Exchange {
  name: String,
  chance_of_happening: i32,
  prompts: Vec<String>,
  valid_responses: Vec<Response>,
}

#[derive(Serialize, Deserialize)]
struct Conversation {
  exchanges: Vec<Exchange>,
}

pub fn generate_conversation() -> convo::Tree {
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

  // Create list of conversation nodes
  for exchange in &exchange_data {
    let prompt_num: usize = rng.gen_range(0..exchange.prompts.len());

    // Factor an exchanges chance_of_happening into whether a node should be created
    let chance_to_happen_roll = rng.gen_range(0..100);
    if chance_to_happen_roll > exchange.chance_of_happening {
      continue;
    }

    let node = Node::new(&exchange.name, &exchange.prompts[prompt_num]);
    conversation_nodes.push(node)
  }

  let mut iterator = conversation_nodes.iter_mut().enumerate().peekable();
  // Iterate over nodes to create response links
  while let Some((i, node)) = iterator.next() {
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

  let root_key = "greeting";
  conversation_tree.set_root_key(root_key).unwrap();

  return conversation_tree;
}

fn get_typed_json_from_file(path: &str) -> Result<Conversation> {
  let data = fs::read_to_string(path).expect("Unable to read file");

  let c: Conversation = serde_json::from_str(&data).expect("JSON does not have correct format.");
  return Ok(c);
}
