use std::collections::HashMap;

pub fn lib() -> std::collections::HashMap<std::string::String, &'static str> {
  let mut translations: HashMap<String, &str> = HashMap::new();

  translations.insert("Yes".to_string(), "Oui");
  translations.insert("No".to_string(), "Non");

  translations
}
