use crate::app::bookdata::Data;
use log::*;
use openfoodfacts as off;
use serde_json::{Value, json};
use std::collections::HashMap;

pub fn get_open_food_facts_data(ean13: i64) -> Data {
  let mut result = Data::default();
  let client = off::v2().build().unwrap();
  let data = client.product(ean13.to_string().as_str(), None).unwrap();
  let result_json = json!(data.json::<HashMap::<String, Value>>().unwrap());

  if let Some(products) = result_json["products"].as_array() {
    debug!("That many products {}", products.len());
    for product in products {
      result = Data {
        has_data: true,
        data: format!(
          "name: {}\n allergens: {}.",
          product["product_name"], product["allergens"]
        ),
      };
    }
  }
  result
}
