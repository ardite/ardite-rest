// TODO: consider moving this to Ardite?

use inflector;

pub enum Case {
  Camel,
  Kebab,
  Snake
}

impl Case {
  pub fn from_str(string: &str) -> Option<Self> {
    match string {
      "camel" => Some(Case::Camel),
      "kebab" => Some(Case::Kebab),
      "snake" => Some(Case::Snake),
      _ => None
    }
  }

  pub fn to_case(&self, string: String) -> String {
    match *self {
      Case::Camel => inflector::cases::camelcase::to_camel_case(string),
      Case::Kebab => inflector::cases::kebabcase::to_kebab_case(string),
      Case::Snake => inflector::cases::snakecase::to_snake_case(string)
    }
  }

  pub fn to_url_key(&self, mut string: String) -> String {
    string.push_str("_url");
    self.to_case(string)
  }
}
