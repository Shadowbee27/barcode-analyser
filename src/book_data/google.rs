use crate::app::bookdata::{BookData, Data};
use reqwest::Client;
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

pub async fn get_google_book_data(ean13: i32) -> Data {
  let url = format!(
    "https://www.googleapis.com/books/v1/volumes?q=isbn:{}",
    ean13,
  );
  let cleint = Client::new();
  let bookdata = cleint
    .get(url)
    .send()
    .await
    .unwrap()
    .json::<BookRoot>()
    .await
    .unwrap();

  let data = Data {
    has_data: true,
    data: convert_to_string(&bookdata.items.first().unwrap()).await,
  };
  return data;
}

async fn convert_to_string(data: &Item) -> String {
  let mut data = data.clone();
  let bookdata = data.to_bookdata();

  format!("{}", bookdata)
}

impl Item {
  pub fn to_bookdata(&mut self) -> BookData<'_> {
    let mut authors: Vec<String> = Vec::new();
    let mut description = String::new();
    if let Some(se) = self.volumeInfo.authors.take() {
      authors = se;
    }
    if let Some(se) = self.volumeInfo.description.take() {
      description = se;
    }
    BookData {
      title: &self.volumeInfo.title,
      authors,
      description,
    }
  }
}
