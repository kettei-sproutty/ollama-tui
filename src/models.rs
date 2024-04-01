#[derive(Debug)]
pub struct Model {
  name: String,
  version: String,
}

impl Model {
  pub fn pull_model(&self) -> &Self {
    &self
  }
}

pub fn fetch_models() -> Vec<Model> {
  vec![]
}
