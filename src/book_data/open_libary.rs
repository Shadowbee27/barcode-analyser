/*use crate::app::bookdata::Data;
use reqwest::Client;

pub async fn get_open_libary_data(ean13: i32) -> Data {
  let url = format!(
    "https://openlibrary.org/api/books?bibkeys={}&format=json",
    ean13
  );
  let cleint = Client::new();
  let result = cleint.get(url).send().await.unwrap();
  Data::default()
}
*/
