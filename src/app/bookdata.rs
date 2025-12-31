use serde::Serialize;

#[allow(unused)]
#[derive(Debug, Clone, Serialize)]
pub struct BookData {
  pub title: String,
  pub authors: Vec<String>,
  pub description: String,
}
#[derive(Debug, Clone, Default)]
pub struct Data {
  pub has_data: bool,
  pub data: String,
}

impl std::fmt::Display for BookData {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Title: {}\nAuthors: {}\nDescription: {}",
      self.title,
      if self.authors.is_empty() {
        "Unknown".to_string()
      } else {
        self.authors.join(", ")
      },
      if self.description.is_empty() {
        "No description available".to_string()
      } else {
        self.description.clone()
      },
    )
  }
}
