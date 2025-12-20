use serde::{Deserialize, Serialize};
#[allow(unused)]
#[derive(Debug, Clone, Serialize)]
struct BookData {
  title: String,
  authors: Vec<String>,
  thumbnail: String,
  description: String,
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
struct BookRoot {
  items: Vec<Item>,
}

#[allow(unused, non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct Item {
  volumeInfo: VolumeInfo,
}

#[allow(unused, non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct VolumeInfo {
  title: String,
  authors: Option<Vec<String>>,
  description: Option<String>,
  imageLinks: Option<ImageLinks>,
}

#[allow(unused, non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
struct ImageLinks {
  thumbnail: Option<String>,
  smallThumbnail: Option<String>,
}
