use inflections::case;

pub enum Case {
  Camel,
  Pascal,
  Kebab,
  Train,
  Snake,
  Constant
}

impl Case {
  pub fn from_name(name: &str) -> Option<Case> {
    match name {
      "camel" => Some(Case::Camel),
      "pascal" => Some(Case::Pascal),
      "kebab" => Some(Case::Kebab),
      "train" => Some(Case::Train),
      "snake" => Some(Case::Snake),
      "constant" => Some(Case::Constant),
      _ => None
    }
  }

  pub fn to_case(&self, name: &str) -> String {
    match *self {
      Case::Camel => case::to_camel_case(name),
      Case::Pascal => case::to_pascal_case(name),
      Case::Kebab => case::to_kebab_case(name),
      Case::Train => case::to_train_case(name),
      Case::Snake => case::to_snake_case(name),
      Case::Constant => case::to_constant_case(name)
    }
  }
}
