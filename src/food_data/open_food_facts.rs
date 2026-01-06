use crate::app::bookdata::Data;
use log::info;
use openfoodfacts as off;
use serde::Deserialize;
use serde::Serialize;

pub fn get_open_food_facts_data(ean13: i64) -> Data {
  let mut result: Data;
  let client = off::v0().build().unwrap();
  let data = client
    .product(ean13.to_string().as_str(), None)
    .and_then(|resp| Ok(resp.json::<ProductRoot>().unwrap()));

  match data {
    Ok(product) => {
      if "product found" == product.status_verbose {
        println!("{:?}", product);
        result = Data {
          has_data: true,
          data: convert_to_string(product),
        };
      } else {
        println!("Codeeeeeeeeeeeeeeeeeeeeee: {}", product.status_verbose);
        result = Data {
          has_data: false,
          data: String::new(),
        };
      }
    }
    Err(_) => {
      result = Data {
        has_data: false,
        data: String::new(),
      }
    }
  };
  //println!("{}", result_json);

  result
}

fn convert_to_string(product: ProductRoot) -> String {
  let product = product.product;
  format!(
    "Brand: {}\n Countrie: {}\n Ingredients: {}",
    product.brands, product.countries, product.ingredients_text
  )
}

#[allow(unused)]
#[derive(Debug, Deserialize, Serialize)]
pub struct ProductRoot {
  code: String,
  product: item,
  status_verbose: String,
}

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize)]
struct item {
  brands: String,
  countries: String,
  ingredients_text: String,
  nutrient_levels: nutrient_levels,
  nutriments: nutriments,
  nutriscore_grade: String,
  pnns_groups_1: String,
  pnns_groups_2: String,
  product_name: String,
  product_type: String,
  quantity: String,
}

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize)]
struct nutrient_levels {
  fat: String,
  salt: String,
  #[serde(rename = "saturated-fat")]
  saturated_fat: String,
  sugars: String,
}

#[allow(unused, non_camel_case_types)]
#[derive(Debug, Deserialize, Serialize)]
struct nutriments {
  energy_100g: i32,
  energy_unit: String,
  sugars_unit: String,
  sugars_value: i32,
}
