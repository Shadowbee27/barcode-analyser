use crate::app::bookdata::{BookData, GBookData};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
struct BookRoot {
  items: Vec<Item>,
}

#[allow(unused, non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Item {
  volumeInfo: VolumeInfo,
}

#[allow(unused, non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct VolumeInfo {
  title: String,
  authors: Option<Vec<String>>,
  description: Option<String>,
  imageLinks: Option<ImageLinks>,
}

#[allow(unused, non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
struct ImageLinks {
  thumbnail: Option<String>,
  smallThumbnail: Option<String>,
}

pub fn get_google_book_data(ean13: i64) -> GBookData {
  let url = format!(
    "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
    ean13,
  );
  let client = Client::new();
  let result = client
    .get(&url)
    .send()
    .and_then(|resp| resp.json::<BookRoot>());

  match result {
    Ok(bookroot) => {
      if let Some(item) = bookroot.items.first() {
        GBookData {
          has_data: true,
          data: convert_to_string(item),
        }
      } else {
        GBookData {
          has_data: false,
          data: String::new(),
        }
      }
    }
    Err(_) => GBookData {
      has_data: false,
      data: String::new(),
    },
  }
}

fn convert_to_string(data: &Item) -> String {
  let data = data.clone();
  let bookdata = data.to_bookdata();

  format!("{}", bookdata)
}

impl Item {
  pub fn to_bookdata(self) -> BookData {
    let mut authors: Vec<String> = Vec::new();
    let mut description = String::new();
    if let Some(se) = self.volumeInfo.authors {
      authors = se;
    }
    if let Some(se) = self.volumeInfo.description {
      description = se;
    }
    BookData {
      title: self.volumeInfo.title,
      authors,
      description,
    }
  }
}
