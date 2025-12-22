use serde::Serialize;

#[allow(unused)]
#[derive(Debug, Clone, Serialize)]
pub struct BookData<'a> {
  pub title: &'a String,
  pub authors: Vec<String>,
  pub description: String,
}
#[derive(Debug, Clone)]
pub struct Data {
  pub has_data: bool,
  pub data: String,
}

impl Default for Data {
  fn default() -> Self {
    Data {
      has_data: false,
      data: String::new(),
    }
  }
}

impl<'a> std::fmt::Display for BookData<'a> {
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
